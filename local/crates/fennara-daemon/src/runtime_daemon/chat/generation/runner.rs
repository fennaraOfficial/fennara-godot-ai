use axum::extract::ws::Message;
use futures_util::Sink;
use serde_json::{Value, json};
use std::{sync::Arc, time::Instant};
use tokio::sync::Mutex;

use crate::runtime_daemon::{godot_bridge, state::AppState};

use super::super::{
    BoundChatProject, ClientRequest, context, context_compaction, ids, images, prompt, providers,
    send_chat_list, send_chat_updated, send_error, send_json, settings, store, tools, trace,
};
use super::{
    CHAT_ALREADY_RUNNING_MESSAGE, cost, is_chat_cancelled,
    publisher::{AssistantStreamError, stream_one_assistant},
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
    let mut provider_settings = providers::settings_from_chat(&settings);
    if let Err(error) =
        providers::hydrate_selected_local_model_limits(&mut provider_settings, &model).await
    {
        trace.warn(
            "context.local_model_limits_unavailable",
            "skipped",
            json!({ "model": model.as_str(), "message": error }),
        );
    }
    let allow_tool_image_followups = providers::selected_model_supports_image_input(
        &provider_settings,
        &model,
        &reasoning_effort,
    );
    trace.event_status(
        "model.image_input",
        if allow_tool_image_followups {
            "supported"
        } else {
            "unsupported"
        },
        json!({
            "model": model.as_str(),
            "tool_image_followups": allow_tool_image_followups
        }),
    );
    let summary_budgets =
        summary_budgets_for_model(&provider_settings, &model, &reasoning_effort, &trace);

    let replay_span = trace.start_span("prompt.replay", json!({ "chat_id": chat_id.as_str() }));
    let mut replay_messages = match replay_messages_for_budget(&chat_id, summary_budgets) {
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
    if let Some(budgets) = summary_budgets {
        let estimated = estimate_provider_input_tokens(
            &provider_settings,
            &model,
            &reasoning_effort,
            &provider_messages,
        );
        if let Some(estimated) = estimated {
            if (estimated as usize) > budgets.summary_trigger_tokens {
                let summary_span = trace.start_span(
                    "context.summary",
                    json!({
                        "chat_id": chat_id.as_str(),
                        "estimated_input_tokens": estimated,
                        "summary_trigger_tokens": budgets.summary_trigger_tokens,
                        "tail_budget_tokens": budgets.tail_budget_tokens,
                        "summary_replay_budget_tokens": budgets.summary_replay_budget_tokens
                    }),
                );
                send_context_compaction_status(sender, request_id.clone(), &chat_id, "running")
                    .await?;
                match try_create_context_summary(
                    provider_settings.clone(),
                    &model,
                    &reasoning_effort,
                    &chat_id,
                    budgets,
                    estimated,
                    None,
                )
                .await
                {
                    Ok(Some(summary)) => {
                        summary_span.finish(
                            "ok",
                            json!({
                                "summary_id": summary.id,
                                "covered_start_sequence": summary.covered_start_sequence,
                                "covered_end_sequence": summary.covered_end_sequence,
                                "source_message_count": summary.source_message_count
                            }),
                        );
                        send_context_compaction_status(
                            sender,
                            request_id.clone(),
                            &chat_id,
                            "done",
                        )
                        .await?;
                        replay_messages = match replay_messages_for_budget(&chat_id, Some(budgets))
                        {
                            Ok(messages) => messages,
                            Err(error) => {
                                trace.warn(
                                    "context.summary.replay_reload_failed",
                                    "failed",
                                    json!({ "message": error.as_str() }),
                                );
                                replay_messages
                            }
                        };
                        provider_messages = build_provider_messages(
                            &replay_messages,
                            &model_message,
                            &user_images,
                            &prompt_context,
                        );
                    }
                    Ok(None) => {
                        summary_span.finish("skipped", json!({ "reason": "no_candidate" }));
                        send_context_compaction_status(
                            sender,
                            request_id.clone(),
                            &chat_id,
                            "skipped",
                        )
                        .await?;
                        if let Some(messages) =
                            bounded_replay_after_summary_failure(&chat_id, budgets, &trace)
                        {
                            replay_messages = messages;
                            provider_messages = build_provider_messages(
                                &replay_messages,
                                &model_message,
                                &user_images,
                                &prompt_context,
                            );
                        }
                    }
                    Err(error) => {
                        summary_span.fail(json!({ "message": error.as_str() }));
                        trace.warn(
                            "context.summary.failed",
                            "failed",
                            json!({ "message": error.as_str() }),
                        );
                        send_context_compaction_status(
                            sender,
                            request_id.clone(),
                            &chat_id,
                            "failed",
                        )
                        .await?;
                        if let Some(messages) =
                            bounded_replay_after_summary_failure(&chat_id, budgets, &trace)
                        {
                            replay_messages = messages;
                            provider_messages = build_provider_messages(
                                &replay_messages,
                                &model_message,
                                &user_images,
                                &prompt_context,
                            );
                        }
                    }
                }
            }
        }
    }
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
    let mut overflow_retry_used = false;
    let mut overflow_recovery_retry_sent = false;

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
            Ok(Err(stream_error)) => {
                if should_retry_context_overflow(&stream_error, overflow_retry_used) {
                    overflow_retry_used = true;
                    let recovery_span = trace.start_span(
                        "context.overflow_recovery",
                        json!({
                            "chat_id": chat_id.as_str(),
                            "before_sequence": user_message.sequence
                        }),
                    );
                    if let Some(budgets) = summary_budgets {
                        let retry_estimated_tokens = estimate_provider_input_tokens(
                            &provider_settings,
                            &model,
                            &reasoning_effort,
                            &provider_messages,
                        )
                        .unwrap_or(0);
                        send_context_compaction_status(
                            sender,
                            request_id.clone(),
                            &chat_id,
                            "running",
                        )
                        .await?;
                        match try_create_context_summary(
                            provider_settings.clone(),
                            &model,
                            &reasoning_effort,
                            &chat_id,
                            budgets,
                            retry_estimated_tokens,
                            Some(user_message.sequence),
                        )
                        .await
                        {
                            Ok(Some(summary)) => {
                                trace.event_status(
                                    "context.overflow_recovery.summary",
                                    "ok",
                                    json!({
                                        "summary_id": summary.id,
                                        "covered_start_sequence": summary.covered_start_sequence,
                                        "covered_end_sequence": summary.covered_end_sequence
                                    }),
                                );
                                send_context_compaction_status(
                                    sender,
                                    request_id.clone(),
                                    &chat_id,
                                    "done",
                                )
                                .await?;
                            }
                            Ok(None) => {
                                trace.event_status(
                                    "context.overflow_recovery.summary",
                                    "skipped",
                                    json!({ "reason": "no_candidate" }),
                                );
                                send_context_compaction_status(
                                    sender,
                                    request_id.clone(),
                                    &chat_id,
                                    "skipped",
                                )
                                .await?;
                            }
                            Err(error) => {
                                trace.event_status(
                                    "context.overflow_recovery.summary",
                                    "failed",
                                    json!({ "message": error.as_str() }),
                                );
                                send_context_compaction_status(
                                    sender,
                                    request_id.clone(),
                                    &chat_id,
                                    "failed",
                                )
                                .await?;
                            }
                        }
                        if let Some(messages) = bounded_replay_after_summary_failure_before_sequence(
                            &chat_id,
                            budgets,
                            user_message.sequence,
                            &trace,
                        ) {
                            replay_messages = messages;
                            provider_messages = build_provider_messages(
                                &replay_messages,
                                &model_message,
                                &user_images,
                                &prompt_context,
                            );
                            recovery_span.finish(
                                "retrying",
                                json!({ "replay_message_count": replay_messages.len() }),
                            );
                            overflow_recovery_retry_sent = true;
                            continue;
                        }
                    } else {
                        trace.event_status(
                            "context.overflow_recovery.summary",
                            "skipped",
                            json!({ "reason": "budget_unavailable" }),
                        );
                    }
                    recovery_span.finish("unavailable", json!({}));
                }
                let error = stream_error.error;
                let user_error_message =
                    generation_failure_user_message(&error, overflow_recovery_retry_sent);
                if error.code() == "context_overflow" {
                    trace.warn(
                        "context.overflow_recovery.exhausted",
                        "failed",
                        json!({
                            "recovery_retry_sent": overflow_recovery_retry_sent,
                            "model": model.as_str(),
                            "estimated_input_tokens": estimate_provider_input_tokens(
                                &provider_settings,
                                &model,
                                &reasoning_effort,
                                &provider_messages,
                            ),
                            "provider_usable_input_tokens": summary_budgets
                                .map(|budgets| budgets.provider_usable_input_tokens),
                            "tail_budget_tokens": summary_budgets
                                .map(|budgets| budgets.tail_budget_tokens),
                            "summary_replay_budget_tokens": summary_budgets
                                .map(|budgets| budgets.summary_replay_budget_tokens),
                        }),
                    );
                }
                let error_text = format!("Request failed: {user_error_message}");
                let error_json = json!({
                    "code": error.code(),
                    "message": user_error_message.as_str()
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
                return send_error(sender, request_id, error.code(), &user_error_message).await;
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
            allow_tool_image_followups,
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

fn summary_budgets_for_model(
    provider_settings: &providers::ProviderSettings,
    model: &str,
    reasoning_effort: &str,
    trace: &trace::TraceRecorder,
) -> Option<context_compaction::SummaryBudgets> {
    match providers::model_context_estimate(provider_settings, model, reasoning_effort) {
        Ok(estimate) => {
            if let Some(usable) = estimate.usable_input_tokens {
                Some(
                    context_compaction::SummaryBudgets::from_model_context(
                        usable,
                        estimate.raw_context_tokens,
                    )
                    .with_model_output_limit(estimate.max_output_tokens),
                )
            } else {
                let local_unknown_context = is_unknown_local_context_model(model);
                let fallback_context_tokens = if local_unknown_context {
                    context_compaction::SummaryBudgets::unknown_local_context_fallback_tokens()
                } else {
                    context_compaction::SummaryBudgets::unknown_context_fallback_tokens()
                };
                trace.warn(
                    "context.summary.unknown_model_limit",
                    "fallback",
                    json!({
                        "fallback_context_tokens": fallback_context_tokens,
                        "local_unknown_context": local_unknown_context
                    }),
                );
                Some(if local_unknown_context {
                    context_compaction::SummaryBudgets::for_unknown_local_context()
                } else {
                    context_compaction::SummaryBudgets::for_unknown_context()
                })
            }
        }
        Err(error) => {
            trace.warn(
                "context.summary.budget_unavailable",
                "skipped",
                json!({ "error_code": error.code(), "message": error.user_message() }),
            );
            None
        }
    }
}

fn is_unknown_local_context_model(model: &str) -> bool {
    matches!(provider_for_model(model), "ollama" | "local" | "lmstudio")
}

fn bounded_replay_after_summary_failure(
    chat_id: &str,
    budgets: context_compaction::SummaryBudgets,
    trace: &trace::TraceRecorder,
) -> Option<Vec<Value>> {
    bounded_replay_after_summary_failure_before_sequence_optional(chat_id, budgets, None, trace)
}

fn bounded_replay_after_summary_failure_before_sequence(
    chat_id: &str,
    budgets: context_compaction::SummaryBudgets,
    before_sequence: i64,
    trace: &trace::TraceRecorder,
) -> Option<Vec<Value>> {
    bounded_replay_after_summary_failure_before_sequence_optional(
        chat_id,
        budgets,
        Some(before_sequence),
        trace,
    )
}

fn bounded_replay_after_summary_failure_before_sequence_optional(
    chat_id: &str,
    budgets: context_compaction::SummaryBudgets,
    before_sequence: Option<i64>,
    trace: &trace::TraceRecorder,
) -> Option<Vec<Value>> {
    let fallback_span = trace.start_span(
        "context.summary.fallback_replay",
        json!({
            "chat_id": chat_id,
            "before_sequence": before_sequence,
            "tail_budget_tokens": budgets.tail_budget_tokens,
            "summary_replay_budget_tokens": budgets.summary_replay_budget_tokens
        }),
    );
    let replay = if let Some(before_sequence) = before_sequence {
        store::replay_messages_with_summary_and_exact_tail_budget_before_sequence(
            chat_id,
            budgets.summary_replay_budget_tokens,
            budgets.tail_budget_tokens,
            before_sequence,
        )
    } else {
        store::replay_messages_with_summary_and_exact_tail_budget(
            chat_id,
            budgets.summary_replay_budget_tokens,
            budgets.tail_budget_tokens,
        )
    };
    match replay {
        Ok(messages) => {
            fallback_span.finish("ok", json!({ "message_count": messages.len() }));
            Some(messages)
        }
        Err(error) => {
            fallback_span.fail(json!({ "message": error.as_str() }));
            trace.warn(
                "context.summary.fallback_replay_failed",
                "failed",
                json!({ "message": error.as_str() }),
            );
            None
        }
    }
}

fn should_retry_context_overflow(error: &AssistantStreamError, retry_used: bool) -> bool {
    !retry_used && !error.emitted_output && error.error.code() == "context_overflow"
}

fn generation_failure_user_message(
    error: &providers::LlmError,
    overflow_recovery_retry_sent: bool,
) -> String {
    if error.code() != "context_overflow" || !overflow_recovery_retry_sent {
        return error.user_message();
    }

    format!(
        "{}\n\nProvider detail: {}",
        "The recent conversation is still larger than the selected model's context window after Fennara summarized older history and retried. Use a model with a larger context length, reduce the current message/context snippets/images, or split/clear the recent output.",
        error.user_message()
    )
}

fn replay_messages_for_budget(
    chat_id: &str,
    budgets: Option<context_compaction::SummaryBudgets>,
) -> Result<Vec<Value>, String> {
    if let Some(budgets) = budgets {
        store::replay_messages_with_summary_budget(chat_id, budgets.summary_replay_budget_tokens)
    } else {
        store::replay_messages(chat_id)
    }
}

fn estimate_provider_input_tokens(
    provider_settings: &providers::ProviderSettings,
    model: &str,
    reasoning_effort: &str,
    provider_messages: &[Value],
) -> Option<u32> {
    providers::estimate_chat_context(
        provider_settings,
        &providers::ChatRequest {
            model: model.to_string(),
            reasoning_effort: reasoning_effort.to_string(),
            messages: provider_messages.to_vec(),
            tools: tools::definitions(),
            max_output_tokens: None,
        },
    )
    .ok()
    .map(|estimate| estimate.estimated_input_tokens)
}

async fn send_context_compaction_status<S>(
    sender: &mut S,
    request_id: Option<String>,
    chat_id: &str,
    status: &str,
) -> Result<(), S::Error>
where
    S: Sink<Message> + Unpin,
    S::Error: std::fmt::Debug,
{
    send_json(
        sender,
        json!({
            "type": "chat_context_compaction",
            "request_id": request_id,
            "chat_id": chat_id,
            "status": status
        }),
    )
    .await
}

async fn try_create_context_summary(
    provider_settings: providers::ProviderSettings,
    model: &str,
    reasoning_effort: &str,
    chat_id: &str,
    budgets: context_compaction::SummaryBudgets,
    trigger_estimated_tokens: u32,
    before_sequence: Option<i64>,
) -> Result<Option<context_compaction::ContextSummaryChunk>, String> {
    let candidate = if let Some(before_sequence) = before_sequence {
        store::context_summary_candidate_before_sequence(
            chat_id,
            budgets.tail_budget_tokens,
            before_sequence,
        )?
    } else {
        store::context_summary_candidate(chat_id, budgets.tail_budget_tokens)?
    };
    let Some(candidate) = candidate else {
        return Ok(None);
    };
    let summary_messages = context_compaction::build_summary_messages(&candidate);
    let summary_output_max_tokens = budgets.summary_output_max_tokens();
    let generation_id = ids::new_id("ctxgen");
    let prompt_estimate = providers::estimate_chat_context(
        &provider_settings,
        &providers::ChatRequest {
            model: model.to_string(),
            reasoning_effort: reasoning_effort.to_string(),
            messages: summary_messages.clone(),
            tools: Vec::new(),
            max_output_tokens: Some(summary_output_max_tokens),
        },
    )
    .ok()
    .map(|estimate| estimate.estimated_input_tokens)
    .unwrap_or(0);
    let available_prompt_tokens = budgets
        .provider_usable_input_tokens
        .saturating_sub(summary_output_max_tokens as usize);
    if (prompt_estimate as usize) > available_prompt_tokens {
        return Err(format!(
            "Summary prompt is estimated at {prompt_estimate} input tokens, which exceeds the summary prompt budget of {available_prompt_tokens} tokens."
        ));
    }
    let usage_slot = Arc::new(Mutex::new(None::<Value>));
    let usage_for_stream = Arc::clone(&usage_slot);
    let completion = providers::stream_chat(
        &provider_settings,
        &providers::ChatRequest {
            model: model.to_string(),
            reasoning_effort: reasoning_effort.to_string(),
            messages: summary_messages,
            tools: Vec::new(),
            max_output_tokens: Some(summary_output_max_tokens),
        },
        None,
        move |item| {
            let usage_for_stream = Arc::clone(&usage_for_stream);
            async move {
                if let providers::StreamItem::Usage(usage) = item {
                    *usage_for_stream.lock().await = Some(usage);
                }
                Ok(true)
            }
        },
    )
    .await
    .map_err(|error| error.user_message())?;

    if !is_clean_context_summary_finish_reason(&completion.finish_reason) {
        return Err(format!(
            "Summary model did not complete cleanly: {:?}",
            completion.finish_reason
        ));
    }
    let summary_markdown = completion.content.trim();
    if summary_markdown.is_empty() {
        return Err("Summary model returned an empty summary.".to_string());
    }
    let usage = usage_slot.lock().await.clone();
    let completion_estimate = summary_markdown.len().max(1).div_ceil(4);
    let metadata = json!({
        "trigger_estimated_tokens": trigger_estimated_tokens,
        "provider_usable_input_tokens": budgets.provider_usable_input_tokens,
        "raw_context_tokens": budgets.raw_context_tokens,
        "compaction_working_budget": budgets.compaction_working_budget,
        "summary_trigger_tokens": budgets.summary_trigger_tokens,
        "tail_budget_tokens": budgets.tail_budget_tokens,
        "summary_replay_budget_tokens": budgets.summary_replay_budget_tokens,
        "summary_prompt_tokens": prompt_estimate,
        "completion_tokens": completion_estimate,
        "total_tokens": prompt_estimate.saturating_add(completion_estimate as u32),
        "source_message_count": candidate.source_message_count,
        "summary_output_max_tokens": summary_output_max_tokens
    });

    store::insert_context_summary(
        chat_id,
        &generation_id,
        summary_markdown,
        &candidate,
        model,
        reasoning_effort,
        usage.as_ref(),
        &metadata,
    )
    .map(Some)
}

fn is_clean_context_summary_finish_reason(reason: &providers::FinishReason) -> bool {
    matches!(reason, providers::FinishReason::Stop)
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

    #[test]
    fn context_summary_finish_reason_must_be_clean_stop() {
        assert!(is_clean_context_summary_finish_reason(
            &providers::FinishReason::Stop
        ));
        assert!(!is_clean_context_summary_finish_reason(
            &providers::FinishReason::Length
        ));
        assert!(!is_clean_context_summary_finish_reason(
            &providers::FinishReason::ContentFilter
        ));
        assert!(!is_clean_context_summary_finish_reason(
            &providers::FinishReason::ToolCalls
        ));
        assert!(!is_clean_context_summary_finish_reason(
            &providers::FinishReason::Cancelled
        ));
        assert!(!is_clean_context_summary_finish_reason(
            &providers::FinishReason::Unknown("weird".to_string())
        ));
    }

    #[test]
    fn context_overflow_after_recovery_gets_specific_user_message() {
        let error = providers::LlmError::ContextOverflow {
            provider: "local".to_string(),
            message: "estimated 100 input tokens, usable 80".to_string(),
        };

        let message = generation_failure_user_message(&error, true);

        assert!(message.contains("after Fennara summarized older history and retried"));
        assert!(message.contains("current message/context snippets/images"));
        assert!(message.contains("Provider detail: estimated 100 input tokens, usable 80"));
    }

    #[test]
    fn context_overflow_without_recovery_keeps_provider_message() {
        let error = providers::LlmError::ContextOverflow {
            provider: "local".to_string(),
            message: "estimated 100 input tokens, usable 80".to_string(),
        };

        let message = generation_failure_user_message(&error, false);

        assert_eq!(message, "estimated 100 input tokens, usable 80");
    }
}
