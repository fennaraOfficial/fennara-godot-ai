use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use serde_json::{Value, json};
use sha2::Sha256;
use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::time::Duration;

const DAEMON_ADDR: &str = "127.0.0.1:41287";
const CONTROL_HEADER: &str = "X-Fennara-Control-Token";
const CONTROL_TOKEN_FILE: &str = "daemon-control-token";
const MAX_DAEMON_RESPONSE_BYTES: usize = 32 * 1024 * 1024;
const MAX_CHALLENGE_RESPONSE_BYTES: usize = 4096;
type HmacSha256 = Hmac<Sha256>;

pub(crate) fn daemon_status() -> Result<Value, String> {
    daemon_get("/status")
}

pub(crate) fn daemon_tool_call(tool: &str, args: Value) -> Result<Value, String> {
    let body = json!({
        "tool": tool,
        "args": args
    })
    .to_string();
    daemon_post("/tools/call", &body)
}

fn daemon_get(path: &str) -> Result<Value, String> {
    let control_token = daemon_control_token()?;
    verify_daemon(&control_token)?;
    let mut stream = TcpStream::connect(DAEMON_ADDR)
        .map_err(|error| format!("Open a Godot project with Fennara enabled. ({error})"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .map_err(|error| error.to_string())?;
    stream
        .set_write_timeout(Some(Duration::from_secs(2)))
        .map_err(|error| error.to_string())?;

    let request = format!(
        "GET {path} HTTP/1.1\r\nHost: 127.0.0.1\r\n{CONTROL_HEADER}: {control_token}\r\nConnection: close\r\n\r\n"
    );
    stream
        .write_all(request.as_bytes())
        .map_err(|error| error.to_string())?;

    read_http_json_response(stream)
}

fn daemon_post(path: &str, body: &str) -> Result<Value, String> {
    let control_token = daemon_control_token()?;
    verify_daemon(&control_token)?;
    let mut stream = TcpStream::connect(DAEMON_ADDR)
        .map_err(|error| format!("Open a Godot project with Fennara enabled. ({error})"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(300)))
        .map_err(|error| error.to_string())?;
    stream
        .set_write_timeout(Some(Duration::from_secs(2)))
        .map_err(|error| error.to_string())?;

    let request = format!(
        "POST {path} HTTP/1.1\r\nHost: 127.0.0.1\r\n{CONTROL_HEADER}: {control_token}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    stream
        .write_all(request.as_bytes())
        .map_err(|error| error.to_string())?;

    read_http_json_response(stream)
}

fn read_http_json_response(stream: TcpStream) -> Result<Value, String> {
    read_http_json_response_limited(stream, MAX_DAEMON_RESPONSE_BYTES)
}

fn read_http_json_response_limited(stream: TcpStream, max_bytes: usize) -> Result<Value, String> {
    let mut response = String::new();
    stream
        .take(max_bytes as u64 + 1)
        .read_to_string(&mut response)
        .map_err(|error| error.to_string())?;
    if response.len() > max_bytes {
        return Err("daemon HTTP response exceeded the local size limit".to_string());
    }

    let (headers, body) = response
        .split_once("\r\n\r\n")
        .ok_or_else(|| "invalid daemon HTTP response".to_string())?;
    if !headers.starts_with("HTTP/1.1 200") && !headers.starts_with("HTTP/1.0 200") {
        return Err("daemon returned non-200 status".to_string());
    }
    serde_json::from_str(body).map_err(|error| error.to_string())
}

fn daemon_control_token() -> Result<String, String> {
    let path = fennara_app_dir()?.join(CONTROL_TOKEN_FILE);
    daemon_control_token_at(&path)
}

fn daemon_control_token_at(path: &Path) -> Result<String, String> {
    let raw = fs::read_to_string(&path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    let token = raw.trim();
    let valid = URL_SAFE_NO_PAD
        .decode(token)
        .is_ok_and(|bytes| bytes.len() == 32);
    if !valid {
        return Err(format!(
            "{} does not contain a valid daemon control token",
            path.display()
        ));
    }
    Ok(token.to_string())
}

fn verify_daemon(control_token: &str) -> Result<(), String> {
    verify_daemon_at(control_token, DAEMON_ADDR)
}

fn verify_daemon_at(control_token: &str, address: &str) -> Result<(), String> {
    let mut nonce = [0_u8; 32];
    getrandom::fill(&mut nonce)
        .map_err(|error| format!("failed to create daemon challenge: {error}"))?;
    let encoded_nonce = URL_SAFE_NO_PAD.encode(nonce);

    let mut stream = TcpStream::connect(address)
        .map_err(|error| format!("Open a Godot project with Fennara enabled. ({error})"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .map_err(|error| error.to_string())?;
    stream
        .set_write_timeout(Some(Duration::from_secs(2)))
        .map_err(|error| error.to_string())?;
    let request = format!(
        "GET /control/challenge?nonce={encoded_nonce} HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n"
    );
    stream
        .write_all(request.as_bytes())
        .map_err(|error| error.to_string())?;
    let response = read_http_json_response_limited(stream, MAX_CHALLENGE_RESPONSE_BYTES)?;
    let proof = response
        .get("proof")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            "The process on the Fennara daemon port could not prove its identity.".to_string()
        })?;
    let proof = URL_SAFE_NO_PAD.decode(proof).map_err(|_| {
        "The process on the Fennara daemon port returned an invalid proof.".to_string()
    })?;
    let token_bytes = URL_SAFE_NO_PAD
        .decode(control_token)
        .map_err(|_| "The local daemon control token is invalid.".to_string())?;
    let mut mac = HmacSha256::new_from_slice(&token_bytes)
        .map_err(|_| "The local daemon control token is invalid.".to_string())?;
    mac.update(&nonce);
    mac.verify_slice(&proof).map_err(|_| {
        "The process on the Fennara daemon port failed identity verification.".to_string()
    })
}

fn fennara_app_dir() -> Result<PathBuf, String> {
    #[cfg(target_os = "windows")]
    {
        env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .map(|path| path.join("Fennara"))
            .ok_or_else(|| "LOCALAPPDATA is not set".to_string())
    }

    #[cfg(target_os = "macos")]
    {
        env::var_os("HOME")
            .map(PathBuf::from)
            .map(|path| {
                path.join("Library")
                    .join("Application Support")
                    .join("Fennara")
            })
            .ok_or_else(|| "HOME is not set".to_string())
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        if let Some(path) = env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .filter(|path| path.is_absolute())
        {
            return Ok(path.join("fennara"));
        }
        env::var_os("HOME")
            .map(PathBuf::from)
            .map(|path| path.join(".local").join("share").join("fennara"))
            .ok_or_else(|| "HOME and XDG_DATA_HOME are not set".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;
    use std::thread;

    fn test_token_path() -> PathBuf {
        std::env::temp_dir().join(format!("fennara-mcp-control-token-{}", std::process::id()))
    }

    fn mock_challenge_server(proof: &str) -> (String, thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap().to_string();
        let proof = proof.to_string();
        let handle = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0_u8; 2048];
            let _ = stream.read(&mut request).unwrap();
            let body = json!({ "ok": true, "proof": proof }).to_string();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            stream.write_all(response.as_bytes()).unwrap();
        });
        (address, handle)
    }

    #[test]
    fn daemon_control_token_validates_file_contents() {
        let path = test_token_path();
        let _ = fs::remove_file(&path);

        let missing = daemon_control_token_at(&path).unwrap_err();
        assert!(missing.contains("failed to read"));

        fs::write(&path, "not-a-token\n").unwrap();
        let malformed = daemon_control_token_at(&path).unwrap_err();
        assert!(malformed.contains("does not contain a valid daemon control token"));

        fs::write(&path, URL_SAFE_NO_PAD.encode([1_u8; 3])).unwrap();
        let incorrect_length = daemon_control_token_at(&path).unwrap_err();
        assert!(incorrect_length.contains("does not contain a valid daemon control token"));

        let token = URL_SAFE_NO_PAD.encode([7_u8; 32]);
        fs::write(&path, format!("{token}\n")).unwrap();
        assert_eq!(daemon_control_token_at(&path).unwrap(), token);

        let _ = fs::remove_file(path);
    }

    #[test]
    fn verify_daemon_rejects_invalid_proof_encoding() {
        let token = URL_SAFE_NO_PAD.encode([9_u8; 32]);
        let (address, server) = mock_challenge_server("not*base64");

        let error = verify_daemon_at(&token, &address).unwrap_err();

        assert!(error.contains("returned an invalid proof"));
        server.join().unwrap();
    }

    #[test]
    fn verify_daemon_rejects_incorrect_hmac_proof() {
        let token = URL_SAFE_NO_PAD.encode([9_u8; 32]);
        let wrong_proof = URL_SAFE_NO_PAD.encode([0_u8; 32]);
        let (address, server) = mock_challenge_server(&wrong_proof);

        let error = verify_daemon_at(&token, &address).unwrap_err();

        assert!(error.contains("failed identity verification"));
        server.join().unwrap();
    }
}
