pub(super) mod cost;
pub(super) mod publisher;
pub(super) mod request;
pub(super) mod runner;
pub(super) mod tool_loop;

use crate::runtime_daemon::state::AppState;

pub(super) const CHAT_ALREADY_RUNNING_MESSAGE: &str =
    "Chat is already running. Wait or cancel the current turn.";

pub(super) struct ActiveChatTurn {
    state: AppState,
    chat_id: String,
}

pub(super) async fn try_begin_chat_turn(state: &AppState, chat_id: &str) -> Option<ActiveChatTurn> {
    let mut active_turns = state.active_chat_turns.write().await;
    if !active_turns.insert(chat_id.to_string()) {
        return None;
    }
    Some(ActiveChatTurn {
        state: state.clone(),
        chat_id: chat_id.to_string(),
    })
}

impl Drop for ActiveChatTurn {
    fn drop(&mut self) {
        if let Ok(mut active_turns) = self.state.active_chat_turns.try_write() {
            active_turns.remove(&self.chat_id);
            return;
        }

        let state = self.state.clone();
        let chat_id = self.chat_id.clone();
        tokio::spawn(async move {
            state.active_chat_turns.write().await.remove(&chat_id);
        });
    }
}

pub(super) async fn is_chat_cancelled(state: &AppState, chat_id: &str) -> bool {
    state.cancelled_chats.read().await.contains(chat_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::oneshot;

    #[tokio::test]
    async fn active_chat_turn_rejects_second_turn_until_guard_drops() {
        let (shutdown_tx, _shutdown_rx) = oneshot::channel();
        let state = AppState::new(shutdown_tx);

        let first = try_begin_chat_turn(&state, "chat_1").await;
        assert!(first.is_some());
        assert!(try_begin_chat_turn(&state, "chat_1").await.is_none());

        drop(first);

        assert!(try_begin_chat_turn(&state, "chat_1").await.is_some());
    }
}
