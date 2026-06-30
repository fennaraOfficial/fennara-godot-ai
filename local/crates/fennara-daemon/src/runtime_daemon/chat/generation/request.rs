use serde_json::Value;

use super::super::{images, prompt, providers};

pub(super) fn build_provider_messages(
    replay_messages: &[Value],
    user_message: &str,
    user_images: &[images::ChatImage],
    runtime_context: &prompt::PromptRuntimeContext,
) -> Vec<Value> {
    // Historical media is stripped to placeholders by store::replay_messages.
    // Only current-turn attachments are allowed to carry image bytes forward.
    let system_prompt = prompt::PromptBuilder::new(runtime_context).build();
    providers::build_messages(&system_prompt, replay_messages, user_message, user_images)
}
