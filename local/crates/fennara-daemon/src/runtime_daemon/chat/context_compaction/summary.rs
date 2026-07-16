use super::{
    tokens::estimate_text_tokens,
    types::{ContextSummaryChunk, ReplayGroup, ReplayRow},
};

pub(crate) const SUMMARY_OUTPUT_MAX_TOKENS: u32 = 4_096;
const UNKNOWN_CONTEXT_FALLBACK_TOKENS: u32 = 64_000;
const UNKNOWN_LOCAL_CONTEXT_FALLBACK_TOKENS: u32 = 8_192;
const LARGE_CONTEXT_THRESHOLD: usize = 400_000;
const LARGE_CONTEXT_WORKING_BUDGET: usize = 450_000;
const LARGE_CONTEXT_TAIL_BUDGET_MAX: usize = 100_000;
const SUMMARY_REPLAY_BUDGET_MIN: usize = 512;
pub(crate) const SUMMARY_REPLAY_BUDGET_MAX: usize = 64_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SummaryBudgets {
    pub(crate) provider_usable_input_tokens: usize,
    pub(crate) raw_context_tokens: usize,
    pub(crate) compaction_working_budget: usize,
    pub(crate) summary_trigger_tokens: usize,
    pub(crate) tail_budget_tokens: usize,
    pub(crate) summary_replay_budget_tokens: usize,
    model_output_max_tokens: Option<u32>,
}

impl SummaryBudgets {
    pub(crate) fn from_model_context(
        provider_usable_input_tokens: u32,
        raw_context_tokens: Option<u32>,
    ) -> Self {
        let provider_usable_input_tokens = provider_usable_input_tokens as usize;
        let raw_context_tokens = raw_context_tokens
            .map(|tokens| tokens as usize)
            .unwrap_or(provider_usable_input_tokens);
        let compaction_working_budget = if raw_context_tokens > LARGE_CONTEXT_THRESHOLD {
            provider_usable_input_tokens.min(LARGE_CONTEXT_WORKING_BUDGET)
        } else {
            provider_usable_input_tokens
        };
        let summary_trigger_tokens = ((compaction_working_budget as f64) * 0.9).floor() as usize;
        let summary_trigger_tokens = summary_trigger_tokens.min(LARGE_CONTEXT_THRESHOLD);
        let mut tail_budget_tokens = compaction_working_budget / 4;
        if raw_context_tokens > LARGE_CONTEXT_THRESHOLD {
            tail_budget_tokens = tail_budget_tokens.min(LARGE_CONTEXT_TAIL_BUDGET_MAX);
        }
        let summary_replay_budget_tokens =
            ((compaction_working_budget as f64) * 0.15).floor() as usize;
        let summary_replay_budget_tokens = summary_replay_budget_tokens
            .clamp(SUMMARY_REPLAY_BUDGET_MIN, SUMMARY_REPLAY_BUDGET_MAX);

        Self {
            provider_usable_input_tokens,
            raw_context_tokens,
            compaction_working_budget,
            summary_trigger_tokens,
            tail_budget_tokens,
            summary_replay_budget_tokens,
            model_output_max_tokens: None,
        }
    }

    pub(crate) fn with_model_output_limit(mut self, max_output_tokens: Option<u32>) -> Self {
        self.model_output_max_tokens = max_output_tokens;
        self
    }

    pub(crate) fn for_unknown_context() -> Self {
        Self::from_model_context(
            UNKNOWN_CONTEXT_FALLBACK_TOKENS,
            Some(UNKNOWN_CONTEXT_FALLBACK_TOKENS),
        )
    }

    pub(crate) fn for_unknown_local_context() -> Self {
        Self::from_model_context(
            UNKNOWN_LOCAL_CONTEXT_FALLBACK_TOKENS,
            Some(UNKNOWN_LOCAL_CONTEXT_FALLBACK_TOKENS),
        )
    }

    pub(crate) fn unknown_context_fallback_tokens() -> u32 {
        UNKNOWN_CONTEXT_FALLBACK_TOKENS
    }

    pub(crate) fn unknown_local_context_fallback_tokens() -> u32 {
        UNKNOWN_LOCAL_CONTEXT_FALLBACK_TOKENS
    }

    pub(crate) fn summary_output_max_tokens(self) -> u32 {
        let budget = match self.raw_context_tokens {
            0..=8_192 => 512,
            8_193..=16_384 => 1_024,
            16_385..=65_536 => 2_048,
            _ => SUMMARY_OUTPUT_MAX_TOKENS,
        };
        self.model_output_max_tokens
            .map_or(budget, |limit| budget.min(limit))
    }
}

#[derive(Clone, Debug)]
pub(crate) struct SummaryCandidate {
    pub(crate) groups: Vec<ReplayGroup>,
    pub(crate) previous_summary_markdown: Option<String>,
    pub(crate) covered_start_message_id: String,
    pub(crate) covered_start_sequence: i64,
    pub(crate) covered_end_message_id: String,
    pub(crate) covered_end_sequence: i64,
    pub(crate) tail_start_message_id: Option<String>,
    pub(crate) tail_start_sequence: Option<i64>,
    pub(crate) source_message_count: i64,
}

pub(crate) fn select_next_summary_candidate(
    groups: &[ReplayGroup],
    summaries: &[ContextSummaryChunk],
    tail_budget_tokens: usize,
) -> Option<SummaryCandidate> {
    if groups.is_empty() {
        return None;
    }
    let tail_start = exact_tail_start_index(groups, tail_budget_tokens);
    if tail_start == 0 {
        return None;
    }

    let clean_chain = clean_summary_chain(groups, summaries);
    let latest_covered = clean_chain
        .last()
        .map(|summary| summary.covered_end_sequence);
    let start_index = if let Some(latest_covered) = latest_covered {
        let overlapping = groups.iter().take(tail_start).any(|group| {
            group_start_sequence(group) <= latest_covered
                && latest_covered < group_end_sequence(group)
        });
        if overlapping {
            return None;
        }
        groups
            .iter()
            .take(tail_start)
            .position(|group| group_start_sequence(group) > latest_covered)?
    } else {
        0
    };
    if start_index >= tail_start {
        return None;
    }

    let selected_groups = groups[start_index..tail_start].to_vec();
    let first = first_row(selected_groups.first()?)?;
    let last = last_row(selected_groups.last()?)?;
    let tail_start_row = groups.get(tail_start).and_then(first_row);
    let previous_summary_markdown =
        (!clean_chain.is_empty()).then(|| render_previous_summary_markdown(&clean_chain));
    let covered_start_sequence = clean_chain
        .first()
        .map(|summary| summary.covered_start_sequence)
        .unwrap_or(first.sequence);
    let covered_start_message_id = clean_chain
        .first()
        .and_then(|summary| summary.covered_start_message_id.clone())
        .or_else(|| row_id_for_sequence(groups, covered_start_sequence))
        .unwrap_or_else(|| first.id.clone());
    let covered_end_message_id = last.id.clone();
    let covered_end_sequence = last.sequence;
    let source_message_count =
        count_rows_in_range(groups, covered_start_sequence, covered_end_sequence);
    let tail_start_message_id = tail_start_row.map(|row| row.id.clone());
    let tail_start_sequence = tail_start_row.map(|row| row.sequence);

    Some(SummaryCandidate {
        groups: selected_groups,
        previous_summary_markdown,
        covered_start_message_id,
        covered_start_sequence,
        covered_end_message_id,
        covered_end_sequence,
        tail_start_message_id,
        tail_start_sequence,
        source_message_count,
    })
}

pub(crate) fn clean_summary_chain<'a>(
    groups: &[ReplayGroup],
    summaries: &'a [ContextSummaryChunk],
) -> Vec<&'a ContextSummaryChunk> {
    let Some(first_sequence) = groups.first().and_then(first_row).map(|row| row.sequence) else {
        return Vec::new();
    };
    let mut sorted = summaries.iter().collect::<Vec<_>>();
    sorted.sort_by_key(|summary| {
        (
            summary.covered_start_sequence,
            summary.covered_end_sequence,
            summary.created_at_ms,
        )
    });

    let incremental = clean_incremental_summary_chain(groups, &sorted, first_sequence);
    let cumulative = best_cumulative_summary(groups, &sorted, first_sequence);
    if let Some(cumulative) = cumulative {
        let incremental_end = incremental
            .last()
            .map(|summary| summary.covered_end_sequence)
            .unwrap_or(i64::MIN);
        if cumulative.covered_end_sequence >= incremental_end {
            return vec![cumulative];
        }
    }

    incremental
}

fn clean_incremental_summary_chain<'a>(
    groups: &[ReplayGroup],
    sorted: &[&'a ContextSummaryChunk],
    first_sequence: i64,
) -> Vec<&'a ContextSummaryChunk> {
    let mut clean = Vec::new();
    let mut expected_start = first_sequence;
    for summary in sorted.iter().copied() {
        if summary.covered_start_sequence != expected_start {
            break;
        }
        if !has_valid_summary_boundary(groups, summary) {
            break;
        }
        expected_start = summary.covered_end_sequence.saturating_add(1);
        clean.push(summary);
    }
    clean
}

fn best_cumulative_summary<'a>(
    groups: &[ReplayGroup],
    sorted: &[&'a ContextSummaryChunk],
    first_sequence: i64,
) -> Option<&'a ContextSummaryChunk> {
    sorted
        .iter()
        .copied()
        .filter(|summary| summary.covered_start_sequence == first_sequence)
        .filter(|summary| has_valid_summary_boundary(groups, summary))
        .max_by_key(|summary| (summary.covered_end_sequence, summary.created_at_ms))
}

fn has_valid_summary_boundary(groups: &[ReplayGroup], summary: &ContextSummaryChunk) -> bool {
    summary.covered_end_sequence >= summary.covered_start_sequence
        && !starts_inside_group(groups, summary.covered_start_sequence)
        && !ends_inside_group(groups, summary.covered_end_sequence)
}

pub(crate) fn select_summary_chunks_for_replay<'a>(
    chunks: &[&'a ContextSummaryChunk],
    summary_replay_budget_tokens: usize,
) -> Vec<&'a ContextSummaryChunk> {
    if chunks.is_empty() {
        return Vec::new();
    }
    let mut selected = Vec::new();
    let mut total = 0usize;
    for chunk in chunks.iter().rev() {
        let tokens = estimate_text_tokens(&chunk.summary_markdown);
        if selected.is_empty() || total.saturating_add(tokens) <= summary_replay_budget_tokens {
            total = total.saturating_add(tokens);
            selected.push(*chunk);
        } else {
            break;
        }
    }
    selected.reverse();
    selected
}

pub(crate) fn apply_exact_tail_replay(
    groups: Vec<ReplayGroup>,
    tail_budget_tokens: usize,
) -> Vec<ReplayGroup> {
    if groups.is_empty() {
        return groups;
    }

    let tail_start = exact_tail_start_index(&groups, tail_budget_tokens);
    let mut selected = groups
        .iter()
        .take(tail_start)
        .filter(|group| is_synthetic_context_summary_group(group))
        .cloned()
        .collect::<Vec<_>>();
    selected.extend(groups.into_iter().skip(tail_start));
    selected
}

fn exact_tail_start_index(groups: &[ReplayGroup], tail_budget_tokens: usize) -> usize {
    if groups.is_empty() {
        return 0;
    }
    if tail_budget_tokens == 0 {
        return groups.len() - 1;
    }

    let mut total = 0usize;
    let mut start = groups.len();
    for (index, group) in groups.iter().enumerate().rev() {
        let tokens = estimate_group_tokens(group);
        let next = total.saturating_add(tokens);
        if total > 0 && next > tail_budget_tokens {
            break;
        }
        total = next;
        start = index;
        if total > tail_budget_tokens {
            break;
        }
    }
    start.min(groups.len() - 1)
}

fn estimate_group_tokens(group: &ReplayGroup) -> usize {
    group
        .rows
        .iter()
        .map(estimate_row_tokens)
        .sum::<usize>()
        .max(1)
}

fn estimate_row_tokens(row: &ReplayRow) -> usize {
    let mut tokens = estimate_text_tokens(&row.content);
    if let Some(tool_calls_json) = row.tool_calls_json.as_deref() {
        tokens = tokens.saturating_add(estimate_text_tokens(tool_calls_json));
    }
    tokens.max(1)
}

fn count_rows_in_range(groups: &[ReplayGroup], start_sequence: i64, end_sequence: i64) -> i64 {
    groups
        .iter()
        .flat_map(|group| group.rows.iter())
        .filter(|row| start_sequence <= row.sequence && row.sequence <= end_sequence)
        .count() as i64
}

fn row_id_for_sequence(groups: &[ReplayGroup], sequence: i64) -> Option<String> {
    groups
        .iter()
        .flat_map(|group| group.rows.iter())
        .find(|row| row.sequence == sequence)
        .map(|row| row.id.clone())
}

fn render_previous_summary_markdown(chunks: &[&ContextSummaryChunk]) -> String {
    if chunks.len() == 1 {
        return chunks[0].summary_markdown.trim().to_string();
    }

    let mut lines = vec!["# Previous Conversation Summary".to_string()];
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

fn is_synthetic_context_summary_group(group: &ReplayGroup) -> bool {
    !group.rows.is_empty() && group.rows.iter().all(|row| row.sequence <= 0)
}

fn first_row(group: &ReplayGroup) -> Option<&ReplayRow> {
    group.rows.first()
}

fn last_row(group: &ReplayGroup) -> Option<&ReplayRow> {
    group.rows.last()
}

fn group_start_sequence(group: &ReplayGroup) -> i64 {
    first_row(group).map(|row| row.sequence).unwrap_or_default()
}

fn group_end_sequence(group: &ReplayGroup) -> i64 {
    last_row(group).map(|row| row.sequence).unwrap_or_default()
}

fn starts_inside_group(groups: &[ReplayGroup], sequence: i64) -> bool {
    groups.iter().any(|group| {
        let start = group_start_sequence(group);
        let end = group_end_sequence(group);
        start < sequence && sequence <= end
    })
}

fn ends_inside_group(groups: &[ReplayGroup], sequence: i64) -> bool {
    groups.iter().any(|group| {
        let start = group_start_sequence(group);
        let end = group_end_sequence(group);
        start <= sequence && sequence < end
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(id: &str, sequence: i64, role: &str, content: &str) -> ReplayRow {
        ReplayRow {
            id: id.to_string(),
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

    fn group(sequence: i64, content: &str) -> ReplayGroup {
        ReplayGroup::new(vec![row(
            &format!("msg_{sequence}"),
            sequence,
            "user",
            content,
        )])
    }

    fn summary(start: i64, end: i64) -> ContextSummaryChunk {
        ContextSummaryChunk {
            id: format!("sum_{start}_{end}"),
            chat_id: "chat_1".to_string(),
            generation_id: None,
            summary_markdown: format!("summary {start}-{end}"),
            covered_start_message_id: Some(format!("msg_{start}")),
            covered_start_sequence: start,
            covered_end_message_id: format!("msg_{end}"),
            covered_end_sequence: end,
            tail_start_message_id: None,
            tail_start_sequence: None,
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
    fn budget_examples_match_design_shape() {
        let tiny = SummaryBudgets::from_model_context(2_096, Some(4_096));
        assert_eq!(tiny.compaction_working_budget, 2_096);
        assert_eq!(tiny.summary_trigger_tokens, 1_886);
        assert_eq!(tiny.tail_budget_tokens, 524);
        assert_eq!(tiny.summary_replay_budget_tokens, 512);
        assert_eq!(tiny.summary_output_max_tokens(), 512);

        let local_16k = SummaryBudgets::from_model_context(14_384, Some(16_384));
        assert_eq!(local_16k.summary_output_max_tokens(), 1_024);

        let unknown_local = SummaryBudgets::for_unknown_local_context();
        assert_eq!(unknown_local.provider_usable_input_tokens, 8_192);
        assert_eq!(unknown_local.summary_output_max_tokens(), 512);

        let mid = SummaryBudgets::from_model_context(62_000, Some(64_000));
        assert_eq!(mid.summary_output_max_tokens(), 2_048);
        assert_eq!(
            mid.with_model_output_limit(Some(1_024))
                .summary_output_max_tokens(),
            1_024
        );

        let small = SummaryBudgets::from_model_context(126_000, Some(128_000));
        assert_eq!(small.compaction_working_budget, 126_000);
        assert_eq!(small.summary_trigger_tokens, 113_400);
        assert_eq!(small.tail_budget_tokens, 31_500);
        assert_eq!(small.summary_replay_budget_tokens, 18_900);
        assert_eq!(small.summary_output_max_tokens(), 4_096);

        let huge = SummaryBudgets::from_model_context(998_000, Some(1_000_000));
        assert_eq!(huge.compaction_working_budget, 450_000);
        assert_eq!(huge.summary_trigger_tokens, 400_000);
        assert_eq!(huge.tail_budget_tokens, 100_000);
        assert_eq!(huge.summary_replay_budget_tokens, 64_000);
        assert_eq!(huge.summary_output_max_tokens(), 4_096);
    }

    #[test]
    fn selection_covers_only_unsummarized_gap_before_tail() {
        let groups = vec![
            group(1, "old"),
            group(2, "already"),
            group(3, "gap"),
            group(4, "tail ".repeat(100).as_str()),
        ];
        let summaries = vec![summary(1, 2)];

        let candidate = select_next_summary_candidate(&groups, &summaries, 125).unwrap();

        assert_eq!(candidate.covered_start_sequence, 1);
        assert_eq!(candidate.covered_end_sequence, 3);
        assert_eq!(candidate.tail_start_sequence, Some(4));
        assert_eq!(candidate.source_message_count, 3);
        assert_eq!(candidate.groups[0].rows[0].sequence, 3);
        assert!(
            candidate
                .previous_summary_markdown
                .as_deref()
                .unwrap()
                .contains("summary 1-2")
        );
    }

    #[test]
    fn replay_summary_budget_drops_oldest_chunks_first() {
        let first = ContextSummaryChunk {
            summary_markdown: "a".repeat(100),
            ..summary(1, 1)
        };
        let second = ContextSummaryChunk {
            summary_markdown: "b".repeat(100),
            ..summary(2, 2)
        };
        let chunks = vec![&first, &second];

        let selected = select_summary_chunks_for_replay(&chunks, 30);

        assert_eq!(selected, vec![&second]);
    }

    #[test]
    fn clean_summary_chain_rejects_summary_that_splits_group() {
        let groups = vec![
            group(1, "old"),
            ReplayGroup::new(vec![
                row("msg_2", 2, "assistant", "calling tool"),
                row("msg_3", 3, "tool", "large result"),
            ]),
            group(4, "tail"),
        ];
        let summaries = vec![summary(1, 2)];

        let clean = clean_summary_chain(&groups, &summaries);

        assert!(clean.is_empty());
    }

    #[test]
    fn clean_summary_chain_allows_boundaries_on_rows_absent_from_replay() {
        let groups = vec![group(1, "old"), group(4, "tail")];
        let summaries = vec![summary(1, 3)];

        let clean = clean_summary_chain(&groups, &summaries);

        assert_eq!(clean, vec![&summaries[0]]);
    }

    #[test]
    fn clean_summary_chain_prefers_latest_cumulative_checkpoint() {
        let groups = vec![group(1, "one"), group(2, "two"), group(3, "three")];
        let first = summary(1, 1);
        let second = ContextSummaryChunk {
            summary_markdown: "summary 1-2".to_string(),
            created_at_ms: 2,
            ..summary(1, 2)
        };
        let summaries = vec![first, second];

        let clean = clean_summary_chain(&groups, &summaries);

        assert_eq!(clean, vec![&summaries[1]]);
    }

    #[test]
    fn exact_tail_replay_preserves_synthetic_summary_prefix() {
        let groups = vec![
            ReplayGroup::new(vec![ReplayRow {
                sequence: 0,
                ..row("synthetic", 0, "assistant", "summary")
            }]),
            group(1, "old"),
            group(2, &"tail ".repeat(100)),
        ];

        let replay = apply_exact_tail_replay(groups, 20);

        assert_eq!(replay[0].rows[0].sequence, 0);
        assert_eq!(replay[1].rows[0].sequence, 2);
    }
}
