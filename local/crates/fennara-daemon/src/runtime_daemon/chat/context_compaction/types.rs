use serde_json::Value;

#[derive(Clone, Debug)]
pub(crate) struct ReplayRow {
    pub(crate) id: String,
    pub(crate) sequence: i64,
    pub(crate) role: String,
    pub(crate) status: String,
    pub(crate) content: String,
    pub(crate) tool_call_id: Option<String>,
    pub(crate) tool_name: Option<String>,
    pub(crate) tool_calls_json: Option<String>,
    pub(crate) metadata_json: Option<String>,
    pub(crate) raw_result_json: Option<String>,
    pub(crate) arguments_json: Option<String>,
    pub(crate) target_keys_json: Option<String>,
    pub(crate) tool_status: Option<String>,
}

impl ReplayRow {
    pub(crate) fn is_user(&self) -> bool {
        self.role == "user"
    }

    pub(crate) fn is_tool(&self) -> bool {
        self.role == "tool"
    }

    pub(crate) fn is_incomplete_tool_state(&self) -> bool {
        matches!(self.status.as_str(), "pending" | "in_progress" | "running")
            || matches!(
                self.tool_status.as_deref(),
                Some("pending" | "in_progress" | "running")
            )
    }

    pub(crate) fn tool_result(&self) -> Option<ToolResultRef<'_>> {
        if self.is_tool() {
            Some(ToolResultRef { row: self })
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ReplayGroup {
    pub(crate) rows: Vec<ReplayRow>,
}

impl ReplayGroup {
    pub(crate) fn new(rows: Vec<ReplayRow>) -> Self {
        Self { rows }
    }

    pub(crate) fn starts_user_turn(&self) -> bool {
        self.rows.first().is_some_and(ReplayRow::is_user)
    }

    pub(crate) fn has_incomplete_tool_state(&self) -> bool {
        self.rows.iter().any(ReplayRow::is_incomplete_tool_state)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CompactionAction {
    KeepExact,
    ReplaceWithPlaceholder,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum PlaceholderReason {
    OldToolResultUnderPressure {
        protected_estimated_tokens: usize,
        minimum_saved_estimated_tokens: usize,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PlaceholderSpec {
    pub(crate) tool_name: String,
    pub(crate) tool_call_id: Option<String>,
    pub(crate) targets: Vec<String>,
    pub(crate) details: Vec<String>,
    pub(crate) reason: PlaceholderReason,
}

#[derive(Clone, Debug)]
pub(crate) struct PlannedReplayRow {
    pub(crate) row: ReplayRow,
    pub(crate) action: CompactionAction,
    pub(crate) placeholder: Option<PlaceholderSpec>,
}

impl PlannedReplayRow {
    fn exact(row: ReplayRow) -> Self {
        Self {
            row,
            action: CompactionAction::KeepExact,
            placeholder: None,
        }
    }

    pub(crate) fn replace_with_placeholder(&mut self, placeholder: PlaceholderSpec) {
        self.action = CompactionAction::ReplaceWithPlaceholder;
        self.placeholder = Some(placeholder);
    }

    pub(crate) fn is_exact(&self) -> bool {
        self.action == CompactionAction::KeepExact
    }
}

#[derive(Clone, Debug)]
pub(crate) struct PlannedReplayGroup {
    pub(crate) rows: Vec<PlannedReplayRow>,
}

#[derive(Clone, Debug)]
pub(crate) struct ReplayPlan {
    pub(crate) groups: Vec<PlannedReplayGroup>,
}

impl ReplayPlan {
    pub(crate) fn from_groups(groups: Vec<ReplayGroup>) -> Self {
        Self {
            groups: groups
                .into_iter()
                .map(|group| PlannedReplayGroup {
                    rows: group
                        .rows
                        .into_iter()
                        .map(PlannedReplayRow::exact)
                        .collect(),
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ContextSummaryChunk {
    pub(crate) id: String,
    pub(crate) chat_id: String,
    pub(crate) generation_id: Option<String>,
    pub(crate) summary_markdown: String,
    pub(crate) covered_start_message_id: Option<String>,
    pub(crate) covered_start_sequence: i64,
    pub(crate) covered_end_message_id: String,
    pub(crate) covered_end_sequence: i64,
    pub(crate) tail_start_message_id: Option<String>,
    pub(crate) tail_start_sequence: Option<i64>,
    pub(crate) source_message_count: i64,
    pub(crate) model: Option<String>,
    pub(crate) provider_id: Option<String>,
    pub(crate) model_id: Option<String>,
    pub(crate) model_variant: Option<String>,
    pub(crate) model_ref_json: Option<String>,
    pub(crate) metadata_json: Option<String>,
    pub(crate) created_at_ms: i64,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ToolResultRef<'a> {
    pub(crate) row: &'a ReplayRow,
}

impl<'a> ToolResultRef<'a> {
    pub(crate) fn tool_name(self) -> Option<&'a str> {
        self.row.tool_name.as_deref()
    }

    pub(crate) fn tool_call_id(self) -> Option<&'a str> {
        self.row.tool_call_id.as_deref()
    }

    pub(crate) fn content_markdown(self) -> &'a str {
        &self.row.content
    }

    pub(crate) fn status(self) -> &'a str {
        self.row
            .tool_status
            .as_deref()
            .unwrap_or(self.row.status.as_str())
    }

    pub(crate) fn arguments(self) -> Option<Value> {
        parse_json(self.row.arguments_json.as_deref())
    }

    pub(crate) fn metadata(self) -> Option<Value> {
        parse_json(self.row.metadata_json.as_deref())
    }

    pub(crate) fn raw_result(self) -> Option<Value> {
        parse_json(self.row.raw_result_json.as_deref())
    }

    pub(crate) fn target_keys(self) -> Vec<String> {
        parse_string_array(self.row.target_keys_json.as_deref())
    }
}

pub(crate) fn parse_json(raw: Option<&str>) -> Option<Value> {
    serde_json::from_str(raw?).ok()
}

pub(crate) fn parse_string_array(raw: Option<&str>) -> Vec<String> {
    let Some(Value::Array(values)) = parse_json(raw) else {
        return Vec::new();
    };
    values
        .iter()
        .filter_map(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}
