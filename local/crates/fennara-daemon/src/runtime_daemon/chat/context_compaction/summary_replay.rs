use super::{
    summary::{clean_summary_chain, select_summary_chunks_for_replay},
    types::{ContextSummaryChunk, ReplayGroup, ReplayRow},
};

pub(crate) fn apply_summary_replay(
    groups: Vec<ReplayGroup>,
    summaries: &[ContextSummaryChunk],
    summary_replay_budget_tokens: usize,
) -> Vec<ReplayGroup> {
    let clean_chain = clean_summary_chain(&groups, summaries);
    let selected = select_summary_chunks_for_replay(&clean_chain, summary_replay_budget_tokens);
    let Some(latest_summary) = selected.last() else {
        return groups;
    };
    let latest_covered_sequence = latest_summary.covered_end_sequence;

    let mut replay_groups = vec![
        ReplayGroup::new(vec![synthetic_user_summary_question()]),
        ReplayGroup::new(vec![synthetic_assistant_summary(&selected)]),
    ];
    replay_groups.extend(
        groups
            .into_iter()
            .filter(|group| group_start_sequence(group) > latest_covered_sequence),
    );
    replay_groups
}

fn synthetic_user_summary_question() -> ReplayRow {
    ReplayRow {
        id: "synthetic_context_summary_user".to_string(),
        sequence: 0,
        role: "user".to_string(),
        status: "done".to_string(),
        content: "What did we do so far?".to_string(),
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

fn synthetic_assistant_summary(chunks: &[&ContextSummaryChunk]) -> ReplayRow {
    ReplayRow {
        id: "synthetic_context_summary_assistant".to_string(),
        sequence: 0,
        role: "assistant".to_string(),
        status: "done".to_string(),
        content: render_summary_chunks(chunks),
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

fn render_summary_chunks(chunks: &[&ContextSummaryChunk]) -> String {
    let mut lines = vec!["# Earlier Conversation Summary".to_string()];
    for chunk in chunks {
        lines.push(String::new());
        lines.push(format!(
            "## Messages {}-{}",
            chunk.covered_start_sequence, chunk.covered_end_sequence
        ));
        lines.push(chunk.summary_markdown.trim().to_string());
    }
    lines.join("\n")
}

fn group_start_sequence(group: &ReplayGroup) -> i64 {
    group
        .rows
        .first()
        .map(|row| row.sequence)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(sequence: i64, role: &str, content: &str) -> ReplayRow {
        ReplayRow {
            id: format!("msg_{sequence}"),
            sequence,
            role: role.to_string(),
            status: "done".to_string(),
            content: content.to_string(),
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

    fn group(sequence: i64) -> ReplayGroup {
        ReplayGroup::new(vec![row(sequence, "user", &format!("message {sequence}"))])
    }

    fn summary(start: i64, end: i64, text: &str) -> ContextSummaryChunk {
        ContextSummaryChunk {
            id: format!("sum_{start}_{end}"),
            chat_id: "chat_1".to_string(),
            generation_id: None,
            summary_markdown: text.to_string(),
            covered_start_message_id: Some(format!("msg_{start}")),
            covered_start_sequence: start,
            covered_end_message_id: format!("msg_{end}"),
            covered_end_sequence: end,
            tail_start_message_id: Some(format!("msg_{}", end + 1)),
            tail_start_sequence: Some(end + 1),
            source_message_count: end - start + 1,
            model: None,
            provider_id: None,
            model_id: None,
            model_variant: None,
            model_ref_json: None,
            metadata_json: None,
            created_at_ms: start,
        }
    }

    #[test]
    fn injects_synthetic_summary_then_exact_tail() {
        let groups = vec![group(1), group(2), group(3)];
        let summaries = vec![summary(1, 2, "did old work")];

        let replay = apply_summary_replay(groups, &summaries, 64_000);

        assert_eq!(replay[0].rows[0].role, "user");
        assert_eq!(replay[0].rows[0].content, "What did we do so far?");
        assert_eq!(replay[1].rows[0].role, "assistant");
        assert!(replay[1].rows[0].content.contains("did old work"));
        assert_eq!(replay[2].rows[0].sequence, 3);
    }

    #[test]
    fn ignores_corrupt_gap_chain_and_keeps_exact_history() {
        let groups = vec![group(1), group(2), group(3)];
        let summaries = vec![summary(2, 2, "gap")];

        let replay = apply_summary_replay(groups.clone(), &summaries, 64_000);

        assert_eq!(replay.len(), groups.len());
        assert_eq!(replay[0].rows[0].sequence, 1);
    }
}
