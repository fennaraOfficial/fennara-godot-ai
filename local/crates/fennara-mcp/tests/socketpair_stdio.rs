#![cfg(unix)]

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use serde_json::{Value, json};
use sha2::Sha256;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::fd::{FromRawFd, IntoRawFd};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

#[test]
fn launcher_carries_its_project_binding_through_stdio_to_daemon_requests() {
    let fixture = Fixture::new();
    let project = fixture.project("bound-project");
    let canonical_project = fs::canonicalize(&project).expect("canonical project path");
    let control_token = URL_SAFE_NO_PAD.encode([42_u8; 32]);
    let daemon = FakeDaemon::start(control_token.clone(), &canonical_project);
    let mut mcp = fixture.launcher(
        &["--project-path".as_ref(), project.as_os_str()],
        &control_token,
        daemon.address(),
    );

    let initialize = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2025-06-18",
            "capabilities": {},
            "clientInfo": { "name": "socketpair-test", "version": "0" }
        }
    });
    mcp.request(&initialize);
    assert!(mcp.response()["result"].is_object());

    mcp.request(&json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list",
        "params": {}
    }));
    let tools = mcp.response();
    for tool in tools["result"]["tools"]
        .as_array()
        .expect("tools/list result")
    {
        assert!(
            tool["inputSchema"]["properties"]
                .get("project_path")
                .is_none(),
            "model-facing tool {} must not expose a per-call project selector",
            tool["name"]
        );
    }

    mcp.request(&json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": { "name": "fennara_status", "arguments": {} }
    }));
    let status = mcp.response();
    let status_text = status["result"]["content"][0]["text"]
        .as_str()
        .expect("status text");
    assert!(status_text.contains("Routing mode: bound"));
    assert!(status_text.contains("Binding source: cli"));
    assert!(status_text.contains(&markdown_escape(
        canonical_project.to_str().unwrap()
    )));

    mcp.request(&json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "tools/call",
        "params": {
            "name": "script_diagnostics",
            "arguments": {}
        }
    }));
    assert_eq!(mcp.response()["result"]["isError"], false);

    let posts = daemon.finish();
    assert_eq!(posts.len(), 2);
    assert_eq!(posts[0].0, "/status/bound");
    assert_eq!(
        posts[0].1["project_path"],
        canonical_project.to_str().unwrap()
    );
    assert_eq!(posts[1].0, "/tools/call");
    assert_eq!(posts[1].1["tool"], "script_diagnostics");
    assert_eq!(
        posts[1].1["project_path"],
        canonical_project.to_str().unwrap()
    );
    assert!(posts[1].1["args"].get("project_path").is_none());
}

struct Fixture {
    root: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "fennara-mcp-socketpair-test-{}-{unique}",
            std::process::id()
        ));
        let bin_dir = root.join("app/bin");
        fs::create_dir_all(&bin_dir).expect("create fake app bin dir");

        let launcher = bin_dir.join("fennara-mcp");
        fs::copy(env!("CARGO_BIN_EXE_fennara-mcp"), &launcher).expect("copy launcher");
        fs::set_permissions(&launcher, fs::Permissions::from_mode(0o755))
            .expect("make launcher executable");
        fs::write(
            root.join("app/current.json"),
            json!({ "mcp_runtime": env!("CARGO_BIN_EXE_fennara-mcp-runtime") }).to_string(),
        )
        .expect("write fake current manifest");
        Self { root }
    }

    fn project(&self, name: &str) -> PathBuf {
        let project = self.root.join(name);
        fs::create_dir(&project).expect("create project root");
        fs::write(project.join("project.godot"), "[application]\n").expect("write project.godot");
        project
    }

    fn launcher(
        &self,
        args: &[&std::ffi::OsStr],
        control_token: &str,
        daemon_address: &str,
    ) -> McpProcess {
        let data_dir = self.root.join("data");
        let xdg_token_dir = data_dir.join("fennara");
        let macos_token_dir = self
            .root
            .join("Library")
            .join("Application Support")
            .join("Fennara");
        fs::create_dir_all(&xdg_token_dir).expect("create XDG app-data directory");
        fs::create_dir_all(&macos_token_dir).expect("create macOS app-data directory");
        let token = format!("{control_token}\n");
        fs::write(xdg_token_dir.join("daemon-control-token"), &token)
            .expect("write XDG control token");
        fs::write(macos_token_dir.join("daemon-control-token"), &token)
            .expect("write macOS control token");

        let (stdin_parent, stdin_child) = UnixStream::pair().expect("stdin socketpair");
        let (stdout_parent, stdout_child) = UnixStream::pair().expect("stdout socketpair");
        stdout_parent
            .set_read_timeout(Some(Duration::from_secs(5)))
            .expect("set stdout read timeout");

        let child = Command::new(self.root.join("app/bin/fennara-mcp"))
            .args(args)
            .env("HOME", &self.root)
            .env("XDG_DATA_HOME", data_dir)
            .env("FENNARA_TEST_DAEMON_ADDR", daemon_address)
            .stdin(unsafe { Stdio::from_raw_fd(stdin_child.into_raw_fd()) })
            .stdout(unsafe { Stdio::from_raw_fd(stdout_child.into_raw_fd()) })
            .stderr(Stdio::inherit())
            .spawn()
            .expect("spawn launcher with socket stdio");

        McpProcess {
            child,
            stdin: stdin_parent,
            stdout: BufReader::new(stdout_parent),
        }
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

struct McpProcess {
    child: Child,
    stdin: UnixStream,
    stdout: BufReader<UnixStream>,
}

impl McpProcess {
    fn request(&mut self, request: &Value) {
        writeln!(self.stdin, "{request}").expect("write MCP request");
    }

    fn response(&mut self) -> Value {
        let mut line = String::new();
        self.stdout.read_line(&mut line).expect("read MCP response");
        serde_json::from_str(&line).unwrap_or_else(|error| {
            panic!("parse MCP response {line:?}: {error}");
        })
    }
}

impl Drop for McpProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

struct FakeDaemon {
    address: String,
    posts: mpsc::Receiver<(String, Value)>,
    handle: Option<thread::JoinHandle<()>>,
}

impl FakeDaemon {
    fn start(control_token: String, project: &Path) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind fake daemon");
        listener
            .set_nonblocking(true)
            .expect("make fake daemon nonblocking");
        let address = listener.local_addr().unwrap().to_string();
        let project = project.to_str().unwrap().to_string();
        let (posts_tx, posts) = mpsc::channel();
        let handle = thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(8);
            let mut handled = 0;
            while handled < 4 && Instant::now() < deadline {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        let request = read_http_request(&mut stream);
                        let (request_line, headers, body) = request
                            .split_once("\r\n")
                            .and_then(|(line, rest)| {
                                rest.split_once("\r\n\r\n")
                                    .map(|(headers, body)| (line, headers, body))
                            })
                            .expect("parse fake-daemon request");
                        let mut fields = request_line.split_whitespace();
                        let method = fields.next().unwrap();
                        let path = fields.next().unwrap();
                        if let Some(nonce) = path.strip_prefix("/control/challenge?nonce=") {
                            let nonce = URL_SAFE_NO_PAD.decode(nonce).expect("decode nonce");
                            let token = URL_SAFE_NO_PAD
                                .decode(&control_token)
                                .expect("decode control token");
                            let mut mac = HmacSha256::new_from_slice(&token).unwrap();
                            mac.update(&nonce);
                            respond_json(
                                &mut stream,
                                &json!({
                                    "ok": true,
                                    "proof": URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes())
                                }),
                            );
                        } else {
                            assert_eq!(method, "POST");
                            let expected_control_header =
                                format!("x-fennara-control-token: {control_token}");
                            assert!(headers
                                .lines()
                                .any(|line| line.eq_ignore_ascii_case(&expected_control_header)));
                            let body: Value = serde_json::from_str(body).expect("parse POST body");
                            posts_tx.send((path.to_string(), body)).unwrap();
                            if path == "/status/bound" {
                                respond_json(
                                    &mut stream,
                                    &json!({
                                        "ok": true,
                                        "daemon": "fennara-daemon",
                                        "version": "test",
                                        "godot_plugin_connected": true,
                                        "routing_mode": "bound",
                                        "bound_editor_state": "connected",
                                        "selected_project": {
                                            "session_id": "fake-session",
                                            "project_name": "Fixture",
                                            "project_path": project,
                                        },
                                        "editor_filesystem": {
                                            "state": "ready",
                                            "asset_tools_ready": true,
                                        },
                                        "active_project": null,
                                        "legacy_active_project": null,
                                        "active_session_id": null,
                                        "connected_projects": [],
                                    }),
                                );
                            } else {
                                respond_json(
                                    &mut stream,
                                    &json!({ "ok": true, "marker": "fake-daemon" }),
                                );
                            }
                        }
                        handled += 1;
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(error) => panic!("accept fake-daemon request: {error}"),
                }
            }
            assert_eq!(handled, 4, "fake daemon did not receive every MCP request");
        });
        Self {
            address,
            posts,
            handle: Some(handle),
        }
    }

    fn address(&self) -> &str {
        &self.address
    }

    fn finish(mut self) -> Vec<(String, Value)> {
        self.handle.take().unwrap().join().unwrap();
        self.posts.try_iter().collect()
    }
}

fn read_http_request(stream: &mut TcpStream) -> String {
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();
    let mut request = Vec::new();
    let mut buffer = [0_u8; 2048];
    let mut expected_len = None;
    loop {
        let read = stream.read(&mut buffer).expect("read HTTP request");
        assert!(read > 0, "HTTP connection closed before request completed");
        request.extend_from_slice(&buffer[..read]);
        if expected_len.is_none()
            && let Some(header_end) = request.windows(4).position(|bytes| bytes == b"\r\n\r\n")
        {
            let headers = String::from_utf8_lossy(&request[..header_end]);
            let content_len = headers
                .lines()
                .find_map(|line| {
                    line.to_ascii_lowercase()
                        .strip_prefix("content-length: ")
                        .and_then(|value| value.parse::<usize>().ok())
                })
                .unwrap_or(0);
            expected_len = Some(header_end + 4 + content_len);
        }
        if expected_len.is_some_and(|length| request.len() >= length) {
            break;
        }
    }
    String::from_utf8(request).expect("HTTP request should be UTF-8")
}

fn markdown_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        if matches!(
            ch,
            '\\' | '`' | '*' | '_' | '[' | ']' | '<' | '>' | '(' | ')'
        ) {
            escaped.push('\\');
        }
        escaped.push(ch);
    }
    escaped
}

fn respond_json(stream: &mut TcpStream, value: &Value) {
    let body = value.to_string();
    write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
    .expect("write fake-daemon response");
}
