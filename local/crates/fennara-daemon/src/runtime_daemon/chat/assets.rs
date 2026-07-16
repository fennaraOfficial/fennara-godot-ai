use axum::{
    extract::{Path as AxumPath, Query},
    http::{HeaderMap, StatusCode, Uri, header},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use std::{
    fs,
    path::{Component, Path},
};

use super::{is_allowed_browser_origin, store};

const CACHE_CONTROL: &str = "no-store";
const MAX_TOOL_MEDIA_IMAGE_BYTES: u64 = 8 * 1024 * 1024;

struct Asset {
    content_type: &'static str,
    body: &'static [u8],
}

#[derive(Debug, Deserialize)]
pub(crate) struct ToolMediaQuery {
    token: Option<String>,
}

pub(crate) async fn chat_index() -> Response {
    asset_response("index.html")
}

pub(crate) async fn chat_index_redirect(uri: Uri) -> Response {
    let target = match uri.query() {
        Some(query) => format!("/chat/?{query}"),
        None => "/chat/".to_string(),
    };
    (StatusCode::TEMPORARY_REDIRECT, [(header::LOCATION, target)]).into_response()
}

pub(crate) async fn chat_asset(AxumPath(path): AxumPath<String>) -> Response {
    let path = path.trim_start_matches('/');
    if path.is_empty() || path.contains("..") || path.contains('\\') {
        return StatusCode::NOT_FOUND.into_response();
    }
    asset_response(path)
}

pub(crate) async fn chat_tool_media(
    AxumPath((tool_call_id, image_index)): AxumPath<(String, usize)>,
    Query(query): Query<ToolMediaQuery>,
    headers: HeaderMap,
) -> Response {
    if !is_allowed_browser_origin(&headers) {
        return StatusCode::FORBIDDEN.into_response();
    }
    if !is_safe_tool_call_id(&tool_call_id) {
        return StatusCode::NOT_FOUND.into_response();
    }
    let Some(access_token) = query.token.as_deref() else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let image = match store::tool_image_file(&tool_call_id, image_index, access_token) {
        Ok(Some(image)) => image,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let Some(mime_type) = normalize_supported_image_mime(&image.mime_type) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let path = Path::new(&image.file_path);
    let Ok(path) = fs::canonicalize(path) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    if !path.is_file() || !is_fennara_media_path(&path) {
        return StatusCode::NOT_FOUND.into_response();
    }
    let Ok(metadata) = fs::metadata(&path) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    if metadata.len() == 0 || metadata.len() > MAX_TOOL_MEDIA_IMAGE_BYTES {
        return StatusCode::NOT_FOUND.into_response();
    }
    let Ok(body) = fs::read(&path) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    if body.len() as u64 > MAX_TOOL_MEDIA_IMAGE_BYTES {
        return StatusCode::NOT_FOUND.into_response();
    }
    if detect_image_mime(&body) != Some(mime_type) {
        return StatusCode::NOT_FOUND.into_response();
    }
    (
        [
            (header::CONTENT_TYPE, mime_type.to_string()),
            (header::CACHE_CONTROL, CACHE_CONTROL.to_string()),
        ],
        body,
    )
        .into_response()
}

fn asset_response(path: &str) -> Response {
    let Some(asset) = asset(path) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    (
        [
            (header::CONTENT_TYPE, asset.content_type),
            (header::CACHE_CONTROL, CACHE_CONTROL),
        ],
        asset.body,
    )
        .into_response()
}

fn asset(path: &str) -> Option<Asset> {
    let asset = match path {
        "index.html" => Asset {
            content_type: "text/html; charset=utf-8",
            body: include_bytes!("../../../../../../godot_demo/addons/fennara/dist/index.html"),
        },
        "app.js" => js(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/app.js"
        )),
        "attachment-manager.js" => js(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/attachment-manager.js"
        )),
        "chat-navigation.js" => js(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/chat-navigation.js"
        )),
        "command-palette.js" => js(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/command-palette.js"
        )),
        "custom-provider-dialog.js" => js(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/custom-provider-dialog.js"
        )),
        "composer-actions.js" => js(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/composer-actions.js"
        )),
        "daemon-client.js" => js(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/daemon-client.js"
        )),
        "effort-controls.js" => js(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/effort-controls.js"
        )),
        "model-picker.js" => js(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/model-picker.js"
        )),
        "mcp-apps-settings.js" => js(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/mcp-apps-settings.js"
        )),
        "overlay-manager.js" => js(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/overlay-manager.js"
        )),
        "project-file-links.js" => js(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/project-file-links.js"
        )),
        "project-status.js" => js(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/project-status.js"
        )),
        "provider-popovers.js" => js(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/provider-popovers.js"
        )),
        "settings-panel.js" => js(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/settings-panel.js"
        )),
        "shell-bindings.js" => js(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/shell-bindings.js"
        )),
        "stored-transcript.js" => js(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/stored-transcript.js"
        )),
        "transcript-renderer.js" => js(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/transcript-renderer.js"
        )),
        "usage-summary.js" => js(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/usage-summary.js"
        )),
        "styles.css" => css(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/styles.css"
        )),
        "styles/base.css" => css(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/styles/base.css"
        )),
        "styles/chat.css" => css(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/styles/chat.css"
        )),
        "styles/controls.css" => css(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/styles/controls.css"
        )),
        "styles/custom-provider.css" => css(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/styles/custom-provider.css"
        )),
        "styles/drawer.css" => css(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/styles/drawer.css"
        )),
        "styles/icons.css" => css(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/styles/icons.css"
        )),
        "styles/model-picker.css" => css(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/styles/model-picker.css"
        )),
        "styles/responsive.css" => css(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/styles/responsive.css"
        )),
        "styles/settings.css" => css(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/styles/settings.css"
        )),
        "vendor/markdown-it.min.js" => js(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/vendor/markdown-it.min.js"
        )),
        "vendor/markdown-it-task-lists.min.js" => js(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/vendor/markdown-it-task-lists.min.js"
        )),
        "vendor/purify.min.js" => js(include_bytes!(
            "../../../../../../godot_demo/addons/fennara/dist/vendor/purify.min.js"
        )),
        _ => return None,
    };
    Some(asset)
}

fn css(body: &'static [u8]) -> Asset {
    Asset {
        content_type: "text/css; charset=utf-8",
        body,
    }
}

fn js(body: &'static [u8]) -> Asset {
    Asset {
        content_type: "text/javascript; charset=utf-8",
        body,
    }
}

fn is_safe_tool_call_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
}

fn normalize_supported_image_mime(mime: &str) -> Option<&'static str> {
    match mime.trim().to_ascii_lowercase().as_str() {
        "image/png" => Some("image/png"),
        "image/jpeg" | "image/jpg" => Some("image/jpeg"),
        "image/webp" => Some("image/webp"),
        "image/gif" => Some("image/gif"),
        _ => None,
    }
}

fn detect_image_mime(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        return Some("image/png");
    }
    if bytes.starts_with(b"\xff\xd8\xff") {
        return Some("image/jpeg");
    }
    if bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP" {
        return Some("image/webp");
    }
    if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        return Some("image/gif");
    }
    None
}

fn is_fennara_media_path(path: &Path) -> bool {
    path.components().any(|component| {
        matches!(
            component,
            Component::Normal(name) if name.to_string_lossy().eq_ignore_ascii_case(".fennara")
        )
    })
}

#[cfg(test)]
mod tests {
    use super::{asset, detect_image_mime, is_fennara_media_path, normalize_supported_image_mime};

    #[test]
    fn browser_chat_assets_referenced_by_html_are_embedded() {
        let index = include_str!("../../../../../../godot_demo/addons/fennara/dist/index.html");
        for line in index.lines() {
            assert_referenced_asset(line, "<script src=\"./");
            assert_referenced_asset(line, "<link rel=\"stylesheet\" href=\"./");
        }
    }

    #[test]
    fn browser_chat_stylesheet_imports_are_embedded() {
        let styles = include_str!("../../../../../../godot_demo/addons/fennara/dist/styles.css");
        for line in styles.lines() {
            assert_referenced_asset(line, "@import \"./");
        }
    }

    #[test]
    fn tool_media_route_accepts_only_supported_image_mimes() {
        assert_eq!(
            normalize_supported_image_mime("image/png"),
            Some("image/png")
        );
        assert_eq!(
            normalize_supported_image_mime("image/jpg"),
            Some("image/jpeg")
        );
        assert_eq!(normalize_supported_image_mime("text/plain"), None);
    }

    #[test]
    fn tool_media_route_checks_saved_file_magic() {
        assert_eq!(
            detect_image_mime(b"\x89PNG\r\n\x1a\nrest"),
            Some("image/png")
        );
        assert_eq!(detect_image_mime(b"\xff\xd8\xffrest"), Some("image/jpeg"));
        assert_eq!(detect_image_mime(b"not an image"), None);
    }

    #[test]
    fn tool_media_route_requires_fennara_media_path_component() {
        let temp = std::env::temp_dir();
        assert!(is_fennara_media_path(
            &temp.join(".fennara").join("shot.png")
        ));
        assert!(!is_fennara_media_path(&temp.join("shot.png")));
    }

    fn assert_referenced_asset(line: &str, prefix: &str) {
        let Some(start) = line.find(prefix) else {
            return;
        };
        let path_start = start + prefix.len();
        let Some(path_end) = line[path_start..].find('"') else {
            return;
        };
        let reference = &line[path_start..path_start + path_end];
        let path = reference.split(['?', '#']).next().unwrap_or(reference);
        assert!(
            asset(path).is_some(),
            "missing embedded chat asset: {reference}"
        );
    }
}
