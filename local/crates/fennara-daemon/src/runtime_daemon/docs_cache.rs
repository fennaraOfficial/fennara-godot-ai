use serde::Deserialize;
use serde_json::Value;
use std::{
    path::PathBuf,
    sync::atomic::Ordering,
    time::{Duration, SystemTime},
};

use super::{
    state::AppState,
    util::{fennara_app_dir, sanitize_path_component},
};

const DOCS_BRANCH: &str = "master";
const DOCS_HOST: &str = "https://raw.githubusercontent.com";
const DOCS_CACHE_TTL_SECS: u64 = 7 * 24 * 60 * 60;
const DOC_MODULES: &[&str] = &[
    "noise",
    "csg",
    "gridmap",
    "gdscript",
    "gltf",
    "multiplayer",
    "navigation_2d",
    "navigation_3d",
    "websocket",
    "webrtc",
    "openxr",
    "interactive_music",
    "regex",
    "text_server_adv",
    "text_server_fb",
    "theora",
    "vorbis",
    "mono",
    "fbx",
    "svg",
    "webxr",
    "mobile_vr",
    "mp3",
    "upnp",
    "jsonrpc",
    "enet",
    "zip",
    "godot_physics_2d",
    "godot_physics_3d",
    "jolt_physics",
];

#[derive(Debug, Deserialize)]
struct WarmDocsRequest {
    branch: Option<String>,
    class_names: Vec<String>,
}

pub(crate) async fn handle_docs_warmup_request(state: &AppState, value: &Value) {
    let request: WarmDocsRequest = match serde_json::from_value(value.clone()) {
        Ok(request) => request,
        Err(_) => return,
    };

    let branch = request
        .branch
        .unwrap_or_else(|| DOCS_BRANCH.to_string())
        .trim()
        .to_string();
    if branch.is_empty() || request.class_names.is_empty() {
        return;
    }

    if state
        .docs_warmup_running
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return;
    }

    let warmup_flag = state.docs_warmup_running.clone();
    tokio::spawn(async move {
        let client = match reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent("FennaraDaemon/1.0")
            .build()
        {
            Ok(client) => client,
            Err(_) => {
                warmup_flag.store(false, Ordering::SeqCst);
                return;
            }
        };

        for class_name in request.class_names {
            let class_name = class_name.trim();
            if class_name.is_empty() {
                continue;
            }
            let _ = warm_cached_doc(&client, &branch, class_name).await;
        }

        warmup_flag.store(false, Ordering::SeqCst);
    });
}

async fn warm_cached_doc(
    client: &reqwest::Client,
    branch: &str,
    class_name: &str,
) -> Result<(), String> {
    match warm_cached_doc_for_branch(client, branch, class_name).await {
        Ok(()) => Ok(()),
        Err(err) if branch != DOCS_BRANCH => {
            warm_cached_doc_for_branch(client, DOCS_BRANCH, class_name)
                .await
                .map_err(|fallback_err| {
                    format!(
                        "branch {branch} failed: {err}; fallback branch {DOCS_BRANCH} failed: {fallback_err}"
                    )
                })
        }
        Err(err) => Err(err),
    }
}

async fn warm_cached_doc_for_branch(
    client: &reqwest::Client,
    branch: &str,
    class_name: &str,
) -> Result<(), String> {
    let cache_path = cache_file_path(branch, class_name)?;
    if cache_path.is_file() && cache_file_is_fresh(&cache_path) {
        return Ok(());
    }

    if let Some(parent) = cache_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|err| format!("create cache dir failed: {err}"))?;
    }

    let xml = fetch_class_xml(client, branch, class_name).await?;
    tokio::fs::write(&cache_path, xml)
        .await
        .map_err(|err| format!("write cache file failed: {err}"))?;
    Ok(())
}

fn cache_file_is_fresh(path: &std::path::Path) -> bool {
    let metadata = match std::fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(_) => return false,
    };
    let modified = match metadata.modified() {
        Ok(modified) => modified,
        Err(_) => return false,
    };
    let age = match SystemTime::now().duration_since(modified) {
        Ok(age) => age,
        Err(_) => return false,
    };
    age.as_secs() <= DOCS_CACHE_TTL_SECS
}

fn cache_file_path(branch: &str, class_name: &str) -> Result<PathBuf, String> {
    Ok(fennara_app_dir()?
        .join(".cache")
        .join("godot_docs")
        .join(sanitize_path_component(branch))
        .join(format!("{}.xml", sanitize_path_component(class_name))))
}

async fn fetch_class_xml(
    client: &reqwest::Client,
    branch: &str,
    class_name: &str,
) -> Result<String, String> {
    let core_path = format!("{DOCS_HOST}/godotengine/godot/{branch}/doc/classes/{class_name}.xml");
    if let Some(body) = fetch_if_present(client, &core_path).await? {
        return Ok(body);
    }

    for module_name in DOC_MODULES {
        let module_path = format!(
            "{DOCS_HOST}/godotengine/godot/{branch}/modules/{module_name}/doc_classes/{class_name}.xml"
        );
        if let Some(body) = fetch_if_present(client, &module_path).await? {
            return Ok(body);
        }
    }

    Err(format!("class docs not found for {class_name}"))
}

async fn fetch_if_present(client: &reqwest::Client, url: &str) -> Result<Option<String>, String> {
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|err| format!("request failed: {err}"))?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    if !response.status().is_success() {
        return Err(format!("unexpected status {} for {url}", response.status()));
    }

    response
        .text()
        .await
        .map(Some)
        .map_err(|err| format!("read body failed: {err}"))
}
