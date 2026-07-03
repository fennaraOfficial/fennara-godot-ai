use super::types::ReplayGroup;

pub(crate) const DEFAULT_LATEST_EXACT_USER_TURNS: usize = 2;

pub(crate) fn protected_groups(groups: &[ReplayGroup], latest_user_turns: usize) -> Vec<bool> {
    if groups.is_empty() {
        return Vec::new();
    }
    let mut protected = vec![false; groups.len()];
    let start = protected_tail_start(groups, latest_user_turns);
    for index in start..groups.len() {
        protected[index] = true;
    }
    for (index, group) in groups.iter().enumerate() {
        if group.has_incomplete_tool_state() {
            protected[index] = true;
        }
    }
    protected
}

fn protected_tail_start(groups: &[ReplayGroup], latest_user_turns: usize) -> usize {
    if latest_user_turns == 0 {
        return groups.len();
    }

    let mut seen_user_turns = 0usize;
    for (index, group) in groups.iter().enumerate().rev() {
        if group.starts_user_turn() {
            seen_user_turns += 1;
            if seen_user_turns == latest_user_turns {
                return index;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_daemon::chat::context_compaction::types::ReplayRow;

    fn row(role: &str) -> ReplayRow {
        ReplayRow {
            id: format!("msg_{role}"),
            sequence: 1,
            role: role.to_string(),
            status: "done".to_string(),
            content: String::new(),
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
    fn protects_latest_two_user_turns_and_followups() {
        let groups = vec![
            ReplayGroup::new(vec![row("user")]),
            ReplayGroup::new(vec![row("assistant")]),
            ReplayGroup::new(vec![row("user")]),
            ReplayGroup::new(vec![row("assistant")]),
            ReplayGroup::new(vec![row("user")]),
            ReplayGroup::new(vec![row("assistant")]),
        ];

        assert_eq!(
            protected_groups(&groups, DEFAULT_LATEST_EXACT_USER_TURNS),
            vec![false, false, true, true, true, true]
        );
    }
}
