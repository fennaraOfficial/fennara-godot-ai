use serde_json::{Value, json};
use std::{io::SeekFrom, path::Path};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncSeekExt},
    process::Child,
};

use super::state::RuntimeLogCursor;

const READY_MARKER: &str = "FENNARA_RUNTIME_SESSION_READY";
const STARTUP_ORIENTATION_MARKER: &str = "FENNARA_RUNTIME_ORIENTATION_NOTE";
const MAX_SHOWN_LINES: usize = 60;
const HEAD_LINES: usize = 20;
const MAX_SHOWN_CHARS: usize = 12_000;
const MAX_LINE_CHARS: usize = 1_000;

pub(crate) struct LogCapture {
    pub(crate) receipt: Value,
    pub(crate) lines: Vec<(u64, String)>,
}

pub(crate) async fn wait_for_ready(
    child: &mut Child,
    log_path: &Path,
    from_byte: u64,
    timeout_ms: u64,
) -> Result<(bool, bool, bool, u64), String> {
    let started = std::time::Instant::now();
    let deadline = started + std::time::Duration::from_millis(timeout_ms);
    loop {
        let process_exited = child
            .try_wait()
            .map_err(|err| format!("runtime session wait failed: {err}"))?
            .is_some();
        let startup = startup_markers(log_path, from_byte).await;
        if (startup.ready_seen && startup.orientation_seen)
            || process_exited
            || std::time::Instant::now() >= deadline
        {
            return Ok((
                startup.ready_seen,
                startup.orientation_seen,
                process_exited,
                started.elapsed().as_millis().min(u128::from(u64::MAX)) as u64,
            ));
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

pub(crate) async fn capture_update(
    session_id: &str,
    log_path: &Path,
    mode: &str,
    cursor: &mut RuntimeLogCursor,
) -> LogCapture {
    let mut receipt = json!({
        "available": false,
        "mode": mode,
        "session_id": session_id,
        "log_path": log_path.to_string_lossy(),
    });

    let Ok(metadata) = tokio::fs::metadata(log_path).await else {
        receipt["error"] = json!("Could not stat log file.");
        return LogCapture {
            receipt,
            lines: vec![],
        };
    };
    let file_size = metadata.len();
    let log_reset = cursor.byte_offset > file_size;
    if log_reset {
        *cursor = RuntimeLogCursor::default();
    }

    let cursor_before = cursor.clone();
    let Ok(bytes) = read_from(log_path, cursor_before.byte_offset).await else {
        receipt["error"] = json!("Could not open or read log file.");
        return LogCapture {
            receipt,
            lines: vec![],
        };
    };

    let complete_len = complete_line_prefix_len(&bytes);
    let pending_bytes = bytes.len().saturating_sub(complete_len);
    let complete_bytes = &bytes[..complete_len];
    let text = String::from_utf8_lossy(complete_bytes);
    let raw_lines: Vec<String> = text.lines().map(trim_cr).collect();
    let first_line = if raw_lines.is_empty() {
        0
    } else {
        cursor_before.line + 1
    };
    let lines: Vec<(u64, String)> = raw_lines
        .iter()
        .enumerate()
        .map(|(index, line)| (cursor_before.line + index as u64 + 1, line.clone()))
        .collect();

    let ready_line = lines
        .iter()
        .find_map(|(line_no, line)| line.contains(READY_MARKER).then_some(*line_no))
        .unwrap_or(0);
    let orientation_line = lines
        .iter()
        .find_map(|(line_no, line)| {
            line.contains(STARTUP_ORIENTATION_MARKER)
                .then_some(*line_no)
        })
        .unwrap_or(0);
    let (shown_lines, omitted, truncated) = shown_excerpt(&lines);
    let shown_ranges = line_ranges(&shown_lines);

    cursor.byte_offset = cursor_before.byte_offset + complete_len as u64;
    cursor.line = cursor_before.line + raw_lines.len() as u64;

    receipt = json!({
        "available": true,
        "mode": mode,
        "session_id": session_id,
        "log_path": log_path.to_string_lossy(),
        "log_reset": log_reset,
        "cursor_before_line": cursor_before.line,
        "cursor_after_line": cursor.line,
        "byte_offset_before": cursor_before.byte_offset,
        "byte_offset_after": cursor.byte_offset,
        "bytes_added": complete_len,
        "pending_partial_bytes": pending_bytes,
        "lines_added": raw_lines.len(),
        "shown_line_count": shown_lines.len(),
        "omitted_line_count": omitted,
        "truncated_line_count": truncated,
        "first_line": first_line,
        "last_line": if raw_lines.is_empty() { 0 } else { cursor.line },
        "shown_first_line": shown_lines.first().map(|line| line.0).unwrap_or(0),
        "shown_last_line": shown_lines.last().map(|line| line.0).unwrap_or(0),
        "shown_ranges": shown_ranges,
        "line_limit": MAX_SHOWN_LINES,
        "char_limit": MAX_SHOWN_CHARS,
        "line_char_limit": MAX_LINE_CHARS,
        "runtime_session_ready_seen": ready_line > 0,
        "runtime_session_ready_line": ready_line,
        "runtime_session_orientation_seen": orientation_line > 0,
        "runtime_session_orientation_line": orientation_line,
        "lines": shown_lines.into_iter().map(|(_, line)| line).collect::<Vec<_>>(),
    });
    LogCapture { receipt, lines }
}

pub(crate) async fn capture_from_offset(
    session_id: &str,
    log_path: &Path,
    mode: &str,
    byte_offset: u64,
) -> LogCapture {
    let mut cursor = RuntimeLogCursor {
        byte_offset,
        line: 0,
    };
    capture_update(session_id, log_path, mode, &mut cursor).await
}

pub(crate) fn findings_for_script(lines: &[(u64, String)], script_run_id: &str) -> Value {
    let mut blocks: Vec<Value> = Vec::new();
    let mut current: Vec<String> = Vec::new();
    let mut in_slice = false;

    for (_, line) in lines {
        if !in_slice {
            if line.starts_with("FENNARA_SCRIPT_STARTED:") && line.contains(script_run_id) {
                in_slice = true;
            }
            continue;
        }
        if (line.starts_with("FENNARA_SCRIPT_COMPLETED:")
            || line.starts_with("FENNARA_SCRIPT_FAILED:"))
            && line.contains(script_run_id)
        {
            flush_block(&mut blocks, &mut current);
            break;
        }
        if is_issue_start(line) {
            flush_block(&mut blocks, &mut current);
            current.push(line.clone());
        } else if !current.is_empty() && (is_issue_continuation(line) || line.trim().is_empty()) {
            current.push(line.clone());
        } else {
            flush_block(&mut blocks, &mut current);
        }
    }
    flush_block(&mut blocks, &mut current);

    let warning_count = blocks
        .iter()
        .filter(|block| block["kind"] == "warning")
        .count();
    let crash_count = blocks
        .iter()
        .filter(|block| block["kind"] == "crash")
        .count();
    let error_count = blocks
        .iter()
        .filter(|block| block["kind"] == "error" || block["kind"] == "crash")
        .count();
    let compacted = blocks
        .iter()
        .take(12)
        .filter_map(|block| block["text"].as_str())
        .collect::<Vec<_>>()
        .join("\n\n");

    json!({
        "log_available": true,
        "has_findings": !blocks.is_empty(),
        "error_count": error_count,
        "warning_count": warning_count,
        "crash_count": crash_count,
        "blocks": blocks,
        "compacted": compacted,
    })
}

async fn read_from(path: &Path, offset: u64) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(path).await?;
    file.seek(SeekFrom::Start(offset)).await?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).await?;
    Ok(bytes)
}

#[derive(Default)]
struct StartupMarkers {
    ready_seen: bool,
    orientation_seen: bool,
}

async fn startup_markers(path: &Path, offset: u64) -> StartupMarkers {
    read_from(path, offset)
        .await
        .ok()
        .map(|bytes| {
            let complete_len = complete_line_prefix_len(&bytes);
            String::from_utf8_lossy(&bytes[..complete_len]).to_string()
        })
        .map(|text| startup_markers_from_text(&text))
        .unwrap_or_default()
}

fn startup_markers_from_text(text: &str) -> StartupMarkers {
    let Some(ready_index) = text.find(READY_MARKER) else {
        return StartupMarkers::default();
    };
    StartupMarkers {
        ready_seen: true,
        orientation_seen: text[ready_index..].contains(STARTUP_ORIENTATION_MARKER),
    }
}

fn shown_excerpt(lines: &[(u64, String)]) -> (Vec<(u64, String)>, usize, usize) {
    let tail_lines = MAX_SHOWN_LINES.saturating_sub(HEAD_LINES);
    let mut shown = Vec::new();
    let mut chars = 0usize;
    let mut truncated = 0usize;
    if lines.len() <= MAX_SHOWN_LINES {
        let selected: Vec<(u64, String)> = lines.iter().cloned().collect();
        for (line_no, line) in selected {
            push_shown_line(line_no, line, &mut shown, &mut chars, &mut truncated);
        }
    } else {
        for (line_no, line) in lines.iter().skip(lines.len() - tail_lines).cloned() {
            push_shown_line(line_no, line, &mut shown, &mut chars, &mut truncated);
        }
        for (line_no, line) in lines.iter().take(HEAD_LINES).cloned() {
            push_shown_line(line_no, line, &mut shown, &mut chars, &mut truncated);
        }
        shown.sort_by_key(|(line_no, _)| *line_no);
    }

    let omitted = lines.len().saturating_sub(shown.len());
    (shown, omitted, truncated)
}

fn push_shown_line(
    line_no: u64,
    line: String,
    shown: &mut Vec<(u64, String)>,
    chars: &mut usize,
    truncated: &mut usize,
) {
    let (line, was_truncated) = truncate_line(line);
    if was_truncated {
        *truncated += 1;
    }
    let next_chars = *chars + line.len() + 1;
    if *chars > 0 && next_chars > MAX_SHOWN_CHARS {
        return;
    }
    *chars = next_chars;
    shown.push((line_no, line));
}

fn complete_line_prefix_len(bytes: &[u8]) -> usize {
    if bytes.is_empty() {
        return 0;
    }
    if matches!(bytes.last(), Some(b'\n')) {
        return bytes.len();
    }
    bytes
        .iter()
        .rposition(|byte| *byte == b'\n')
        .map(|index| index + 1)
        .unwrap_or(0)
}

fn line_ranges(lines: &[(u64, String)]) -> Vec<Value> {
    let mut ranges = Vec::new();
    let mut start = 0u64;
    let mut previous = 0u64;
    for (line_no, _) in lines {
        if start == 0 {
            start = *line_no;
            previous = *line_no;
        } else if *line_no == previous + 1 {
            previous = *line_no;
        } else {
            ranges.push(json!({ "first": start, "last": previous }));
            start = *line_no;
            previous = *line_no;
        }
    }
    if start > 0 {
        ranges.push(json!({ "first": start, "last": previous }));
    }
    ranges
}

fn trim_cr(line: &str) -> String {
    line.strip_suffix('\r').unwrap_or(line).to_string()
}

fn truncate_line(line: String) -> (String, bool) {
    if line.chars().count() <= MAX_LINE_CHARS {
        return (line, false);
    }
    (
        line.chars().take(MAX_LINE_CHARS).collect::<String>() + " ... [line truncated]",
        true,
    )
}

fn flush_block(blocks: &mut Vec<Value>, current: &mut Vec<String>) {
    if current.is_empty() {
        return;
    }
    let kind = issue_kind(current);
    blocks.push(json!({
        "kind": kind,
        "text": current.join("\n"),
    }));
    current.clear();
}

fn issue_kind(block: &[String]) -> &'static str {
    let Some(first) = block.first() else {
        return "log";
    };
    if first.starts_with("WARNING:") {
        "warning"
    } else if first.contains("CrashHandlerException")
        || first.contains("Program crashed with signal")
    {
        "crash"
    } else {
        "error"
    }
}

fn is_issue_start(line: &str) -> bool {
    line.starts_with("ERROR:")
        || line.starts_with("SCRIPT ERROR:")
        || line.starts_with("WARNING:")
        || line.contains("CrashHandlerException")
        || line.contains("Program crashed with signal")
}

fn is_issue_continuation(line: &str) -> bool {
    line.starts_with("   at:")
        || line.starts_with("          at:")
        || line.starts_with("   GDScript backtrace")
        || line.starts_with("          GDScript backtrace")
        || line.starts_with("       [")
        || line.starts_with("              [")
        || line.starts_with('[')
        || line.starts_with("-- END OF")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn numbered_lines(count: u64) -> Vec<(u64, String)> {
        (1..=count)
            .map(|line| (line, format!("line {line}")))
            .collect()
    }

    #[test]
    fn shown_excerpt_keeps_all_lines_when_under_limit() {
        let lines = numbered_lines(3);

        let (shown, omitted, truncated) = shown_excerpt(&lines);

        assert_eq!(shown, lines);
        assert_eq!(omitted, 0);
        assert_eq!(truncated, 0);
    }

    #[test]
    fn shown_excerpt_uses_head_and_tail_when_over_limit() {
        let lines = numbered_lines(65);

        let (shown, omitted, truncated) = shown_excerpt(&lines);

        assert_eq!(shown.len(), MAX_SHOWN_LINES);
        assert_eq!(shown.first().map(|line| line.0), Some(1));
        assert_eq!(shown.get(HEAD_LINES - 1).map(|line| line.0), Some(20));
        assert_eq!(shown.get(HEAD_LINES).map(|line| line.0), Some(26));
        assert_eq!(shown.last().map(|line| line.0), Some(65));
        assert_eq!(omitted, 5);
        assert_eq!(truncated, 0);
    }

    #[test]
    fn shown_excerpt_reports_line_and_character_truncation() {
        let lines: Vec<(u64, String)> = (1..=20)
            .map(|line| (line, "x".repeat(MAX_LINE_CHARS)))
            .collect();

        let (shown, omitted, truncated) = shown_excerpt(&lines);

        assert!(shown.len() < lines.len());
        assert_eq!(omitted, lines.len() - shown.len());
        assert_eq!(truncated, 0);
    }

    #[test]
    fn line_ranges_groups_contiguous_line_numbers() {
        let lines = vec![
            (2, "a".to_string()),
            (3, "b".to_string()),
            (5, "c".to_string()),
            (8, "d".to_string()),
            (9, "e".to_string()),
        ];

        assert_eq!(
            line_ranges(&lines),
            vec![
                json!({ "first": 2, "last": 3 }),
                json!({ "first": 5, "last": 5 }),
                json!({ "first": 8, "last": 9 }),
            ]
        );
    }

    #[test]
    fn complete_line_prefix_len_excludes_partial_tail() {
        assert_eq!(complete_line_prefix_len(b""), 0);
        assert_eq!(complete_line_prefix_len(b"partial"), 0);
        assert_eq!(complete_line_prefix_len(b"one\npartial"), 4);
        assert_eq!(complete_line_prefix_len(b"one\r\ntwo\n"), 9);
    }

    #[test]
    fn truncate_line_preserves_short_lines_and_marks_long_lines() {
        let (short, short_truncated) = truncate_line("short".to_string());
        assert_eq!(short, "short");
        assert!(!short_truncated);

        let (long, long_truncated) = truncate_line("z".repeat(MAX_LINE_CHARS + 1));
        assert!(long_truncated);
        assert_eq!(long.chars().take(MAX_LINE_CHARS).count(), MAX_LINE_CHARS);
        assert!(long.ends_with(" ... [line truncated]"));
    }

    #[test]
    fn findings_for_script_extracts_errors_warnings_and_crashes_inside_script_slice() {
        let lines = vec![
            (1, "ERROR: before".to_string()),
            (2, "FENNARA_SCRIPT_STARTED: run-1".to_string()),
            (3, "WARNING: watch this".to_string()),
            (4, "   at: res://foo.gd:10".to_string()),
            (5, "ERROR: broken".to_string()),
            (6, "CrashHandlerException: crashed".to_string()),
            (7, "FENNARA_SCRIPT_COMPLETED: run-1".to_string()),
            (8, "ERROR: after".to_string()),
        ];

        let findings = findings_for_script(&lines, "run-1");

        assert_eq!(findings["log_available"], json!(true));
        assert_eq!(findings["has_findings"], json!(true));
        assert_eq!(findings["warning_count"], json!(1));
        assert_eq!(findings["error_count"], json!(2));
        assert_eq!(findings["crash_count"], json!(1));
        assert!(
            findings["compacted"]
                .as_str()
                .unwrap()
                .contains("WARNING: watch this")
        );
        assert!(!findings["compacted"].as_str().unwrap().contains("before"));
        assert!(!findings["compacted"].as_str().unwrap().contains("after"));
    }
}
