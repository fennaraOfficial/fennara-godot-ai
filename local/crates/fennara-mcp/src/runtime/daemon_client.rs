use serde_json::{Value, json};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

const DAEMON_ADDR: &str = "127.0.0.1:41287";

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
    let mut stream = TcpStream::connect(DAEMON_ADDR)
        .map_err(|error| format!("Open a Godot project with Fennara enabled. ({error})"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .map_err(|error| error.to_string())?;
    stream
        .set_write_timeout(Some(Duration::from_secs(2)))
        .map_err(|error| error.to_string())?;

    let request = format!("GET {path} HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n");
    stream
        .write_all(request.as_bytes())
        .map_err(|error| error.to_string())?;

    read_http_json_response(stream)
}

fn daemon_post(path: &str, body: &str) -> Result<Value, String> {
    let mut stream = TcpStream::connect(DAEMON_ADDR)
        .map_err(|error| format!("Open a Godot project with Fennara enabled. ({error})"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(300)))
        .map_err(|error| error.to_string())?;
    stream
        .set_write_timeout(Some(Duration::from_secs(2)))
        .map_err(|error| error.to_string())?;

    let request = format!(
        "POST {path} HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    stream
        .write_all(request.as_bytes())
        .map_err(|error| error.to_string())?;

    read_http_json_response(stream)
}

fn read_http_json_response(mut stream: TcpStream) -> Result<Value, String> {
    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .map_err(|error| error.to_string())?;

    let (headers, body) = response
        .split_once("\r\n\r\n")
        .ok_or_else(|| "invalid daemon HTTP response".to_string())?;
    if !headers.starts_with("HTTP/1.1 200") && !headers.starts_with("HTTP/1.0 200") {
        return Err("daemon returned non-200 status".to_string());
    }
    serde_json::from_str(body).map_err(|error| error.to_string())
}
