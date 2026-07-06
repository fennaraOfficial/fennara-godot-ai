#![cfg(unix)]

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::os::fd::{FromRawFd, IntoRawFd};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[test]
fn launcher_answers_initialize_when_stdio_uses_socketpairs() {
    let app_dir = temp_app_dir();
    let bin_dir = app_dir.join("bin");
    fs::create_dir_all(&bin_dir).expect("create fake app bin dir");

    let launcher = bin_dir.join("fennara-mcp");
    fs::copy(env!("CARGO_BIN_EXE_fennara-mcp"), &launcher).expect("copy launcher");
    fs::set_permissions(&launcher, fs::Permissions::from_mode(0o755))
        .expect("make launcher executable");

    let runtime = env!("CARGO_BIN_EXE_fennara-mcp-runtime").replace('\\', "\\\\");
    fs::write(
        app_dir.join("current.json"),
        format!(r#"{{"mcp_runtime":"{runtime}"}}"#),
    )
    .expect("write fake current manifest");

    let (mut stdin_parent, stdin_child) = UnixStream::pair().expect("stdin socketpair");
    let (stdout_parent, stdout_child) = UnixStream::pair().expect("stdout socketpair");
    stdout_parent
        .set_read_timeout(Some(Duration::from_secs(5)))
        .expect("set stdout read timeout");

    let mut child = Command::new(&launcher)
        .stdin(unsafe { Stdio::from_raw_fd(stdin_child.into_raw_fd()) })
        .stdout(unsafe { Stdio::from_raw_fd(stdout_child.into_raw_fd()) })
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn launcher with socket stdio");

    let request = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"socketpair-test","version":"0"}}}"#;
    writeln!(stdin_parent, "{request}").expect("write initialize request");

    let mut line = String::new();
    let mut reader = BufReader::new(stdout_parent);
    reader
        .read_line(&mut line)
        .expect("read initialize response from socket stdout");

    let _ = child.kill();
    let _ = child.wait();
    let _ = fs::remove_dir_all(&app_dir);

    assert!(
        line.contains(r#""id":1"#) && line.contains(r#""result""#),
        "expected initialize result, got: {line:?}"
    );
}

fn temp_app_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "fennara-mcp-socketpair-test-{}-{unique}",
        std::process::id()
    ))
}
