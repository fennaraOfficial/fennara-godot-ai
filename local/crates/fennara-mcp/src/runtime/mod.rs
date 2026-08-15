mod daemon_client;
mod protocol;
#[cfg(test)]
mod schema_tests;
mod schemas;
mod tools;

use fennara_project_identity::ProjectRoot;
use serde_json::Value;
use std::{
    env,
    ffi::{OsStr, OsString},
    io::{self, BufRead, Write},
    path::Path,
};

const PROJECT_PATH_ENV: &str = "FENNARA_PROJECT_PATH";
const PROJECT_PATH_FLAG: &str = "--project-path";
const PROJECT_PATH_EQUALS_PREFIX: &[u8] = b"--project-path=";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum BindingSource {
    Cli,
    Environment,
    Cwd,
}

impl BindingSource {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Cli => "cli",
            Self::Environment => "environment",
            Self::Cwd => "cwd",
        }
    }
}

#[derive(Clone, Debug)]
struct ProjectBinding {
    root: ProjectRoot,
    source: BindingSource,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct RuntimeConfig {
    binding: Option<ProjectBinding>,
}

impl RuntimeConfig {
    pub(crate) fn from_process() -> Result<Self, String> {
        let cwd = env::current_dir().map_err(|error| {
            format!("failed to resolve the Fennara MCP working directory: {error}")
        })?;
        Self::from_args_and_env(env::args_os().skip(1), env::var_os(PROJECT_PATH_ENV), &cwd)
    }

    fn from_args_and_env(
        args: impl IntoIterator<Item = OsString>,
        env_project_path: Option<OsString>,
        cwd: &Path,
    ) -> Result<Self, String> {
        let mut args = args.into_iter();
        let mut cli_project_path = None;

        while let Some(arg) = args.next() {
            if arg == PROJECT_PATH_FLAG {
                if cli_project_path.is_some() {
                    return Err("--project-path may be provided only once".to_string());
                }
                let value = args
                    .next()
                    .ok_or_else(|| "--project-path requires a path".to_string())?;
                cli_project_path = Some(value);
            } else if let Some(value) = project_path_equals_value(&arg) {
                if cli_project_path.is_some() {
                    return Err("--project-path may be provided only once".to_string());
                }
                cli_project_path = Some(value.to_owned());
            } else {
                return Err(format!("unknown Fennara MCP option: {arg:?}"));
            }
        }

        let binding = if let Some(value) = cli_project_path {
            Some(resolve_binding(&value, cwd, BindingSource::Cli)?)
        } else if let Some(value) = env_project_path {
            Some(resolve_binding(&value, cwd, BindingSource::Environment)?)
        } else {
            ProjectRoot::discover_from(cwd)
                .map_err(|error| format!("invalid_project_binding: {error}"))?
                .map(|root| ProjectBinding {
                    root,
                    source: BindingSource::Cwd,
                })
        };

        Ok(Self { binding })
    }

    pub(crate) fn project_root(&self) -> Option<&ProjectRoot> {
        self.binding.as_ref().map(|binding| &binding.root)
    }

    pub(crate) fn project_path(&self) -> Option<&str> {
        self.project_root().map(ProjectRoot::as_protocol_str)
    }

    pub(crate) fn binding_source(&self) -> Option<BindingSource> {
        self.binding.as_ref().map(|binding| binding.source)
    }
}

fn resolve_binding(
    value: &OsStr,
    cwd: &Path,
    source: BindingSource,
) -> Result<ProjectBinding, String> {
    ProjectRoot::resolve_from(value, cwd)
        .map(|root| ProjectBinding { root, source })
        .map_err(|error| format!("invalid_project_binding: {error}"))
}

fn project_path_equals_value(arg: &OsStr) -> Option<&OsStr> {
    let bytes = arg.as_encoded_bytes();
    bytes
        .strip_prefix(PROJECT_PATH_EQUALS_PREFIX)
        // SAFETY: The suffix comes from the same OsStr, and the split follows the
        // ASCII '=' byte, which is a valid boundary in Rust's platform encoding.
        .map(|value| unsafe { OsStr::from_encoded_bytes_unchecked(value) })
}

pub(crate) fn run_stdio(config: RuntimeConfig) {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let Ok(line) = line else {
            continue;
        };

        if line.trim().is_empty() {
            continue;
        }

        let response = match serde_json::from_str::<Value>(&line) {
            Ok(request) => protocol::handle_request(request, &config),
            Err(error) => Some(protocol::error_response(
                Value::Null,
                -32700,
                format!("Parse error: {error}"),
            )),
        };

        if let Some(response) = response {
            if writeln!(stdout, "{response}").is_err() {
                break;
            }
            let _ = stdout.flush();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BindingSource, RuntimeConfig, protocol, schemas, tools};
    use serde_json::{Value, json};
    use std::{
        env,
        ffi::OsString,
        fs,
        path::{Path, PathBuf},
        sync::atomic::{AtomicU64, Ordering},
    };

    static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

    struct Fixture {
        path: PathBuf,
    }

    impl Fixture {
        fn new() -> Self {
            let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
            let path = env::temp_dir().join(format!(
                "fennara-mcp-binding-{}-{sequence}",
                std::process::id()
            ));
            fs::create_dir(&path).expect("create MCP binding fixture");
            Self { path }
        }

        fn project(&self, relative: impl AsRef<Path>) -> PathBuf {
            let root = self.path.join(relative);
            fs::create_dir_all(&root).expect("create Godot project root");
            fs::write(root.join("project.godot"), b"[application]\n").expect("write project.godot");
            root
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.path).expect("remove MCP binding fixture");
        }
    }

    fn listed_tool_names() -> Vec<String> {
        tools::tools_list_result()["tools"]
            .as_array()
            .expect("tools/list should return a tools array")
            .iter()
            .filter_map(|tool| tool.get("name").and_then(Value::as_str))
            .map(ToOwned::to_owned)
            .collect()
    }

    fn initialize_request(protocol_version: &str) -> Value {
        json!({
            "protocolVersion": protocol_version,
            "capabilities": {},
            "clientInfo": {
                "name": "test-client",
                "version": "0.0.0"
            }
        })
    }

    #[test]
    fn cli_project_binding_takes_precedence_over_environment_and_cwd() {
        let fixture = Fixture::new();
        let cli_root = fixture.project("cli");
        let env_root = fixture.project("environment");
        let cwd_root = fixture.project("cwd");
        let cwd = cwd_root.join("src/nested");
        fs::create_dir_all(&cwd).unwrap();

        let from_cli = RuntimeConfig::from_args_and_env(
            [
                OsString::from("--project-path"),
                cli_root.as_os_str().to_owned(),
            ],
            Some(env_root.as_os_str().to_owned()),
            &cwd,
        )
        .expect("CLI project binding should resolve");

        assert_eq!(
            from_cli.project_path(),
            fs::canonicalize(cli_root).unwrap().to_str()
        );
        assert_eq!(from_cli.binding_source(), Some(BindingSource::Cli));
    }

    #[test]
    fn environment_project_binding_takes_precedence_over_cwd() {
        let fixture = Fixture::new();
        let env_root = fixture.project("environment");
        let cwd_root = fixture.project("cwd");
        let cwd = cwd_root.join("src/nested");
        fs::create_dir_all(&cwd).unwrap();

        let from_env = RuntimeConfig::from_args_and_env(
            Vec::<OsString>::new(),
            Some(env_root.as_os_str().to_owned()),
            &cwd,
        )
        .expect("environment project binding should resolve");

        assert_eq!(
            from_env.project_path(),
            fs::canonicalize(env_root).unwrap().to_str()
        );
        assert_eq!(from_env.binding_source(), Some(BindingSource::Environment));
    }

    #[test]
    fn cwd_discovery_binds_to_the_nearest_godot_project_ancestor() {
        let fixture = Fixture::new();
        let outer = fixture.project("outer");
        let inner = fixture.project("outer/packages/inner");
        let cwd = inner.join("src/nested");
        fs::create_dir_all(&cwd).unwrap();

        let config = RuntimeConfig::from_args_and_env(Vec::<OsString>::new(), None, &cwd)
            .expect("cwd discovery should resolve");

        assert_eq!(
            config.project_path(),
            fs::canonicalize(inner).unwrap().to_str()
        );
        assert_ne!(
            config.project_path(),
            fs::canonicalize(outer).unwrap().to_str()
        );
        assert_eq!(config.binding_source(), Some(BindingSource::Cwd));
    }

    #[test]
    fn cwd_without_a_godot_project_uses_legacy_unbound_mode() {
        let fixture = Fixture::new();
        let cwd = fixture.path.join("ordinary/nested");
        fs::create_dir_all(&cwd).unwrap();

        let config = RuntimeConfig::from_args_and_env(Vec::<OsString>::new(), None, &cwd)
            .expect("missing automatic discovery is not an error");

        assert_eq!(config.project_path(), None);
        assert_eq!(config.binding_source(), None);
    }

    #[test]
    fn explicit_project_binding_failures_do_not_degrade_to_legacy_mode() {
        let fixture = Fixture::new();
        let cwd = fixture.path.join("ordinary");
        fs::create_dir(&cwd).unwrap();

        let invalid_cli = RuntimeConfig::from_args_and_env(
            [OsString::from("--project-path=missing")],
            None,
            &cwd,
        )
        .unwrap_err();
        assert!(invalid_cli.contains("invalid_project_binding"));

        let empty_environment =
            RuntimeConfig::from_args_and_env(Vec::<OsString>::new(), Some(OsString::new()), &cwd)
                .unwrap_err();
        assert!(empty_environment.contains("invalid_project_binding"));
    }

    #[cfg(unix)]
    #[test]
    fn native_cli_path_values_are_not_lossily_converted() {
        use std::os::unix::ffi::{OsStrExt, OsStringExt};

        let fixture = Fixture::new();
        let root = fixture.path.join(OsString::from_vec(b"game-\xff".to_vec()));
        fs::create_dir(&root).unwrap();
        fs::write(root.join("project.godot"), b"[application]\n").unwrap();
        let mut argument = b"--project-path=".to_vec();
        argument.extend_from_slice(root.as_os_str().as_bytes());

        let error =
            RuntimeConfig::from_args_and_env([OsString::from_vec(argument)], None, &fixture.path)
                .unwrap_err();

        assert!(error.contains("invalid_project_binding"));
        assert!(error.contains("Unicode protocol"));
    }

    #[test]
    fn runtime_config_rejects_missing_duplicate_and_unknown_cli_options() {
        let fixture = Fixture::new();
        let cwd = fixture.path.join("ordinary");
        fs::create_dir(&cwd).unwrap();
        let missing =
            RuntimeConfig::from_args_and_env([OsString::from("--project-path")], None, &cwd)
                .unwrap_err();
        assert!(missing.contains("requires a path"));

        let duplicate = RuntimeConfig::from_args_and_env(
            [
                OsString::from("--project-path=a"),
                OsString::from("--project-path=b"),
            ],
            None,
            &cwd,
        )
        .unwrap_err();
        assert!(duplicate.contains("only once"));

        let unknown =
            RuntimeConfig::from_args_and_env([OsString::from("--workspace=project")], None, &cwd)
                .unwrap_err();
        assert!(unknown.contains("unknown Fennara MCP option"));
    }

    #[test]
    fn initialize_negotiates_2025_06_18_when_requested() {
        let params = initialize_request("2025-06-18");
        let result = protocol::initialize_result(Some(&params));

        assert_eq!(result["protocolVersion"], "2025-06-18");
    }

    #[test]
    fn initialize_negotiates_2025_03_26_when_requested() {
        let params = initialize_request("2025-03-26");
        let result = protocol::initialize_result(Some(&params));

        assert_eq!(result["protocolVersion"], "2025-03-26");
    }

    #[test]
    fn initialize_falls_back_to_latest_supported_protocol() {
        let params = initialize_request("2024-11-05");
        let result = protocol::initialize_result(Some(&params));

        assert_eq!(result["protocolVersion"], protocol::MCP_PROTOCOL_VERSION);
    }

    #[test]
    fn tools_list_includes_expected_forwarded_tools() {
        let tool_names = listed_tool_names();

        assert!(tool_names.iter().any(|name| name == "fennara_status"));

        for name in schemas::FORWARDED_TOOLS {
            assert!(
                tool_names.iter().any(|tool_name| tool_name == name),
                "expected tools/list to include {name}"
            );
        }
    }

    #[test]
    fn tools_list_does_not_include_git() {
        let tool_names = listed_tool_names();

        assert!(
            !tool_names.iter().any(|name| name == "git"),
            "tools/list should not expose git"
        );
    }

    #[test]
    fn run_scene_edit_script_description_is_flattened_for_mcp_clients() {
        let tool = schemas::tool_from_embedded_definition(include_str!(
            "../../../../schemas/tools/run_scene_edit_script.json"
        ))
        .expect("run_scene_edit_script definition should parse");

        let description = tool
            .get("description")
            .and_then(Value::as_str)
            .expect("run_scene_edit_script should expose description");

        assert!(description.contains("Run a one-off scene worker script"));
        assert!(description.contains("`mode=inspect`"));
        assert!(description.contains("script_path"));
        assert!(description.contains("ctx.get_scene_root()"));
        assert!(tool.get("description_lines").is_none());
    }

    #[test]
    fn run_asset_import_script_description_explains_rejected_setters() {
        let tool = schemas::tool_from_embedded_definition(include_str!(
            "../../../../schemas/tools/run_asset_import_script.json"
        ))
        .expect("run_asset_import_script definition should parse");

        let description = tool
            .get("description")
            .and_then(Value::as_str)
            .expect("run_asset_import_script should expose description");

        assert!(description.contains("set_import_option()` returns `true"));
        assert!(description.contains("discards every staged import change"));
        assert!(description.contains("if not ctx.set_import_option"));
        assert!(tool.get("description_lines").is_none());
    }

    #[test]
    fn tools_list_uses_embedded_schemas_without_remote_lookup() {
        let tool_names = listed_tool_names();

        assert!(
            tool_names
                .iter()
                .any(|name| name == "run_scene_edit_script")
        );
        assert!(tool_names.iter().any(|name| name == "project_settings"));
    }

    #[test]
    fn tools_list_schemas_are_openai_function_compatible_at_top_level() {
        let tools = tools::tools_list_result()["tools"]
            .as_array()
            .expect("tools/list should return a tools array")
            .clone();
        let unsupported_top_level_keys = ["oneOf", "anyOf", "allOf", "enum", "not"];

        for tool in tools {
            let name = tool
                .get("name")
                .and_then(Value::as_str)
                .expect("tool should have a name");
            let schema = tool
                .get("inputSchema")
                .expect("tool should have an inputSchema");

            assert_eq!(
                schema.get("type").and_then(Value::as_str),
                Some("object"),
                "{name} inputSchema must be a top-level object"
            );

            for key in unsupported_top_level_keys {
                assert!(
                    schema.get(key).is_none(),
                    "{name} inputSchema must not use top-level {key}"
                );
            }
        }
    }

    #[test]
    fn no_mcp_tool_exposes_a_project_path_selector() {
        let result = tools::tools_list_result();
        let tools = result["tools"]
            .as_array()
            .expect("tools/list should return a tools array");

        for tool in tools {
            let name = tool["name"].as_str().expect("tool should have a name");
            let schema = &tool["inputSchema"];
            assert!(
                schema.pointer("/properties/project_path").is_none(),
                "{name} must not expose process routing as a model-facing argument"
            );
        }
    }
}
