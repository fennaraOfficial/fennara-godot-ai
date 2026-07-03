use std::collections::HashSet;

use super::types::{ReplayGroup, ReplayRow};

pub(crate) fn sanitize_replay_groups(rows: &[ReplayRow]) -> Vec<ReplayGroup> {
    let mut groups = Vec::new();
    let mut index = 0;
    while index < rows.len() {
        let row = &rows[index];
        if row.role == "tool" {
            index += 1;
            continue;
        }

        if row.role == "assistant" {
            if let Some(required_tool_ids) = required_tool_call_ids(row.tool_calls_json.as_deref())
            {
                if required_tool_ids.is_empty() {
                    index += 1;
                    continue;
                }

                let mut group = vec![row.clone()];
                let mut seen_tool_ids = HashSet::new();
                let mut next = index + 1;
                while next < rows.len() && rows[next].role == "tool" {
                    if let Some(tool_call_id) = rows[next].tool_call_id.as_deref() {
                        if required_tool_ids.contains(tool_call_id) {
                            seen_tool_ids.insert(tool_call_id.to_string());
                            group.push(rows[next].clone());
                        }
                    }
                    next += 1;
                }

                if seen_tool_ids == required_tool_ids {
                    groups.push(ReplayGroup::new(group));
                }
                index = next;
                continue;
            }
        }

        groups.push(ReplayGroup::new(vec![row.clone()]));
        index += 1;
    }

    groups
}

fn required_tool_call_ids(tool_calls_json: Option<&str>) -> Option<HashSet<String>> {
    let tool_calls_json = tool_calls_json?;
    let Ok(tool_calls) = serde_json::from_str::<serde_json::Value>(tool_calls_json) else {
        return None;
    };
    let Some(tool_calls) = tool_calls.as_array() else {
        return None;
    };
    if tool_calls.is_empty() {
        return None;
    }

    let mut ids = HashSet::new();
    for tool_call in tool_calls {
        let Some(id) = tool_call
            .get("id")
            .and_then(serde_json::Value::as_str)
            .map(str::trim)
            .filter(|id| !id.is_empty())
        else {
            return Some(HashSet::new());
        };
        ids.insert(id.to_string());
    }

    Some(ids)
}
