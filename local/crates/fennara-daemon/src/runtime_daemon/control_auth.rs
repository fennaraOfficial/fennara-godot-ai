use axum::{
    Json,
    extract::{Extension, Query, Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use serde::Deserialize;
use serde_json::json;
use sha2::Sha256;
use std::{
    fs::{self, OpenOptions},
    io::{ErrorKind, Write},
    path::Path,
    sync::Arc,
};

use super::util::fennara_app_dir;

pub(crate) const CONTROL_HEADER: &str = "x-fennara-control-token";
const CONTROL_TOKEN_FILE: &str = "daemon-control-token";
const CONTROL_TOKEN_BYTES: usize = 32;
type HmacSha256 = Hmac<Sha256>;

#[derive(Deserialize)]
pub(crate) struct ChallengeQuery {
    nonce: String,
}

pub(crate) fn load_or_create() -> Result<Arc<str>, String> {
    let path = fennara_app_dir()?.join(CONTROL_TOKEN_FILE);
    load_or_create_at(&path).map(Arc::from)
}

pub(crate) async fn require_control_auth(
    State(expected): State<Arc<str>>,
    request: Request,
    next: Next,
) -> Response {
    if is_authorized(request.headers(), &expected) {
        return next.run(request).await;
    }

    (
        StatusCode::UNAUTHORIZED,
        Json(json!({
            "ok": false,
            "error": "Local daemon control authentication failed."
        })),
    )
        .into_response()
}

pub(crate) async fn challenge(
    Extension(token): Extension<Arc<str>>,
    Query(query): Query<ChallengeQuery>,
) -> Response {
    if query.nonce.len() != 43 {
        return challenge_error();
    }
    let Ok(nonce) = URL_SAFE_NO_PAD.decode(&query.nonce) else {
        return challenge_error();
    };
    if nonce.len() != CONTROL_TOKEN_BYTES {
        return challenge_error();
    }

    let token_bytes = URL_SAFE_NO_PAD
        .decode(token.as_ref())
        .expect("stored control token should remain valid");
    let mut mac = HmacSha256::new_from_slice(&token_bytes)
        .expect("HMAC accepts control tokens of any length");
    mac.update(&nonce);
    Json(json!({
        "ok": true,
        "proof": URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes())
    }))
    .into_response()
}

fn challenge_error() -> Response {
    (
        StatusCode::BAD_REQUEST,
        Json(json!({
            "ok": false,
            "error": "A 32-byte URL-safe base64 nonce is required."
        })),
    )
        .into_response()
}

fn is_authorized(headers: &HeaderMap, expected: &str) -> bool {
    headers
        .get(CONTROL_HEADER)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value == expected)
}

fn load_or_create_at(path: &Path) -> Result<String, String> {
    if let Some(token) = read_valid_token(path)? {
        return Ok(token);
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
    }

    let token = generate_token()?;
    let temp_path = path.with_extension(format!("{}.tmp", std::process::id()));
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }

    let _ = fs::remove_file(&temp_path);
    let mut file = options
        .open(&temp_path)
        .map_err(|error| format!("failed to create {}: {error}", temp_path.display()))?;
    if let Err(error) = file
        .write_all(token.as_bytes())
        .and_then(|_| file.write_all(b"\n"))
        .and_then(|_| file.sync_all())
    {
        let _ = fs::remove_file(&temp_path);
        return Err(format!("failed to write {}: {error}", temp_path.display()));
    }
    drop(file);

    match fs::hard_link(&temp_path, path) {
        Ok(()) => {
            let _ = fs::remove_file(&temp_path);
            Ok(token)
        }
        Err(error) if error.kind() == ErrorKind::AlreadyExists => {
            let _ = fs::remove_file(&temp_path);
            read_valid_token(path)?
                .ok_or_else(|| format!("{} does not contain a valid control token", path.display()))
        }
        Err(error) => {
            let _ = fs::remove_file(&temp_path);
            Err(format!("failed to publish {}: {error}", path.display()))
        }
    }
}

fn read_valid_token(path: &Path) -> Result<Option<String>, String> {
    let raw = match fs::read_to_string(path) {
        Ok(raw) => raw,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(format!("failed to read {}: {error}", path.display())),
    };
    let token = raw.trim();
    let decoded = URL_SAFE_NO_PAD.decode(token).ok();
    Ok(decoded
        .filter(|bytes| bytes.len() == CONTROL_TOKEN_BYTES)
        .map(|_| token.to_string()))
}

fn generate_token() -> Result<String, String> {
    let mut bytes = [0_u8; CONTROL_TOKEN_BYTES];
    getrandom::fill(&mut bytes)
        .map_err(|error| format!("failed to generate daemon control token: {error}"))?;
    Ok(URL_SAFE_NO_PAD.encode(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "fennara-control-auth-{}-{name}",
            std::process::id()
        ))
    }

    #[test]
    fn creates_and_reuses_a_valid_control_token() {
        let path = test_path("create");
        let _ = fs::remove_file(&path);

        let first = load_or_create_at(&path).expect("control token should be created");
        let second = load_or_create_at(&path).expect("control token should be reused");

        assert_eq!(first, second);
        assert_eq!(
            URL_SAFE_NO_PAD.decode(first).unwrap().len(),
            CONTROL_TOKEN_BYTES
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn requires_the_exact_control_header() {
        let mut headers = HeaderMap::new();
        assert!(!is_authorized(&headers, "expected"));

        headers.insert(CONTROL_HEADER, "wrong".parse().unwrap());
        assert!(!is_authorized(&headers, "expected"));

        headers.insert(CONTROL_HEADER, "expected".parse().unwrap());
        assert!(is_authorized(&headers, "expected"));
    }
}
