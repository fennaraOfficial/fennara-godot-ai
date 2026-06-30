use axum::extract::ws::Message;
use futures_util::Sink;
use serde_json::{Value, json};
use std::time::Instant;

use crate::runtime_daemon::{godot_bridge, state::AppState};

use super::super::{
    BoundChatProject, ClientRequest, context, images, prompt, providers, send_chat_list,
    send_chat_updated, send_error, send_json, settings, store, trace,
};
use super::{
    CHAT_ALREADY_RUNNING_MESSAGE, cost, is_chat_cancelled,
    publisher::stream_one_assistant,
    request::build_provider_messages,
    tool_loop::{self, ToolLoopResult},
    try_begin_chat_turn,
};

pub(in crate::runtime_daemon::chat) async fn run_chat<S>(
    sender: &mut S,
    active_chat_id: &mut Option<String>,
    state: &AppState,
    bound_project: &BoundChatProject,
    request: ClientRequest,
) -> Result<(), S::Error>
where
    S: Sink<Message> + Unpin,
    S::Error: std::fmt::Debug,
{
    let request_id = request.request_id.clone();
    let message = request.message.unwrap_or_default();
    let message = message.trim();
    let context_snippets = match context::validate_client_snippets(request.context_snippets) {
        Ok(snippets) => snippets,
        Err(error) => return send_error(sender, request_id, "bad_request", &error).await,
    };
    let model_message = context::message_with_context_snippets(message, &context_snippets);
    let user_images = match images::validate_images(request.images) {
        Ok(images) => images,
        Err(error) => return send_error(sender, request_id, "bad_request", &error).await,
    };
    if model_message.trim().is_empty() && user_images.is_empty() {
        return send_error(
            sender,
            request_id,
            "bad_request",
            "Message or image is required.",
        )
        .await;
    }

    let settings = settings::load_settings();
    let model = request
        .model
        .as_deref()
        .and_then(settings::clean_model)
        .unwrap_or_else(|| settings.model.clone());
    if let Some(error) = providers::missing_auth_for_model(&settings, &model) {
        return send_error(sender, request_id, error.code(), &error.user_message()).await;
    }
    let scope = &bound_project.scope;
    let reasoning_effort = settings::clean_reasoning_effort(
        request
            .reasoning_effort
            .as_deref()
            .unwrap_or(&settings.reasoning_effort),
    )
    .to_string();
    let chat_id = match request.chat_id.or_else(|| active_chat_id.clone()) {
        Some(chat_id) => chat_id,
        None => match store::create_chat(scope, &model, &reasoning_effort) {
            Ok(opened) => opened.chat.id,
            Err(error) => {
                return send_error(sender, request_id, "chat_create_failed", &error).await;
            }
        },
    };
    let turn_started_at = Instant::now();
    let trace = trace::TraceRecorder::new(
        chat_id.clone(),
        request_id.clone(),
        Some(bound_project.session_id.clone()),
    );
    trace.event(
        "turn.start",
        json!({
            "trace_id": trace.trace_id(),
            "turn_id": trace.turn_id(),
            "model": model.as_str(),
            "reasoning_effort": reasoning_effort.as_str(),
            "message_chars": message.chars().count(),
            "image_count": user_images.len(),
            "context_snippet_count": context_snippets.len(),
            "godot_session_id": bound_project.session_id.as_str(),
            "project_path_present": scope.project_path.is_some()
        }),
    );
    if let Err(error) = store::ensure_chat_in_scope(scope, &chat_id) {
        trace.error(
            "turn.failed",
            "failed",
            json!({ "code": "chat_scope_mismatch", "message": error.as_str() }),
        );
        return send_error(sender, request_id, "chat_scope_mismatch", &error).await;
    }
    let Some(_active_turn) = try_begin_chat_turn(state, &chat_id).await else {
        trace.warn(
            "turn.rejected",
            "already_running",
            json!({ "code": "chat_already_running" }),
        );
        return send_error(
            sender,
            request_id,
            "chat_already_running",
            CHAT_ALREADY_RUNNING_MESSAGE,
        )
        .await;
    };
    if let Err(error) = store::set_chat_model(&chat_id, &model, &reasoning_effort) {
        trace.error(
            "turn.failed",
            "failed",
            json!({ "code": "chat_store_failed", "message": error.as_str() }),
        );
        return send_error(sender, request_id, "chat_store_failed", &error).await;
    }
    state.cancelled_chats.write().await.remove(&chat_id);
    *active_chat_id = Some(chat_id.clone());
    let replay_span = trace.start_span("prompt.replay", json!({ "chat_id": chat_id.as_str() }));
    let replay_messages = match store::replay_messages(&chat_id) {
        Ok(messages) => {
            replay_span.finish("ok", json!({ "message_count": messages.len() }));
            messages
        }
        Err(error) => {
            replay_span.fail(json!({ "message": error.as_str() }));
            trace.error(
                "turn.failed",
                "failed",
                json!({ "code": "chat_replay_failed", "message": error.as_str() }),
            );
            return send_error(sender, request_id, "chat_replay_failed", &error).await;
        }
    };
    let active_project = state
        .projects
        .read()
        .await
        .get(&bound_project.session_id)
        .cloned();
    let prompt_context = prompt::PromptRuntimeContext::from_turn(
        settings.approval_mode,
        scope,
        active_project.as_ref(),
    );
    let prompt_span = trace.start_span(
        "prompt.build",
        json!({
            "replay_message_count": replay_messages.len(),
            "image_count": user_images.len(),
            "context_snippet_count": context_snippets.len()
        }),
    );
    let mut provider_messages = build_provider_messages(
        &replay_messages,
        &model_message,
        &user_images,
        &prompt_context,
    );
    prompt_span.finish(
        "ok",
        json!({
            "provider_message_count": provider_messages.len()
        }),
    );
    let snapshot_span = trace.start_span("snapshot", json!({ "chat_id": chat_id.as_str() }));
    let snapshot_result = godot_bridge::begin_snapshot_turn_for_session_traced(
        state,
        Some(&bound_project.session_id),
        &chat_id,
        message,
        Some(&trace),
    )
    .await;
    if snapshot_result.get("ok").and_then(Value::as_bool) == Some(false) {
        let error = snapshot_result
            .get("error")
            .and_then(Value::as_str)
            .unwrap_or("Failed to begin a local revert snapshot.");
        snapshot_span.fail(json!({ "message": error }));
        trace.error(
            "turn.failed",
            "failed",
            json!({ "code": "snapshot_failed", "message": error }),
        );
        return send_error(sender, request_id, "snapshot_failed", error).await;
    }
    snapshot_span.finish(
        "ok",
        json!({
            "response_bytes": trace::value_size(&snapshot_result)
        }),
    );
    state.revertable_chats.write().await.insert(chat_id.clone());

    let user_write_span = trace.start_span(
        "db.write",
        json!({
            "action": "insert_user_message",
            "chat_id": chat_id.as_str()
        }),
    );
    let user_message = match store::insert_user_message(
        &chat_id,
        message,
        merged_user_metadata(
            images::metadata_value(&user_images),
            context::metadata_value(&context_snippets),
        )
        .as_ref(),
    ) {
        Ok(message) => {
            user_write_span.finish(
                "ok",
                json!({
                    "message_id": message.id.as_str(),
                    "content_chars": message.content.chars().count()
                }),
            );
            message
        }
        Err(error) => {
            user_write_span.fail(json!({ "message": error.as_str() }));
            trace.error(
                "turn.failed",
                "failed",
                json!({ "code": "chat_store_failed", "message": error.as_str() }),
            );
            return send_error(sender, request_id, "chat_store_failed", &error).await;
        }
    };
    send_json(
        sender,
        json!({
            "type": "chat_user_message",
            "request_id": request_id.clone(),
            "chat_id": chat_id,
            "user_message": user_message
        }),
    )
    .await?;
    let chat_summary = match store::chat_summary(&chat_id) {
        Ok(chat) => chat,
        Err(error) => return send_error(sender, request_id, "chat_store_failed", &error).await,
    };
    send_json(
        sender,
        json!({
            "type": "chat_updated",
            "request_id": request_id.clone(),
            "chat": chat_summary
        }),
    )
    .await?;
    send_chat_list(sender, None, scope).await?;

    let assistant_generation_write_span = trace.start_span(
        "db.write",
        json!({
            "action": "insert_assistant_placeholder_with_generation",
            "chat_id": chat_id.as_str()
        }),
    );
    let (assistant_message, assistant_generation) =
        match store::insert_assistant_placeholder_with_generation(
            &chat_id,
            &model,
            &reasoning_effort,
        ) {
            Ok((message, generation)) => {
                assistant_generation_write_span.finish(
                    "ok",
                    json!({
                        "message_id": message.id.as_str(),
                        "generation_id": generation.id.as_str()
                    }),
                );
                (message, generation)
            }
            Err(error) => {
                assistant_generation_write_span.fail(json!({ "message": error.as_str() }));
                trace.error(
                    "turn.failed",
                    "failed",
                    json!({ "code": "chat_store_failed", "message": error.as_str() }),
                );
                return send_error(sender, request_id, "chat_store_failed", &error).await;
            }
        };
    let mut current_trace = trace.with_generation(&assistant_generation.id, &assistant_message.id);
    current_trace.event_status(
        "generation.start",
        "running",
        json!({
            "generation_id": assistant_generation.id.as_str(),
            "assistant_message_id": assistant_message.id.as_str(),
            "model": model.as_str()
        }),
    );
    send_json(
        sender,
        json!({
            "type": "chat_stream_start",
            "request_id": request_id.clone(),
            "chat_id": chat_id,
            "assistant_message": assistant_message,
            "model": model,
            "reasoning_effort": reasoning_effort,
            "can_revert": true
        }),
    )
    .await?;

    let mut current_assistant = assistant_message;
    let mut current_generation = assistant_generation;
    let provider_settings = providers::settings_from_chat(&settings);

    let (final_usage, final_text, stored_assistant) = loop {
        let streamed = stream_one_assistant(
            sender,
            request_id.clone(),
            provider_settings.clone(),
            &model,
            &reasoning_effort,
            &provider_messages,
            &current_assistant.id,
            state,
            &chat_id,
            current_trace.clone(),
        )
        .await;

        let streamed = match streamed {
            Ok(Ok(streamed)) => streamed,
            Ok(Err(error)) => {
                let error_text = format!("Request failed: {}", error.user_message());
                let error_json = json!({
                    "code": error.code(),
                    "message": error.user_message()
                });
                let _ =
                    store::finish_generation(&current_generation.id, "failed", Some(&error_json));
                let _ = store::fail_assistant_message(&current_assistant.id, &error_text);
                current_trace.error(
                    "generation.failed",
                    "failed",
                    json!({
                        "generation_id": current_generation.id.as_str(),
                        "error_code": error.code()
                    }),
                );
                trace.error(
                    "turn.failed",
                    "failed",
                    json!({
                        "error_code": error.code(),
                        "duration_ms": turn_started_at.elapsed().as_millis() as i64
                    }),
                );
                send_json(
                    sender,
                    json!({
                        "type": "chat_item_update",
                        "request_id": request_id.clone(),
                        "item": {
                            "type": "message",
                            "content": error_text
                        }
                    }),
                )
                .await?;
                send_chat_updated(sender, request_id.clone(), &chat_id).await?;
                return send_error(sender, request_id, error.code(), &error.user_message()).await;
            }
            Err(error) => return Err(error),
        };

        if is_chat_cancelled(state, &chat_id).await {
            let partial = streamed.completion.content.clone();
            let _ = store::finish_generation(&current_generation.id, "cancelled", None);
            current_trace.event_status(
                "generation.done",
                "cancelled",
                json!({ "generation_id": current_generation.id.as_str() }),
            );
            trace.event_status(
                "turn.cancelled",
                "cancelled",
                json!({
                    "duration_ms": turn_started_at.elapsed().as_millis() as i64,
                    "partial_chars": partial.chars().count()
                }),
            );
            tool_loop::finish_cancelled_turn(
                sender,
                request_id,
                state,
                scope,
                &chat_id,
                &current_assistant.id,
                &partial,
            )
            .await?;
            return Ok(());
        }

        let usage = cost::usage_for_model(&provider_settings, &model, streamed.usage.as_ref());
        if let Some(error) =
            completion_harness_error(&streamed.completion, provider_for_model(&model))
        {
            let error_text = format!("Request failed: {}", error.user_message());
            let error_json = json!({
                "code": error.code(),
                "message": error.user_message()
            });
            let _ = store::finish_generation(&current_generation.id, "failed", Some(&error_json));
            let _ = store::fail_assistant_message(&current_assistant.id, &error_text);
            current_trace.error(
                "generation.failed",
                "failed",
                json!({
                    "generation_id": current_generation.id.as_str(),
                    "error_code": error.code()
                }),
            );
            trace.error(
                "turn.failed",
                "failed",
                json!({
                    "error_code": error.code(),
                    "duration_ms": turn_started_at.elapsed().as_millis() as i64
                }),
            );
            send_json(
                sender,
                json!({
                    "type": "chat_item_update",
                    "request_id": request_id.clone(),
                    "item": {
                        "type": "message",
                        "content": error_text
                    }
                }),
            )
            .await?;
            send_chat_updated(sender, request_id.clone(), &chat_id).await?;
            return send_error(sender, request_id, error.code(), &error.user_message()).await;
        }

        let tool_calls = streamed.completion.tool_calls;
        if tool_calls.is_empty() {
            let final_text = streamed.completion.content.clone();
            let assistant_finish_span = current_trace.start_span(
                "db.write",
                json!({
                    "action": "finish_assistant_message",
                    "assistant_message_id": current_assistant.id.as_str(),
                    "generation_id": current_generation.id.as_str()
                }),
            );
            let stored_assistant = match store::finish_assistant_message(
                &current_assistant.id,
                &final_text,
                streamed.reasoning_content.as_deref(),
                Some(&usage),
                &model,
                Some(&current_generation.id),
            ) {
                Ok(message) => message,
                Err(error) => {
                    let error_json = json!({ "message": error });
                    let _ = store::finish_generation(
                        &current_generation.id,
                        "failed",
                        Some(&error_json),
                    );
                    assistant_finish_span.fail(json!({ "message": error.as_str() }));
                    current_trace.error(
                        "generation.failed",
                        "failed",
                        json!({
                            "generation_id": current_generation.id.as_str(),
                            "code": "chat_store_failed"
                        }),
                    );
                    trace.error(
                        "turn.failed",
                        "failed",
                        json!({
                            "code": "chat_store_failed",
                            "duration_ms": turn_started_at.elapsed().as_millis() as i64
                        }),
                    );
                    return send_error(sender, request_id, "chat_store_failed", &error).await;
                }
            };
            assistant_finish_span.finish(
                "ok",
                json!({
                    "assistant_message_id": stored_assistant.id.as_str(),
                    "content_chars": final_text.chars().count(),
                    "usage_present": true
                }),
            );
            if let Err(error) = store::finish_generation(&current_generation.id, "done", None) {
                current_trace.error(
                    "generation.failed",
                    "failed",
                    json!({
                        "generation_id": current_generation.id.as_str(),
                        "code": "chat_store_failed",
                        "message": error.as_str()
                    }),
                );
                trace.error(
                    "turn.failed",
                    "failed",
                    json!({
                        "code": "chat_store_failed",
                        "duration_ms": turn_started_at.elapsed().as_millis() as i64
                    }),
                );
                return send_error(sender, request_id, "chat_store_failed", &error).await;
            }
            current_trace.event_status(
                "generation.done",
                "done",
                json!({
                    "generation_id": current_generation.id.as_str(),
                    "assistant_message_id": current_assistant.id.as_str(),
                    "content_chars": final_text.chars().count(),
                    "tool_call_count": 0
                }),
            );
            break (Some(usage), final_text, stored_assistant);
        }

        let model_tool_calls = model_tool_calls(&tool_calls);
        let tool_calls_value = Value::Array(model_tool_calls.clone());
        let tool_call_count = tool_calls.len();
        let assistant_tool_write_span = current_trace.start_span(
            "db.write",
            json!({
                "action": "persist_assistant_tool_calls",
                "assistant_message_id": current_assistant.id.as_str(),
                "generation_id": current_generation.id.as_str(),
                "tool_call_count": tool_call_count
            }),
        );
        if let Err(error) = store::finish_assistant_message_with_tool_calls(
            &current_assistant.id,
            &tool_calls_value,
            &streamed.completion.content,
            streamed.reasoning_content.as_deref(),
            Some(&usage),
            &model,
            Some(&current_generation.id),
        )
        .map(|_| ())
        {
            let error_json = json!({ "message": error });
            let _ = store::finish_generation(&current_generation.id, "failed", Some(&error_json));
            assistant_tool_write_span.fail(json!({ "message": error.as_str() }));
            current_trace.error(
                "generation.failed",
                "failed",
                json!({
                    "generation_id": current_generation.id.as_str(),
                    "code": "chat_store_failed"
                }),
            );
            trace.error(
                "turn.failed",
                "failed",
                json!({
                    "code": "chat_store_failed",
                    "duration_ms": turn_started_at.elapsed().as_millis() as i64
                }),
            );
            return send_error(sender, request_id, "chat_store_failed", &error).await;
        }
        assistant_tool_write_span.finish(
            "ok",
            json!({
                "assistant_message_id": current_assistant.id.as_str(),
                "tool_call_count": tool_call_count
            }),
        );
        send_chat_updated(sender, request_id.clone(), &chat_id).await?;
        provider_messages.push(json!({
            "role": "assistant",
            "content": streamed.completion.content,
            "tool_calls": model_tool_calls
        }));

        match tool_loop::run_tool_calls(
            sender,
            request_id.clone(),
            state,
            bound_project,
            scope,
            &chat_id,
            &current_assistant.id,
            &current_generation.id,
            &streamed.completion.content,
            settings.approval_mode,
            tool_calls,
            current_trace.clone(),
        )
        .await?
        {
            ToolLoopResult::Completed {
                provider_messages: tool_messages,
            } => {
                provider_messages.extend(tool_messages);
            }
            ToolLoopResult::Stopped => return Ok(()),
        }
        if let Err(error) = store::finish_generation(&current_generation.id, "done", None) {
            current_trace.error(
                "generation.failed",
                "failed",
                json!({
                    "generation_id": current_generation.id.as_str(),
                    "code": "chat_store_failed",
                    "message": error.as_str()
                }),
            );
            trace.error(
                "turn.failed",
                "failed",
                json!({
                    "code": "chat_store_failed",
                    "duration_ms": turn_started_at.elapsed().as_millis() as i64
                }),
            );
            return send_error(sender, request_id, "chat_store_failed", &error).await;
        }
        current_trace.event_status(
            "generation.done",
            "done",
            json!({
                "generation_id": current_generation.id.as_str(),
                "assistant_message_id": current_assistant.id.as_str(),
                "tool_call_count": tool_call_count
            }),
        );

        trace.event_status(
            "continuation.start",
            "running",
            json!({
                "previous_generation_id": current_generation.id.as_str(),
                "provider_message_count": provider_messages.len()
            }),
        );
        let continuation_generation_span = trace.start_span(
            "db.write",
            json!({
                "action": "insert_assistant_placeholder_with_generation",
                "chat_id": chat_id.as_str(),
                "continuation": true
            }),
        );
        let (next_assistant, next_generation) =
            match store::insert_assistant_placeholder_with_generation(
                &chat_id,
                &model,
                &reasoning_effort,
            ) {
                Ok((message, generation)) => {
                    continuation_generation_span.finish(
                        "ok",
                        json!({
                            "message_id": message.id.as_str(),
                            "generation_id": generation.id.as_str()
                        }),
                    );
                    (message, generation)
                }
                Err(error) => {
                    continuation_generation_span.fail(json!({ "message": error.as_str() }));
                    trace.error(
                        "turn.failed",
                        "failed",
                        json!({
                            "code": "chat_store_failed",
                            "duration_ms": turn_started_at.elapsed().as_millis() as i64
                        }),
                    );
                    return send_error(sender, request_id, "chat_store_failed", &error).await;
                }
            };
        current_assistant = next_assistant;
        current_generation = next_generation;
        current_trace = trace.with_generation(&current_generation.id, &current_assistant.id);
        current_trace.event_status(
            "generation.start",
            "running",
            json!({
                "generation_id": current_generation.id.as_str(),
                "assistant_message_id": current_assistant.id.as_str(),
                "model": model.as_str()
            }),
        );
    };

    trace.event_status(
        "turn.done",
        "done",
        json!({
            "duration_ms": turn_started_at.elapsed().as_millis() as i64,
            "final_generation_id": current_generation.id.as_str(),
            "final_response_chars": final_text.chars().count(),
            "usage_present": final_usage.is_some()
        }),
    );
    send_json(
        sender,
        json!({
            "type": "chat_stream_done",
            "request_id": request_id.clone()
        }),
    )
    .await?;
    send_json(
        sender,
        json!({
            "type": "chat_response",
            "request_id": request_id.clone(),
            "chat_id": chat_id,
            "message": stored_assistant,
            "response": final_text,
            "usage": final_usage
        }),
    )
    .await?;
    send_chat_list(sender, None, scope).await
}

fn merged_user_metadata(images: Option<Value>, context: Option<Value>) -> Option<Value> {
    match (images, context) {
        (None, None) => None,
        (Some(value), None) | (None, Some(value)) => Some(value),
        (Some(mut left), Some(right)) => {
            if let (Some(left), Some(right)) = (left.as_object_mut(), right.as_object()) {
                for (key, value) in right {
                    left.insert(key.clone(), value.clone());
                }
            }
            Some(left)
        }
    }
}

fn model_tool_calls(tool_calls: &[Value]) -> Vec<Value> {
    tool_calls
        .iter()
        .map(|tool_call| {
            let mut value = tool_call.clone();
            if let Some(object) = value.as_object_mut() {
                object.remove("provider_tool_call_id");
            }
            value
        })
        .collect()
}

fn completion_harness_error(
    completion: &providers::ChatCompletion,
    provider: &str,
) -> Option<providers::LlmError> {
    if completion.tool_call_observation.has_malformed() {
        let malformed = &completion.tool_call_observation.malformed[0];
        let name = malformed
            .name
            .as_deref()
            .filter(|name| !name.trim().is_empty())
            .map(|name| format!(" for `{name}`"))
            .unwrap_or_default();
        return Some(providers::LlmError::InvalidProviderOutput {
            provider: provider.to_string(),
            message: format!(
                "The provider returned a malformed tool call{name}: {}.",
                malformed.message
            ),
            raw: malformed.raw.clone(),
        });
    }

    if completion.finish_reason == providers::FinishReason::ToolCalls
        && completion.tool_calls.is_empty()
    {
        return Some(providers::LlmError::InvalidProviderOutput {
            provider: provider.to_string(),
            message: "The provider finished with tool_calls, but no usable tool call was returned."
                .to_string(),
            raw: None,
        });
    }

    match completion.finish_reason {
        providers::FinishReason::Length => Some(providers::LlmError::InvalidProviderOutput {
            provider: provider.to_string(),
            message: "The provider stopped because the response hit the token limit before the turn completed.".to_string(),
            raw: None,
        }),
        providers::FinishReason::ContentFilter => Some(providers::LlmError::ProviderApi {
            provider: provider.to_string(),
            status: None,
            message: "The provider stopped the response because of a content filter.".to_string(),
            retryable: false,
        }),
        _ => None,
    }
}

fn provider_for_model(model: &str) -> &str {
    model.split('/').next().unwrap_or("chat").trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_calls_finish_without_valid_calls_is_harness_error() {
        let completion = providers::ChatCompletion {
            content: String::new(),
            tool_calls: Vec::new(),
            finish_reason: providers::FinishReason::ToolCalls,
            tool_call_observation: providers::ToolCallObservation::none(),
        };

        let error = completion_harness_error(&completion, "openrouter").unwrap();

        assert_eq!(error.code(), "invalid_provider_output");
        assert!(error.user_message().contains("no usable tool call"));
    }

    #[test]
    fn malformed_observed_tool_call_is_harness_error() {
        let completion = providers::ChatCompletion {
            content: String::new(),
            tool_calls: Vec::new(),
            finish_reason: providers::FinishReason::ToolCalls,
            tool_call_observation: providers::ToolCallObservation {
                observed: 1,
                malformed: vec![providers::MalformedToolCall {
                    id: "call_1".to_string(),
                    name: Some("read_file".to_string()),
                    arguments: "{".to_string(),
                    message: "Tool call arguments are not valid JSON".to_string(),
                    raw: Some("{bad".to_string()),
                }],
            },
        };

        let error = completion_harness_error(&completion, "openrouter").unwrap();

        assert_eq!(error.code(), "invalid_provider_output");
        assert!(error.user_message().contains("malformed tool call"));
    }

    #[test]
    fn valid_tool_call_does_not_trip_harness_error() {
        let completion = providers::ChatCompletion {
            content: String::new(),
            tool_calls: vec![json!({
                "id": "call_1",
                "type": "function",
                "function": { "name": "read_file", "arguments": "{}" }
            })],
            finish_reason: providers::FinishReason::ToolCalls,
            tool_call_observation: providers::ToolCallObservation {
                observed: 1,
                malformed: Vec::new(),
            },
        };

        assert!(completion_harness_error(&completion, "openrouter").is_none());
    }

    #[test]
    fn model_tool_calls_strip_provider_tool_call_ids() {
        let tool_calls = model_tool_calls(&[json!({
            "id": "call_internal",
            "provider_tool_call_id": "tool_call_0",
            "type": "function",
            "function": { "name": "read_file", "arguments": "{}" }
        })]);

        assert_eq!(tool_calls[0]["id"], "call_internal");
        assert!(tool_calls[0].get("provider_tool_call_id").is_none());
    }
}
