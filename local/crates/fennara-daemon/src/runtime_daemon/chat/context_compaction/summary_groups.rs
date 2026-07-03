use serde_json::Value;

use super::types::{ReplayGroup, ReplayRow};

pub(crate) fn group_raw_summary_rows(rows: &[ReplayRow]) -> Vec<ReplayGroup> {
    let mut groups = Vec::new();
    let mut index = 0;
    while index < rows.len() {
        let row = &rows[index];
        if row.role == "assistant" && has_tool_calls(row.tool_calls_json.as_deref()) {
            let mut group = vec![row.clone()];
            let mut next = index + 1;
            while next < rows.len() && rows[next].role == "tool" {
                group.push(rows[next].clone());
                next += 1;
            }
            groups.push(ReplayGroup::new(group));
            index = next;
            continue;
        }

        groups.push(ReplayGroup::new(vec![row.clone()]));
        index += 1;
    }
    groups
}

fn has_tool_calls(tool_calls_json: Option<&str>) -> bool {
    let Some(tool_calls_json) = tool_calls_json
        .map(str::trim)
        .filter(|value| !value.is_empty())
    else {
        return false;
    };
    match serde_json::from_str::<Value>(tool_calls_json) {
        Ok(Value::Array(values)) => !values.is_empty(),
        Ok(_) => false,
        Err(_) => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(sequence: i64, role: &str) -> ReplayRow {
        ReplayRow {
            id: format!("msg_{sequence}"),
            sequence,
            role: role.to_string(),
            status: "done".to_string(),
            content: format!("content {sequence}"),
            tool_call_id: None,
            tool_name: None,
            tool_calls_json: None,
            metadata_json: None,
            raw_result_json: None,
            arguments_json: None,
            target_keys_json: None,
            tool_status: None,
        }
    }

    #[test]
    fn groups_assistant_tool_calls_with_following_raw_tool_rows() {
        let mut assistant = row(2, "assistant");
        assistant.tool_calls_json = Some(
            r#"[{"id":"call_1","type":"function","function":{"name":"read_file","arguments":"{}"}}]"#
                .to_string(),
        );
        let rows = vec![row(1, "user"), assistant, row(3, "tool"), row(4, "user")];

        let groups = group_raw_summary_rows(&rows);

        assert_eq!(groups.len(), 3);
        assert_eq!(groups[0].rows[0].sequence, 1);
        assert_eq!(
            groups[1]
                .rows
                .iter()
                .map(|row| row.sequence)
                .collect::<Vec<_>>(),
            vec![2, 3]
        );
        assert_eq!(groups[2].rows[0].sequence, 4);
    }
}
