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
    for (path, replacement) in replacements {
        sanitized = sanitized.replace(&path, replacement);
    }
    sanitized = redact_url_queries(&sanitized);
    sanitized = redact_bearer_tokens(&sanitized);
    redact_secret_assignments(&sanitized)
}

fn redact_url_queries(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            if (word.starts_with("http://") || word.starts_with("https://"))
                && let Some(index) = word.find('?')
            {
                return format!("{}?<redacted>", &word[..index]);
            }
            word.to_string()
        })
        .collect::<Vec<_>>()
        .join(" ")
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
