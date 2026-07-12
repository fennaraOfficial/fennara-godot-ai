use std::path::PathBuf;

pub(super) fn sanitize_text<'a>(
    text: &str,
    replacements: impl Iterator<Item = (&'a PathBuf, &'a str)>,
) -> String {
    let mut sanitized = text.to_string();
    let mut replacements: Vec<_> = replacements
        .flat_map(|(path, replacement)| {
            let native = path.display().to_string();
            let slash = native.replace('\\', "/");
            [(native, replacement), (slash, replacement)]
        })
        .filter(|(path, _)| !path.is_empty())
        .collect();
    replacements.sort_by(|left, right| right.0.len().cmp(&left.0.len()));
    let case_insensitive_paths = cfg!(windows) || cfg!(target_os = "macos");
    for (path, replacement) in replacements {
        sanitized = replace_path(&sanitized, &path, replacement, case_insensitive_paths);
    }
    sanitized = redact_url_queries(&sanitized);
    sanitized = redact_bearer_tokens(&sanitized);
    redact_secret_assignments(&sanitized)
}

fn redact_url_queries(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    let lower = text.to_ascii_lowercase();
    let mut cursor = 0;
    while cursor < text.len() {
        let http = lower[cursor..]
            .find("http://")
            .map(|offset| cursor + offset);
        let https = lower[cursor..]
            .find("https://")
            .map(|offset| cursor + offset);
        let Some(url_start) = http.into_iter().chain(https).min() else {
            output.push_str(&text[cursor..]);
            break;
        };
        output.push_str(&text[cursor..url_start]);
        let url_end = text[url_start..]
            .find(char::is_whitespace)
            .map(|offset| url_start + offset)
            .unwrap_or(text.len());
        let url = &text[url_start..url_end];
        if let Some(query_start) = url.find('?') {
            output.push_str(&url[..=query_start]);
            output.push_str("<redacted>");
        } else {
            output.push_str(url);
        }
        cursor = url_end;
    }
    output
}

pub(super) fn replace_path(
    text: &str,
    path: &str,
    replacement: &str,
    case_insensitive: bool,
) -> String {
    if !case_insensitive {
        return text.replace(path, replacement);
    }
    let lower_text = text.to_ascii_lowercase();
    let lower_path = path.to_ascii_lowercase();
    let mut output = String::with_capacity(text.len());
    let mut cursor = 0;
    while let Some(offset) = lower_text[cursor..].find(&lower_path) {
        let start = cursor + offset;
        output.push_str(&text[cursor..start]);
        output.push_str(replacement);
        cursor = start + path.len();
    }
    output.push_str(&text[cursor..]);
    output
}

fn redact_bearer_tokens(text: &str) -> String {
    let mut output = text.to_string();
    let mut search_from = 0;
    let marker = "bearer ";
    loop {
        let lower = output.to_ascii_lowercase();
        let Some(offset) = lower[search_from..].find(marker) else {
            break;
        };
        let start = search_from + offset;
        let value_start = start + marker.len();
        let value_end = output[value_start..]
            .find(char::is_whitespace)
            .map(|value_offset| value_start + value_offset)
            .unwrap_or(output.len());
        output.replace_range(start..value_end, "Bearer <redacted>");
        search_from = start + "Bearer <redacted>".len();
        if search_from >= output.len() {
            break;
        }
    }
    output
}

fn redact_secret_assignments(text: &str) -> String {
    let mut output = redact_assignment(text.to_string(), "authorization", true);
    for key in [
        "api_key",
        "apikey",
        "access_token",
        "auth_token",
        "password",
        "secret",
    ] {
        output = redact_assignment(output, key, false);
    }
    output
}

fn redact_assignment(mut output: String, key: &str, include_spaces: bool) -> String {
    let mut search_from = 0;
    loop {
        let lower = output.to_ascii_lowercase();
        let Some(key_offset) = lower[search_from..].find(key) else {
            break;
        };
        let search_start = search_from + key_offset + key.len();
        let Some(separator_offset) = output[search_start..].find(['=', ':']) else {
            break;
        };
        let raw_value_start = search_start + separator_offset + 1;
        let value_start = output[raw_value_start..]
            .char_indices()
            .find(|(_, ch)| !ch.is_whitespace())
            .map(|(offset, _)| raw_value_start + offset)
            .unwrap_or(output.len());
        if value_start == output.len() {
            break;
        }

        let first = output[value_start..].chars().next().unwrap_or_default();
        let (replace_start, value_end) = if first == '"' || first == '\'' {
            let content_start = value_start + first.len_utf8();
            let content_end = output[content_start..]
                .find(first)
                .map(|offset| content_start + offset)
                .unwrap_or(output.len());
            (content_start, content_end)
        } else {
            let end = output[value_start..]
                .find(|ch: char| {
                    ch == ','
                        || ch == ';'
                        || ch == '\r'
                        || ch == '\n'
                        || (!include_spaces && ch.is_whitespace())
                })
                .map(|offset| value_start + offset)
                .unwrap_or(output.len());
            (value_start, end)
        };
        output.replace_range(replace_start..value_end, "<redacted>");
        search_from = replace_start + "<redacted>".len();
        if search_from >= output.len() {
            break;
        }
    }
    output
}
