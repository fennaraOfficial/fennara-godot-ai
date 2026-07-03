use super::{
    pressure, tail,
    types::{ReplayGroup, ReplayPlan},
};

#[derive(Clone, Copy, Debug)]
pub(crate) struct ReplayPlanConfig {
    pub(crate) latest_exact_user_turns: usize,
    pub(crate) old_tool_result_protect_tokens: usize,
    pub(crate) old_tool_result_minimum_saved_tokens: usize,
}

impl Default for ReplayPlanConfig {
    fn default() -> Self {
        Self {
            latest_exact_user_turns: tail::DEFAULT_LATEST_EXACT_USER_TURNS,
            old_tool_result_protect_tokens: pressure::DEFAULT_PRUNE_PROTECT_TOKENS,
            old_tool_result_minimum_saved_tokens: pressure::DEFAULT_PRUNE_MINIMUM_SAVED_TOKENS,
        }
    }
}

pub(crate) fn plan_replay(groups: Vec<ReplayGroup>) -> ReplayPlan {
    plan_replay_with_config(groups, ReplayPlanConfig::default())
}

pub(crate) fn plan_replay_with_config(
    groups: Vec<ReplayGroup>,
    config: ReplayPlanConfig,
) -> ReplayPlan {
    let protected = tail::protected_groups(&groups, config.latest_exact_user_turns);
    let mut plan = ReplayPlan::from_groups(groups);
    pressure::apply_pressure_fallback(
        &mut plan,
        &protected,
        config.old_tool_result_protect_tokens,
        config.old_tool_result_minimum_saved_tokens,
    );
    plan
}
