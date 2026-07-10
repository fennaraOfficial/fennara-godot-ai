use super::*;

#[test]
fn rollup_message_count_includes_failed_tool_messages() {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(
        "
        CREATE TABLE chats (
          id TEXT PRIMARY KEY,
          title TEXT NOT NULL DEFAULT 'New chat',
          model TEXT NOT NULL,
          reasoning_effort TEXT NOT NULL DEFAULT 'medium',
          total_cost REAL NOT NULL DEFAULT 0,
          latest_prompt_tokens INTEGER NOT NULL DEFAULT 0,
          message_count INTEGER NOT NULL DEFAULT 0,
          archived_at_ms INTEGER,
          created_at_ms INTEGER NOT NULL,
          updated_at_ms INTEGER NOT NULL
        );
        CREATE TABLE chat_messages (
          id TEXT PRIMARY KEY,
          chat_id TEXT NOT NULL,
          role TEXT NOT NULL,
          status TEXT NOT NULL DEFAULT 'done',
          content TEXT NOT NULL DEFAULT '',
          reasoning_content TEXT,
          usage_json TEXT,
          cost REAL,
          sequence INTEGER NOT NULL,
          created_at_ms INTEGER NOT NULL,
          updated_at_ms INTEGER NOT NULL
        );
        CREATE TABLE chat_usage_logs (
          id TEXT PRIMARY KEY,
          chat_id TEXT NOT NULL,
          agent_type TEXT NOT NULL DEFAULT 'chat',
          prompt_tokens INTEGER NOT NULL DEFAULT 0,
          total_tokens INTEGER NOT NULL DEFAULT 0,
          cost REAL NOT NULL DEFAULT 0,
          created_at_ms INTEGER NOT NULL
        );
        INSERT INTO chats
          (id, model, reasoning_effort, created_at_ms, updated_at_ms)
          VALUES ('chat_1', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1);
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, sequence, created_at_ms, updated_at_ms)
          VALUES
          ('msg_1', 'chat_1', 'user', 'done', 'hello there', 1, 1, 1),
          ('msg_2', 'chat_1', 'assistant', 'done', 'checking', 2, 1, 1),
          ('msg_3', 'chat_1', 'tool', 'failed', 'Tool failed', 3, 1, 1);
        ",
    )
    .unwrap();

    refresh_chat_rollups(&conn, "chat_1").unwrap();

    let message_count: i64 = conn
        .query_row(
            "SELECT message_count FROM chats WHERE id = 'chat_1'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(message_count, 3);
}

#[test]
fn finish_assistant_message_with_tool_calls_persists_usage_and_rollup_atomically() {
    let mut conn = Connection::open_in_memory().unwrap();
    create_tool_persistence_schema(&conn);
    conn.execute_batch(
        "
        INSERT INTO chats
          (id, title, model, reasoning_effort, created_at_ms, updated_at_ms)
          VALUES ('chat_1', 'Chat', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1);
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, sequence, created_at_ms, updated_at_ms)
          VALUES
          ('msg_user', 'chat_1', 'user', 'done', 'hello there', 1, 1, 1),
          ('msg_assistant', 'chat_1', 'assistant', 'in_progress', '', 2, 1, 1);
        ",
    )
    .unwrap();
    let tool_calls = json!([{
        "id": "call_1",
        "type": "function",
        "function": {
            "name": "read_file",
            "arguments": "{\"path\":\"res://player.gd\"}"
        }
    }]);
    let usage = json!({
        "prompt_tokens": 10,
        "completion_tokens": 5,
        "total_tokens": 15,
        "cost": 0.25
    });

    let message = finish_assistant_message_on_connection(
        &mut conn,
        "msg_assistant",
        Some(&tool_calls),
        "I need to inspect a file.",
        Some("thinking"),
        Some(&usage),
        "openrouter/z-ai/glm-5.2",
        Some("gen_1"),
    )
    .unwrap();

    assert_eq!(message.status, "done");
    assert_eq!(message.content, "I need to inspect a file.");
    assert_eq!(message.reasoning_content.as_deref(), Some("thinking"));
    assert_eq!(message.cost, Some(0.25));
    assert_eq!(
        serde_json::from_str::<Value>(message.tool_calls_json.as_deref().unwrap()).unwrap(),
        tool_calls
    );

    let usage_row: (String, i64, i64, f64) = conn
        .query_row(
            "SELECT generation_id, prompt_tokens, total_tokens, cost
             FROM chat_usage_logs
             WHERE assistant_message_id = 'msg_assistant'",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .unwrap();
    assert_eq!(usage_row, ("gen_1".to_string(), 10, 15, 0.25));

    let rollup: (i64, f64, i64) = conn
        .query_row(
            "SELECT message_count, total_cost, latest_prompt_tokens FROM chats WHERE id = 'chat_1'",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .unwrap();
    assert_eq!(rollup, (2, 0.25, 10));
}

#[test]
fn context_summary_usage_does_not_replace_latest_chat_prompt_tokens() {
    let conn = Connection::open_in_memory().unwrap();
    create_tool_persistence_schema(&conn);
    conn.execute_batch(
        "
        INSERT INTO chats
          (id, title, model, reasoning_effort, created_at_ms, updated_at_ms)
          VALUES ('chat_1', 'Chat', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1);
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, sequence, created_at_ms, updated_at_ms)
          VALUES ('msg_user', 'chat_1', 'user', 'done', 'hello', 1, 1, 1);
        INSERT INTO chat_usage_logs
          (id, chat_id, assistant_message_id, generation_id, model, agent_type,
           prompt_tokens, completion_tokens, total_tokens, cost, created_at_ms)
          VALUES
          ('usage_chat', 'chat_1', NULL, 'gen_chat', 'openrouter/z-ai/glm-5.2', 'chat',
           42, 5, 47, 0.10, 1),
          ('usage_summary', 'chat_1', NULL, 'ctxgen_1', 'openrouter/z-ai/glm-5.2', 'context_summary',
           999, 10, 1009, 0.20, 2);
        ",
    )
    .unwrap();

    refresh_chat_rollups(&conn, "chat_1").unwrap();

    let latest_prompt_tokens: i64 = conn
        .query_row(
            "SELECT latest_prompt_tokens FROM chats WHERE id = 'chat_1'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(latest_prompt_tokens, 42);
}

#[test]
fn assistant_placeholder_and_generation_start_atomically() {
    let mut conn = Connection::open_in_memory().unwrap();
    create_tool_persistence_schema(&conn);
    conn.execute(
        "INSERT INTO chats
          (id, title, model, reasoning_effort, created_at_ms, updated_at_ms)
          VALUES ('chat_1', 'Chat', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1)",
        [],
    )
    .unwrap();

    let (message, generation) =
        generations::insert_assistant_placeholder_with_generation_on_connection(
            &mut conn,
            "chat_1",
            "openrouter/z-ai/glm-5.2",
            "medium",
        )
        .unwrap();

    assert_eq!(message.role, "assistant");
    assert_eq!(message.status, "in_progress");
    let stored_generation: (String, String, String) = conn
        .query_row(
            "SELECT id, assistant_message_id, status
             FROM chat_generations
             WHERE id = ?1",
            [generation.id.as_str()],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .unwrap();
    assert_eq!(
        stored_generation,
        (generation.id, message.id.clone(), "running".to_string())
    );
    let stored_message_generation: String = conn
        .query_row(
            "SELECT generation_id FROM chat_messages WHERE id = ?1",
            [message.id.as_str()],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(stored_message_generation, stored_generation.0);
}

#[test]
fn finish_tool_call_with_message_persists_tool_result_atomically() {
    let mut conn = Connection::open_in_memory().unwrap();
    create_tool_persistence_schema(&conn);
    conn.execute_batch(
        "
        INSERT INTO chats
          (id, title, model, reasoning_effort, created_at_ms, updated_at_ms)
          VALUES ('chat_1', 'Chat', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1);
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, sequence, created_at_ms, updated_at_ms)
          VALUES ('msg_assistant', 'chat_1', 'assistant', 'done', 'checking', 1, 1, 1);
        INSERT INTO chat_tool_calls
          (id, chat_id, assistant_message_id, tool_name, arguments_json, status, created_at_ms, updated_at_ms)
          VALUES ('call_1', 'chat_1', 'msg_assistant', 'read_file', '{}', 'in_progress', 1, 1);
        ",
    )
    .unwrap();

    let message = tool_calls::finish_tool_call_with_message_on_connection(
        &mut conn,
        "chat_1",
        "call_1",
        "read_file",
        "done",
        &json!({ "success": true }),
        "model result",
        "display result",
        &json!({ "status": "done" }),
        &["res://player.gd".to_string()],
    )
    .unwrap();

    assert_eq!(message.role, "tool");
    assert_eq!(message.status, "done");
    assert_eq!(message.content, "display result");
    assert_eq!(message.tool_call_id.as_deref(), Some("call_1"));
    assert_eq!(message.tool_name.as_deref(), Some("read_file"));
    assert_eq!(message.sequence, 2);

    let stored: (String, String, String) = conn
        .query_row(
            "SELECT status, mcp_markdown, plugin_markdown FROM chat_tool_calls WHERE id = 'call_1'",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .unwrap();
    assert_eq!(
        stored,
        (
            "done".to_string(),
            "model result".to_string(),
            "display result".to_string()
        )
    );

    let message_count: i64 = conn
        .query_row(
            "SELECT message_count FROM chats WHERE id = 'chat_1'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(message_count, 2);
}

#[test]
fn finish_tool_call_with_message_rolls_back_tool_update_when_message_insert_fails() {
    let mut conn = Connection::open_in_memory().unwrap();
    create_tool_persistence_schema(&conn);
    conn.execute_batch(
        "
        INSERT INTO chat_tool_calls
          (id, chat_id, assistant_message_id, tool_name, arguments_json, status, created_at_ms, updated_at_ms)
          VALUES ('call_1', 'missing_chat', 'msg_assistant', 'read_file', '{}', 'in_progress', 1, 1);
        ",
    )
    .unwrap();

    let error = tool_calls::finish_tool_call_with_message_on_connection(
        &mut conn,
        "missing_chat",
        "call_1",
        "read_file",
        "done",
        &json!({ "success": true }),
        "model result",
        "display result",
        &json!({ "status": "done" }),
        &[],
    )
    .unwrap_err();

    assert_eq!(error, "Chat not found.");
    let stored: (String, Option<String>, Option<String>) = conn
        .query_row(
            "SELECT status, mcp_markdown, plugin_markdown FROM chat_tool_calls WHERE id = 'call_1'",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .unwrap();
    assert_eq!(stored, ("in_progress".to_string(), None, None));

    let message_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM chat_messages", [], |row| row.get(0))
        .unwrap();
    assert_eq!(message_count, 0);
}

#[test]
fn internal_tool_call_ids_allow_same_provider_id_across_chats() {
    let conn = Connection::open_in_memory().unwrap();
    create_tool_persistence_schema(&conn);
    conn.execute_batch(
        "
        INSERT INTO chats
          (id, title, model, reasoning_effort, created_at_ms, updated_at_ms)
          VALUES
          ('chat_1', 'Chat 1', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1),
          ('chat_2', 'Chat 2', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1);
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, sequence, created_at_ms, updated_at_ms)
          VALUES
          ('msg_a', 'chat_1', 'assistant', 'done', 'checking', 1, 1, 1),
          ('msg_b', 'chat_2', 'assistant', 'done', 'checking', 1, 1, 1);
        ",
    )
    .unwrap();

    tool_calls::upsert_tool_call_on_connection(
        &conn,
        "chat_1",
        "msg_a",
        Some("gen_a"),
        "call_a",
        Some("tool_call_0"),
        "read_file",
        &json!({ "path": "res://a.gd" }),
        "in_progress",
    )
    .unwrap();
    tool_calls::upsert_tool_call_on_connection(
        &conn,
        "chat_2",
        "msg_b",
        Some("gen_b"),
        "call_b",
        Some("tool_call_0"),
        "read_file",
        &json!({ "path": "res://b.gd" }),
        "in_progress",
    )
    .unwrap();

    let rows = conn
        .prepare(
            "SELECT id, provider_tool_call_id, chat_id, generation_id
             FROM chat_tool_calls
             ORDER BY chat_id",
        )
        .unwrap()
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assert_eq!(
        rows,
        vec![
            (
                "call_a".to_string(),
                "tool_call_0".to_string(),
                "chat_1".to_string(),
                "gen_a".to_string()
            ),
            (
                "call_b".to_string(),
                "tool_call_0".to_string(),
                "chat_2".to_string(),
                "gen_b".to_string()
            )
        ]
    );
}

#[test]
fn internal_tool_call_ids_allow_same_provider_id_across_generations() {
    let conn = Connection::open_in_memory().unwrap();
    create_tool_persistence_schema(&conn);
    conn.execute_batch(
        "
        INSERT INTO chats
          (id, title, model, reasoning_effort, created_at_ms, updated_at_ms)
          VALUES ('chat_1', 'Chat', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1);
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, sequence, created_at_ms, updated_at_ms)
          VALUES
          ('msg_a', 'chat_1', 'assistant', 'done', 'checking', 1, 1, 1),
          ('msg_b', 'chat_1', 'assistant', 'done', 'checking again', 2, 1, 1);
        ",
    )
    .unwrap();

    tool_calls::upsert_tool_call_on_connection(
        &conn,
        "chat_1",
        "msg_a",
        Some("gen_a"),
        "call_a",
        Some("tool_call_0"),
        "read_file",
        &json!({ "path": "res://a.gd" }),
        "done",
    )
    .unwrap();
    tool_calls::upsert_tool_call_on_connection(
        &conn,
        "chat_1",
        "msg_b",
        Some("gen_b"),
        "call_b",
        Some("tool_call_0"),
        "read_file",
        &json!({ "path": "res://b.gd" }),
        "done",
    )
    .unwrap();

    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM chat_tool_calls
             WHERE provider_tool_call_id = 'tool_call_0'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(count, 2);
}

#[test]
fn replay_uses_internal_tool_call_ids_for_assistant_and_tool_messages() {
    let conn = Connection::open_in_memory().unwrap();
    create_tool_persistence_schema(&conn);
    conn.execute_batch(
        "
        INSERT INTO chats
          (id, title, model, reasoning_effort, created_at_ms, updated_at_ms)
          VALUES ('chat_1', 'Chat', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1);
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, tool_calls_json, sequence, created_at_ms, updated_at_ms)
          VALUES (
            'msg_assistant',
            'chat_1',
            'assistant',
            'done',
            'checking',
            '[{\"id\":\"call_internal\",\"type\":\"function\",\"function\":{\"name\":\"read_file\",\"arguments\":\"{}\"}}]',
            1,
            1,
            1
          );
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, tool_call_id, tool_name, sequence, created_at_ms, updated_at_ms)
          VALUES (
            'msg_tool',
            'chat_1',
            'tool',
            'done',
            'display result',
            'call_internal',
            'read_file',
            2,
            1,
            1
          );
        INSERT INTO chat_tool_calls
          (id, provider_tool_call_id, chat_id, assistant_message_id, tool_name, arguments_json, status,
           raw_result_json, mcp_markdown, plugin_markdown, created_at_ms, updated_at_ms)
          VALUES (
            'call_internal',
            'tool_call_0',
            'chat_1',
            'msg_assistant',
            'read_file',
            '{}',
            'done',
            '{\"files\":[]}',
            'model result',
            'display result',
            1,
            1
          );
        ",
    )
    .unwrap();

    let replay = replay::replay_messages_from_conn(&conn, "chat_1").unwrap();

    assert_eq!(replay.len(), 2);
    assert_eq!(replay[0]["tool_calls"][0]["id"], "call_internal");
    assert!(
        replay[0]["tool_calls"][0]
            .get("provider_tool_call_id")
            .is_none()
    );
    assert_eq!(replay[1]["tool_call_id"], "call_internal");
}

#[test]
fn replay_drops_incomplete_assistant_tool_call_groups() {
    let conn = Connection::open_in_memory().unwrap();
    create_tool_persistence_schema(&conn);
    conn.execute_batch(
        "
        INSERT INTO chats
          (id, title, model, reasoning_effort, created_at_ms, updated_at_ms)
          VALUES ('chat_1', 'Chat', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1);
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, sequence, created_at_ms, updated_at_ms)
          VALUES ('msg_user', 'chat_1', 'user', 'done', 'hello', 1, 1, 1);
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, tool_calls_json, sequence, created_at_ms, updated_at_ms)
          VALUES (
            'msg_assistant',
            'chat_1',
            'assistant',
            'done',
            'checking',
            '[{\"id\":\"call_missing\",\"type\":\"function\",\"function\":{\"name\":\"read_file\",\"arguments\":\"{}\"}}]',
            2,
            1,
            1
          );
        ",
    )
    .unwrap();

    let replay = replay::replay_messages_from_conn(&conn, "chat_1").unwrap();

    assert_eq!(replay.len(), 1);
    assert_eq!(replay[0]["role"], "user");
    assert_eq!(replay[0]["content"], "hello");
}

#[test]
fn replay_keeps_history_without_message_count_cap() {
    let conn = Connection::open_in_memory().unwrap();
    create_tool_persistence_schema(&conn);
    conn.execute_batch(
        "
        INSERT INTO chats
          (id, title, model, reasoning_effort, created_at_ms, updated_at_ms)
          VALUES ('chat_1', 'Chat', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1);
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, tool_calls_json, sequence, created_at_ms, updated_at_ms)
          VALUES (
            'msg_assistant',
            'chat_1',
            'assistant',
            'done',
            'checking',
            '[{\"id\":\"call_1\",\"type\":\"function\",\"function\":{\"name\":\"read_file\",\"arguments\":\"{}\"}}]',
            1,
            1,
            1
          );
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, tool_call_id, tool_name, sequence, created_at_ms, updated_at_ms)
          VALUES ('msg_tool', 'chat_1', 'tool', 'done', 'display result', 'call_1', 'read_file', 2, 1, 1);
        INSERT INTO chat_tool_calls
          (id, provider_tool_call_id, chat_id, assistant_message_id, tool_name, arguments_json, status,
           raw_result_json, mcp_markdown, plugin_markdown, created_at_ms, updated_at_ms)
          VALUES (
            'call_1',
            'tool_call_0',
            'chat_1',
            'msg_assistant',
            'read_file',
            '{}',
            'done',
            '{\"files\":[]}',
            'model result',
            'display result',
            1,
            1
          );
        ",
    )
    .unwrap();
    for index in 0..39 {
        let sequence = index + 3;
        conn.execute(
            "INSERT INTO chat_messages
             (id, chat_id, role, status, content, sequence, created_at_ms, updated_at_ms)
             VALUES (?1, 'chat_1', 'user', 'done', ?2, ?3, 1, 1)",
            params![
                format!("msg_user_{index}"),
                format!("later message {index}"),
                sequence
            ],
        )
        .unwrap();
    }

    let replay = replay::replay_messages_from_conn(&conn, "chat_1").unwrap();

    assert_eq!(replay.len(), 41);
    assert_eq!(replay[0]["role"], "assistant");
    assert_eq!(replay[0]["tool_calls"][0]["id"], "call_1");
    assert_eq!(replay[1]["role"], "tool");
    assert_eq!(replay[1]["tool_call_id"], "call_1");
    assert_eq!(replay[40]["role"], "user");
    assert_eq!(replay[40]["content"], "later message 38");
}

#[test]
fn replay_summary_injects_checkpoint_then_exact_tail() {
    let conn = Connection::open_in_memory().unwrap();
    create_tool_persistence_schema(&conn);
    conn.execute_batch(
        "
        INSERT INTO chats
          (id, title, model, reasoning_effort, created_at_ms, updated_at_ms)
          VALUES ('chat_1', 'Chat', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1);
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, sequence, created_at_ms, updated_at_ms)
          VALUES
          ('msg_1', 'chat_1', 'user', 'done', 'old user', 1, 1, 1),
          ('msg_2', 'chat_1', 'assistant', 'done', 'old answer', 2, 1, 1),
          ('msg_3', 'chat_1', 'user', 'done', 'tail user', 3, 1, 1),
          ('msg_4', 'chat_1', 'assistant', 'done', 'tail answer', 4, 1, 1);
        INSERT INTO chat_context_summaries
          (id, chat_id, generation_id, summary_markdown,
           covered_start_message_id, covered_start_sequence,
           covered_end_message_id, covered_end_sequence,
           tail_start_message_id, tail_start_sequence,
           source_message_count, model, reasoning_effort, created_at_ms)
          VALUES
          ('sum_1', 'chat_1', 'ctxgen_1', 'summary of old work',
           'msg_1', 1, 'msg_2', 2, 'msg_3', 3, 2,
           'openrouter/z-ai/glm-5.2', 'medium', 1);
        ",
    )
    .unwrap();

    let replay =
        replay::replay_messages_with_summary_budget_from_conn(&conn, "chat_1", Some(64_000))
            .unwrap();

    assert_eq!(replay.len(), 3);
    assert_eq!(replay[0]["role"], "user");
    assert!(
        replay[0]["content"]
            .as_str()
            .unwrap()
            .contains("<conversation-checkpoint>")
    );
    assert!(
        replay[0]["content"]
            .as_str()
            .unwrap()
            .contains("summary of old work")
    );
    assert_eq!(replay[1]["content"], "tail user");
    assert_eq!(replay[2]["content"], "tail answer");
}

#[test]
fn replay_summary_budget_keeps_exact_history_when_legacy_chunks_do_not_fit() {
    let conn = Connection::open_in_memory().unwrap();
    create_tool_persistence_schema(&conn);
    conn.execute_batch(
        "
        INSERT INTO chats
          (id, title, model, reasoning_effort, created_at_ms, updated_at_ms)
          VALUES ('chat_1', 'Chat', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1);
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, sequence, created_at_ms, updated_at_ms)
          VALUES
          ('msg_1', 'chat_1', 'user', 'done', 'one', 1, 1, 1),
          ('msg_2', 'chat_1', 'assistant', 'done', 'two', 2, 1, 1),
          ('msg_3', 'chat_1', 'user', 'done', 'three', 3, 1, 1),
          ('msg_4', 'chat_1', 'assistant', 'done', 'four', 4, 1, 1),
          ('msg_5', 'chat_1', 'user', 'done', 'tail', 5, 1, 1);
        INSERT INTO chat_context_summaries
          (id, chat_id, generation_id, summary_markdown,
           covered_start_message_id, covered_start_sequence,
           covered_end_message_id, covered_end_sequence,
           tail_start_message_id, tail_start_sequence,
           source_message_count, model, reasoning_effort, created_at_ms)
          VALUES
          ('sum_1', 'chat_1', 'ctxgen_1', 'old summary text',
           'msg_1', 1, 'msg_2', 2, 'msg_3', 3, 2,
           'openrouter/z-ai/glm-5.2', 'medium', 1),
          ('sum_2', 'chat_1', 'ctxgen_2', 'new summary text',
           'msg_3', 3, 'msg_4', 4, 'msg_5', 5, 2,
           'openrouter/z-ai/glm-5.2', 'medium', 2);
        ",
    )
    .unwrap();

    let replay =
        replay::replay_messages_with_summary_budget_from_conn(&conn, "chat_1", Some(5)).unwrap();

    assert_eq!(replay.len(), 5);
    assert_eq!(replay[0]["content"], "one");
    assert_eq!(replay[1]["content"], "two");
    assert_eq!(replay[2]["content"], "three");
    assert_eq!(replay[3]["content"], "four");
    assert_eq!(replay[4]["content"], "tail");
    let stored_summary_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM chat_context_summaries", [], |row| {
            row.get(0)
        })
        .unwrap();
    assert_eq!(stored_summary_count, 2);
}

#[test]
fn replay_summary_budget_can_use_latest_cumulative_summary_by_itself() {
    let conn = Connection::open_in_memory().unwrap();
    create_tool_persistence_schema(&conn);
    conn.execute_batch(
        "
        INSERT INTO chats
          (id, title, model, reasoning_effort, created_at_ms, updated_at_ms)
          VALUES ('chat_1', 'Chat', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1);
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, sequence, created_at_ms, updated_at_ms)
          VALUES
          ('msg_1', 'chat_1', 'user', 'done', 'one', 1, 1, 1),
          ('msg_2', 'chat_1', 'assistant', 'done', 'two', 2, 1, 1),
          ('msg_3', 'chat_1', 'user', 'done', 'three', 3, 1, 1),
          ('msg_4', 'chat_1', 'assistant', 'done', 'four', 4, 1, 1),
          ('msg_5', 'chat_1', 'user', 'done', 'tail', 5, 1, 1);
        INSERT INTO chat_context_summaries
          (id, chat_id, generation_id, summary_markdown,
           covered_start_message_id, covered_start_sequence,
           covered_end_message_id, covered_end_sequence,
           tail_start_message_id, tail_start_sequence,
           source_message_count, model, reasoning_effort, created_at_ms)
          VALUES
          ('sum_1', 'chat_1', 'ctxgen_1', 'old summary text',
           'msg_1', 1, 'msg_2', 2, 'msg_3', 3, 2,
           'openrouter/z-ai/glm-5.2', 'medium', 1),
          ('sum_2', 'chat_1', 'ctxgen_2', 'updated cumulative summary text',
           'msg_1', 1, 'msg_4', 4, 'msg_5', 5, 4,
           'openrouter/z-ai/glm-5.2', 'medium', 2);
        ",
    )
    .unwrap();

    let replay =
        replay::replay_messages_with_summary_budget_from_conn(&conn, "chat_1", Some(5)).unwrap();
    let summary = replay[0]["content"].as_str().unwrap();

    assert_eq!(replay.len(), 2);
    assert!(summary.contains("updated cumulative summary text"));
    assert_eq!(replay[1]["content"], "tail");
}

#[test]
fn bounded_replay_before_sequence_excludes_current_user_prompt() {
    let conn = Connection::open_in_memory().unwrap();
    create_tool_persistence_schema(&conn);
    conn.execute_batch(
        "
        INSERT INTO chats
          (id, title, model, reasoning_effort, created_at_ms, updated_at_ms)
          VALUES ('chat_1', 'Chat', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1);
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, sequence, created_at_ms, updated_at_ms)
          VALUES
          ('msg_1', 'chat_1', 'user', 'done', 'old user', 1, 1, 1),
          ('msg_2', 'chat_1', 'assistant', 'done', 'old answer', 2, 1, 1),
          ('msg_3', 'chat_1', 'user', 'done', 'current prompt', 3, 1, 1),
          ('msg_4', 'chat_1', 'assistant', 'in_progress', '', 4, 1, 1);
        INSERT INTO chat_context_summaries
          (id, chat_id, generation_id, summary_markdown,
           covered_start_message_id, covered_start_sequence,
           covered_end_message_id, covered_end_sequence,
           tail_start_message_id, tail_start_sequence,
           source_message_count, model, reasoning_effort, created_at_ms)
          VALUES
          ('sum_1', 'chat_1', 'ctxgen_1', 'summary of earlier turns',
           'msg_1', 1, 'msg_2', 2, 'msg_3', 3, 2,
           'openrouter/z-ai/glm-5.2', 'medium', 1);
        ",
    )
    .unwrap();

    let replay =
        replay::replay_messages_with_summary_and_exact_tail_budget_before_sequence_from_conn(
            &conn, "chat_1", 64_000, 64_000, 3,
        )
        .unwrap();

    assert_eq!(replay.len(), 1);
    assert_eq!(replay[0]["role"], "user");
    let checkpoint = replay[0]["content"].as_str().unwrap();
    assert!(checkpoint.contains("summary of earlier turns"));
    assert!(!checkpoint.contains("current prompt"));
}

#[test]
fn raw_summary_groups_keep_rows_that_provider_replay_sanitizes() {
    let conn = Connection::open_in_memory().unwrap();
    create_tool_persistence_schema(&conn);
    conn.execute_batch(
        "
        INSERT INTO chats
          (id, title, model, reasoning_effort, created_at_ms, updated_at_ms)
          VALUES ('chat_1', 'Chat', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1);
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, sequence, created_at_ms, updated_at_ms)
          VALUES
          ('msg_user_old', 'chat_1', 'user', 'done', 'old user fact', 1, 1, 1),
          ('msg_tool_orphan', 'chat_1', 'tool', 'done', 'standalone tool fact', 3, 1, 1);
        ",
    )
    .unwrap();
    conn.execute(
        "INSERT INTO chat_messages
         (id, chat_id, role, status, content, tool_calls_json, sequence, created_at_ms, updated_at_ms)
         VALUES ('msg_assistant_missing_tool', 'chat_1', 'assistant', 'done', 'called missing tool', ?1, 2, 1, 1)",
        params![json!([{
            "id": "call_missing",
            "type": "function",
            "function": { "name": "read_file", "arguments": "{}" }
        }])
        .to_string()],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO chat_messages
         (id, chat_id, role, status, content, sequence, created_at_ms, updated_at_ms)
         VALUES ('msg_user_tail', 'chat_1', 'user', 'done', ?1, 4, 1, 1)",
        params!["tail ".repeat(200)],
    )
    .unwrap();

    let provider_groups = replay::replay_groups_from_conn(&conn, "chat_1").unwrap();
    assert!(
        !provider_groups
            .iter()
            .flat_map(|group| group.rows.iter())
            .any(|row| row.id == "msg_assistant_missing_tool" || row.id == "msg_tool_orphan")
    );

    let raw_groups = replay::raw_summary_groups_from_conn(&conn, "chat_1").unwrap();
    assert!(
        raw_groups
            .iter()
            .flat_map(|group| group.rows.iter())
            .any(|row| row.id == "msg_assistant_missing_tool")
    );
    assert!(
        raw_groups
            .iter()
            .flat_map(|group| group.rows.iter())
            .any(|row| row.id == "msg_tool_orphan")
    );

    let candidate = context_compaction::select_next_summary_candidate(&raw_groups, &[], 1).unwrap();

    assert_eq!(candidate.covered_start_sequence, 1);
    assert_eq!(candidate.covered_end_sequence, 3);
    assert_eq!(candidate.tail_start_sequence, Some(4));
    assert_eq!(candidate.source_message_count, 3);
}

#[test]
fn raw_summary_tail_does_not_split_assistant_tool_call_group() {
    let mut conn = Connection::open_in_memory().unwrap();
    create_tool_persistence_schema(&conn);
    conn.execute_batch(
        "
        INSERT INTO chats
          (id, title, model, reasoning_effort, created_at_ms, updated_at_ms)
          VALUES ('chat_1', 'Chat', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1);
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, sequence, created_at_ms, updated_at_ms)
          VALUES ('msg_user_old', 'chat_1', 'user', 'done', 'old user fact', 1, 1, 1);
        ",
    )
    .unwrap();
    let large_result = large_tool_markdown("large exact tool marker");
    insert_replay_tool_turn(
        &conn,
        2,
        "call_large",
        "read_file",
        "res://large.gd",
        &large_result,
    );

    let raw_groups = replay::raw_summary_groups_from_conn(&conn, "chat_1").unwrap();
    assert_eq!(
        raw_groups
            .iter()
            .map(|group| {
                group
                    .rows
                    .iter()
                    .map(|row| row.sequence)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
        vec![vec![1], vec![2, 3]]
    );

    let candidate = context_compaction::select_next_summary_candidate(&raw_groups, &[], 1).unwrap();

    assert_eq!(candidate.covered_start_sequence, 1);
    assert_eq!(candidate.covered_end_sequence, 1);
    assert_eq!(candidate.tail_start_sequence, Some(2));
    assert_eq!(candidate.source_message_count, 1);

    let summary = context_compaction::insert_context_summary_on_connection(
        &mut conn,
        context_compaction::InsertContextSummary {
            chat_id: "chat_1",
            generation_id: "ctxgen_1",
            summary_markdown: "summary before tool call",
            candidate: &candidate,
            model: "openrouter/z-ai/glm-5.2",
            reasoning_effort: "medium",
            usage: None,
            metadata: &json!({}),
        },
    )
    .unwrap();
    assert_eq!(summary.covered_end_sequence, 1);

    let replay =
        replay::replay_messages_with_summary_budget_from_conn(&conn, "chat_1", Some(64_000))
            .unwrap();
    let assistant = replay
        .iter()
        .find(|message| message["role"] == "assistant" && message.get("tool_calls").is_some())
        .unwrap();
    let tool = replay
        .iter()
        .find(|message| message["tool_call_id"] == "call_large")
        .unwrap();

    assert_eq!(assistant["tool_calls"][0]["id"], "call_large");
    assert_eq!(tool["content"], large_result);
}

#[test]
fn raw_summary_includes_stale_in_progress_rows_before_insert_validation() {
    let mut conn = Connection::open_in_memory().unwrap();
    create_tool_persistence_schema(&conn);
    conn.execute_batch(
        "
        INSERT INTO chats
          (id, title, model, reasoning_effort, created_at_ms, updated_at_ms)
          VALUES ('chat_1', 'Chat', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1);
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, sequence, created_at_ms, updated_at_ms)
          VALUES
          ('msg_1', 'chat_1', 'user', 'done', 'old user', 1, 1, 1),
          ('msg_2', 'chat_1', 'assistant', 'in_progress', 'stale partial assistant', 2, 1, 1),
          ('msg_3', 'chat_1', 'assistant', 'done', 'later old assistant', 3, 1, 1),
          ('msg_4', 'chat_1', 'user', 'done', 'tail text tail text tail text tail text', 4, 1, 1);
        ",
    )
    .unwrap();

    let raw_groups = replay::raw_summary_groups_from_conn(&conn, "chat_1").unwrap();
    let candidate = context_compaction::select_next_summary_candidate(&raw_groups, &[], 1).unwrap();

    assert_eq!(candidate.covered_start_sequence, 1);
    assert_eq!(candidate.covered_end_sequence, 3);
    assert_eq!(candidate.source_message_count, 3);

    let summary = context_compaction::insert_context_summary_on_connection(
        &mut conn,
        context_compaction::InsertContextSummary {
            chat_id: "chat_1",
            generation_id: "ctxgen_1",
            summary_markdown: "summary includes stale in-progress row textually",
            candidate: &candidate,
            model: "openrouter/z-ai/glm-5.2",
            reasoning_effort: "medium",
            usage: None,
            metadata: &json!({}),
        },
    )
    .unwrap();

    assert_eq!(summary.source_message_count, 3);
}

fn large_tool_markdown(marker: &str) -> String {
    let mut content = String::with_capacity(120_000);
    content.push_str(marker);
    content.push('\n');
    content.push_str(&"x".repeat(120_000usize.saturating_sub(content.len())));
    content
}

fn insert_replay_tool_turn(
    conn: &Connection,
    sequence: i64,
    call_id: &str,
    tool_name: &str,
    path: &str,
    mcp_markdown: &str,
) {
    let assistant_id = format!("msg_assistant_{call_id}");
    let tool_message_id = format!("msg_tool_{call_id}");
    let tool_calls_json = format!(
        r#"[{{"id":"{call_id}","type":"function","function":{{"name":"{tool_name}","arguments":"{{}}"}}}}]"#
    );

    conn.execute(
        "INSERT INTO chat_messages
         (id, chat_id, role, status, content, tool_calls_json, sequence, created_at_ms, updated_at_ms)
         VALUES (?1, 'chat_1', 'assistant', 'done', ?2, ?3, ?4, 1, 1)",
        params![
            assistant_id,
            format!("calling {tool_name}"),
            tool_calls_json,
            sequence
        ],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO chat_messages
         (id, chat_id, role, status, content, tool_call_id, tool_name, sequence, created_at_ms, updated_at_ms)
         VALUES (?1, 'chat_1', 'tool', 'done', 'display result', ?2, ?3, ?4, 1, 1)",
        params![tool_message_id, call_id, tool_name, sequence + 1],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO chat_tool_calls
         (id, chat_id, assistant_message_id, tool_name, arguments_json, status,
          raw_result_json, mcp_markdown, plugin_markdown, metadata_json, target_keys_json,
          created_at_ms, updated_at_ms)
         VALUES (?1, 'chat_1', ?2, ?3, ?4, 'done', ?5, ?6, 'display result', ?7, ?8, 1, 1)",
        params![
            call_id,
            assistant_id,
            tool_name,
            json!({ "path": path }).to_string(),
            json!({ "status": "success", "path": path }).to_string(),
            mcp_markdown,
            json!({ "status": "success", "targets": [{ "path": path }] }).to_string(),
            json!([path]).to_string()
        ],
    )
    .unwrap();
}

#[test]
fn replay_pressure_prunes_old_tool_results_after_default_token_window() {
    let conn = Connection::open_in_memory().unwrap();
    create_tool_persistence_schema(&conn);
    conn.execute_batch(
        "
        INSERT INTO chats
          (id, title, model, reasoning_effort, created_at_ms, updated_at_ms)
          VALUES ('chat_1', 'Chat', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1);
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, sequence, created_at_ms, updated_at_ms)
          VALUES ('msg_user_old', 'chat_1', 'user', 'done', 'old task', 1, 1, 1);
        ",
    )
    .unwrap();

    let old_a = large_tool_markdown("old A exact marker");
    let old_b = large_tool_markdown("old B exact marker");
    let newest_old = large_tool_markdown("newest old exact marker");
    insert_replay_tool_turn(
        &conn,
        2,
        "call_old_a",
        "read_file",
        "res://old_a.gd",
        &old_a,
    );
    insert_replay_tool_turn(
        &conn,
        4,
        "call_old_b",
        "read_file",
        "res://old_b.gd",
        &old_b,
    );
    insert_replay_tool_turn(
        &conn,
        6,
        "call_newest_old",
        "read_file",
        "res://newest_old.gd",
        &newest_old,
    );
    conn.execute_batch(
        "
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, sequence, created_at_ms, updated_at_ms)
          VALUES
          ('msg_user_tail_1', 'chat_1', 'user', 'done', 'tail one', 8, 1, 1),
          ('msg_assistant_tail_1', 'chat_1', 'assistant', 'done', 'tail answer one', 9, 1, 1),
          ('msg_user_tail_2', 'chat_1', 'user', 'done', 'tail two', 10, 1, 1),
          ('msg_assistant_tail_2', 'chat_1', 'assistant', 'done', 'tail answer two', 11, 1, 1);
        ",
    )
    .unwrap();

    let replay = replay::replay_messages_from_conn(&conn, "chat_1").unwrap();
    let old_tool_a = replay
        .iter()
        .find(|message| message["tool_call_id"] == "call_old_a")
        .unwrap();
    let old_tool_b = replay
        .iter()
        .find(|message| message["tool_call_id"] == "call_old_b")
        .unwrap();
    let newest_old_tool = replay
        .iter()
        .find(|message| message["tool_call_id"] == "call_newest_old")
        .unwrap();

    let old_a_content = old_tool_a["content"].as_str().unwrap();
    assert!(old_a_content.contains("[old tool result omitted: read_file"));
    assert!(old_a_content.contains("target=res://old_a.gd"));
    assert!(old_a_content.contains("status=success"));
    assert!(old_a_content.contains("omitted~"));
    assert!(old_a_content.contains("exact in Fennara history"));
    assert!(!old_a_content.contains("Tool call id"));
    assert!(!old_a_content.contains("newest 40000 estimated tokens"));
    assert!(!old_a_content.contains("rerun the tool"));
    assert!(!old_a_content.contains("old A exact marker"));

    let old_b_content = old_tool_b["content"].as_str().unwrap();
    assert!(old_b_content.contains("[old tool result omitted: read_file"));
    assert!(!old_b_content.contains("old B exact marker"));
    assert_eq!(newest_old_tool["content"], newest_old);

    let stored_old: String = conn
        .query_row(
            "SELECT mcp_markdown FROM chat_tool_calls WHERE id = 'call_old_a'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(stored_old, old_a);
}

#[test]
fn replay_pressure_keeps_latest_two_user_turn_tool_results_exact() {
    let conn = Connection::open_in_memory().unwrap();
    create_tool_persistence_schema(&conn);
    conn.execute_batch(
        "
        INSERT INTO chats
          (id, title, model, reasoning_effort, created_at_ms, updated_at_ms)
          VALUES ('chat_1', 'Chat', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1);
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, sequence, created_at_ms, updated_at_ms)
          VALUES ('msg_user_old', 'chat_1', 'user', 'done', 'old task', 1, 1, 1);
        ",
    )
    .unwrap();

    let old_a = large_tool_markdown("old A exact marker");
    let old_b = large_tool_markdown("old B exact marker");
    let newest_old = large_tool_markdown("newest old exact marker");
    let protected_tail = large_tool_markdown("protected tail exact marker");
    insert_replay_tool_turn(
        &conn,
        2,
        "call_old_a",
        "read_file",
        "res://old_a.gd",
        &old_a,
    );
    insert_replay_tool_turn(
        &conn,
        4,
        "call_old_b",
        "read_file",
        "res://old_b.gd",
        &old_b,
    );
    insert_replay_tool_turn(
        &conn,
        6,
        "call_newest_old",
        "read_file",
        "res://newest_old.gd",
        &newest_old,
    );
    conn.execute(
        "INSERT INTO chat_messages
         (id, chat_id, role, status, content, sequence, created_at_ms, updated_at_ms)
         VALUES ('msg_user_tail_1', 'chat_1', 'user', 'done', 'tail one', 8, 1, 1)",
        [],
    )
    .unwrap();
    insert_replay_tool_turn(
        &conn,
        9,
        "call_tail",
        "read_file",
        "res://tail.gd",
        &protected_tail,
    );
    conn.execute_batch(
        "
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, sequence, created_at_ms, updated_at_ms)
          VALUES
          ('msg_user_tail_2', 'chat_1', 'user', 'done', 'tail two', 11, 1, 1),
          ('msg_assistant_tail_2', 'chat_1', 'assistant', 'done', 'tail answer two', 12, 1, 1);
        ",
    )
    .unwrap();

    let replay = replay::replay_messages_from_conn(&conn, "chat_1").unwrap();
    let old_tool = replay
        .iter()
        .find(|message| message["tool_call_id"] == "call_old_a")
        .unwrap();
    let tail_tool = replay
        .iter()
        .find(|message| message["tool_call_id"] == "call_tail")
        .unwrap();

    assert!(
        old_tool["content"]
            .as_str()
            .unwrap()
            .contains("[old tool result omitted:")
    );
    assert_eq!(tail_tool["content"], protected_tail);
}

#[test]
fn replay_messages_do_not_reconstruct_screenshot_images_from_raw_result() {
    let conn = Connection::open_in_memory().unwrap();
    create_tool_persistence_schema(&conn);
    conn.execute_batch(
        "
        INSERT INTO chats
          (id, title, model, reasoning_effort, created_at_ms, updated_at_ms)
          VALUES ('chat_1', 'Chat', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1);
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, tool_calls_json, sequence, created_at_ms, updated_at_ms)
          VALUES (
            'msg_assistant',
            'chat_1',
            'assistant',
            'done',
            'checking screenshot',
            '[{\"id\":\"call_1\",\"type\":\"function\",\"function\":{\"name\":\"screenshot_scene\",\"arguments\":\"{}\"}}]',
            1,
            1,
            1
          );
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, tool_call_id, tool_name, sequence, created_at_ms, updated_at_ms)
          VALUES ('msg_tool', 'chat_1', 'tool', 'done', 'display result', 'call_1', 'screenshot_scene', 2, 1, 1);
        INSERT INTO chat_tool_calls
          (id, chat_id, assistant_message_id, tool_name, arguments_json, status,
           raw_result_json, mcp_markdown, plugin_markdown, created_at_ms, updated_at_ms)
          VALUES (
            'call_1',
            'chat_1',
            'msg_assistant',
            'screenshot_scene',
            '{}',
            'done',
            '{\"image_base64\":\"abc123\",\"mime_type\":\"image/png\"}',
            'model result',
            'display result',
            1,
            1
          );
        ",
    )
    .unwrap();

    let messages = replay::replay_messages_from_conn(&conn, "chat_1").unwrap();

    assert_eq!(messages.len(), 2);
    assert_eq!(
        messages[0],
        json!({
            "role": "assistant",
            "content": "checking screenshot",
            "tool_calls": [{
                "id": "call_1",
                "type": "function",
                "function": { "name": "screenshot_scene", "arguments": "{}" }
            }]
        })
    );
    assert_eq!(
        messages[1],
        json!({
            "role": "tool",
            "content": "model result",
            "tool_call_id": "call_1",
            "name": "screenshot_scene"
        })
    );
}

#[test]
fn cancel_turn_preserves_user_prompt_and_omits_cancelled_outputs_from_replay() {
    let mut conn = Connection::open_in_memory().unwrap();
    create_tool_persistence_schema(&conn);
    conn.execute_batch(
        "
        INSERT INTO chats
          (id, title, model, reasoning_effort, created_at_ms, updated_at_ms)
          VALUES ('chat_1', 'Chat', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1);
        INSERT INTO chat_messages
          (id, chat_id, role, status, content, tool_call_id, tool_name, sequence, created_at_ms, updated_at_ms)
          VALUES
          ('msg_user_1', 'chat_1', 'user', 'done', 'make a thing', NULL, NULL, 1, 1, 1),
          ('msg_assistant_1', 'chat_1', 'assistant', 'in_progress', 'partial old', NULL, NULL, 2, 1, 1),
          ('msg_tool_1', 'chat_1', 'tool', 'done', 'tool output', 'call_1', 'read_file', 3, 1, 1),
          ('msg_user_2', 'chat_1', 'user', 'done', 'next turn', NULL, NULL, 4, 1, 1),
          ('msg_assistant_2', 'chat_1', 'assistant', 'done', 'next answer', NULL, NULL, 5, 1, 1);
        INSERT INTO chat_tool_calls
          (id, chat_id, assistant_message_id, tool_name, arguments_json, status,
           raw_result_json, mcp_markdown, plugin_markdown, created_at_ms, updated_at_ms)
          VALUES (
            'call_1',
            'chat_1',
            'msg_assistant_1',
            'read_file',
            '{}',
            'done',
            '{\"files\":[]}',
            'model tool output',
            'tool output',
            1,
            1
          );
        ",
    )
    .unwrap();

    let assistant =
        cancel_turn_on_connection(&mut conn, "chat_1", "msg_assistant_1", "partial kept").unwrap();

    assert_eq!(assistant.status, "cancelled");
    assert_eq!(assistant.content, "partial kept");
    let statuses = conn
        .prepare("SELECT id, status, content FROM chat_messages ORDER BY sequence ASC")
        .unwrap()
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        statuses,
        vec![
            (
                "msg_user_1".to_string(),
                "done".to_string(),
                "make a thing".to_string()
            ),
            (
                "msg_assistant_1".to_string(),
                "cancelled".to_string(),
                "partial kept".to_string()
            ),
            (
                "msg_tool_1".to_string(),
                "cancelled".to_string(),
                "tool output".to_string()
            ),
            (
                "msg_user_2".to_string(),
                "done".to_string(),
                "next turn".to_string()
            ),
            (
                "msg_assistant_2".to_string(),
                "done".to_string(),
                "next answer".to_string()
            ),
        ]
    );

    let tool_status: String = conn
        .query_row(
            "SELECT status FROM chat_tool_calls WHERE id = 'call_1'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(tool_status, "cancelled");

    let replay = replay::replay_messages_from_conn(&conn, "chat_1").unwrap();
    assert_eq!(replay.len(), 3);
    assert_eq!(replay[0]["role"], "user");
    assert_eq!(replay[0]["content"], "make a thing");
    assert_eq!(replay[1]["role"], "user");
    assert_eq!(replay[1]["content"], "next turn");
    assert_eq!(replay[2]["role"], "assistant");
    assert_eq!(replay[2]["content"], "next answer");
}

fn create_tool_persistence_schema(conn: &Connection) {
    conn.execute_batch(
        "
        CREATE TABLE chats (
          id TEXT PRIMARY KEY,
          title TEXT NOT NULL DEFAULT 'New chat',
          project_path TEXT,
          project_name TEXT,
          model TEXT NOT NULL,
          reasoning_effort TEXT NOT NULL DEFAULT 'medium',
          total_cost REAL NOT NULL DEFAULT 0,
          latest_prompt_tokens INTEGER NOT NULL DEFAULT 0,
          message_count INTEGER NOT NULL DEFAULT 0,
          archived_at_ms INTEGER,
          created_at_ms INTEGER NOT NULL,
          updated_at_ms INTEGER NOT NULL
        );
        CREATE TABLE chat_messages (
          id TEXT PRIMARY KEY,
          chat_id TEXT NOT NULL,
          role TEXT NOT NULL,
          status TEXT NOT NULL DEFAULT 'done',
          content TEXT NOT NULL DEFAULT '',
          reasoning_content TEXT,
          generation_id TEXT,
          provider_id TEXT,
          model_id TEXT,
          model_variant TEXT,
          model_ref_json TEXT,
          tool_call_id TEXT,
          tool_name TEXT,
          tool_calls_json TEXT,
          metadata_json TEXT,
          usage_json TEXT,
          cost REAL,
          sequence INTEGER NOT NULL,
          created_at_ms INTEGER NOT NULL,
          updated_at_ms INTEGER NOT NULL
        );
        CREATE TABLE chat_tool_calls (
          id TEXT PRIMARY KEY,
          provider_tool_call_id TEXT,
          chat_id TEXT NOT NULL,
          assistant_message_id TEXT NOT NULL,
          generation_id TEXT,
          tool_name TEXT NOT NULL,
          arguments_json TEXT NOT NULL DEFAULT '{}',
          status TEXT NOT NULL DEFAULT 'pending',
          raw_result_json TEXT,
          mcp_markdown TEXT,
          plugin_markdown TEXT,
          metadata_json TEXT,
          target_keys_json TEXT,
          created_at_ms INTEGER NOT NULL,
          updated_at_ms INTEGER NOT NULL
        );
        CREATE TABLE chat_generations (
          id TEXT PRIMARY KEY,
          chat_id TEXT NOT NULL,
          assistant_message_id TEXT,
          provider_id TEXT,
          model_id TEXT,
          model_variant TEXT,
          model_ref_json TEXT,
          reasoning_effort TEXT,
          status TEXT NOT NULL DEFAULT 'running',
          error_json TEXT,
          started_at_ms INTEGER NOT NULL,
          finished_at_ms INTEGER
        );
        CREATE TABLE chat_context_summaries (
          id TEXT PRIMARY KEY,
          chat_id TEXT NOT NULL,
          generation_id TEXT,
          summary_markdown TEXT NOT NULL,
          covered_start_message_id TEXT,
          covered_start_sequence INTEGER NOT NULL,
          covered_end_message_id TEXT NOT NULL,
          covered_end_sequence INTEGER NOT NULL,
          tail_start_message_id TEXT,
          tail_start_sequence INTEGER,
          source_message_count INTEGER NOT NULL DEFAULT 0,
          model TEXT,
          reasoning_effort TEXT,
          provider_id TEXT,
          model_id TEXT,
          model_variant TEXT,
          model_ref_json TEXT,
          metadata_json TEXT,
          created_at_ms INTEGER NOT NULL
        );
        CREATE INDEX idx_chat_context_summaries_chat_coverage
          ON chat_context_summaries(chat_id, covered_start_sequence, covered_end_sequence);
        CREATE INDEX idx_chat_context_summaries_chat_created
          ON chat_context_summaries(chat_id, created_at_ms);
        CREATE TABLE chat_usage_logs (
          id TEXT PRIMARY KEY,
          chat_id TEXT NOT NULL,
          assistant_message_id TEXT,
          generation_id TEXT,
          model TEXT NOT NULL DEFAULT '',
          provider_id TEXT,
          model_id TEXT,
          model_variant TEXT,
          model_ref_json TEXT,
          agent_type TEXT NOT NULL DEFAULT 'chat',
          prompt_tokens INTEGER NOT NULL DEFAULT 0,
          completion_tokens INTEGER NOT NULL DEFAULT 0,
          total_tokens INTEGER NOT NULL DEFAULT 0,
          reasoning_tokens INTEGER NOT NULL DEFAULT 0,
          cached_tokens INTEGER NOT NULL DEFAULT 0,
          cache_write_tokens INTEGER NOT NULL DEFAULT 0,
          cost REAL NOT NULL DEFAULT 0,
          upstream_cost REAL,
          provider_name TEXT,
          created_at_ms INTEGER NOT NULL
        );
        CREATE UNIQUE INDEX idx_chat_usage_logs_assistant_message
          ON chat_usage_logs(assistant_message_id)
          WHERE assistant_message_id IS NOT NULL;
        ",
    )
    .unwrap();
    crate::runtime_daemon::chat::schema::create_turn_checkpoint_tables(conn).unwrap();
    crate::runtime_daemon::chat::schema::create_turn_recovery_table(conn).unwrap();
}
