pub mod app;
pub mod events;
pub mod project;
pub mod ui;

#[cfg(test)]
mod tests {
    use super::app::App;
    use std::path::PathBuf;

    #[test]
    fn smoke_test() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_go_to_top_empty_lists() {
        let mut app = App::new(PathBuf::from("/tmp"), false);
        app.go_to_top();
        assert_eq!(app.projects.selected(), None);
    }

    #[test]
    fn test_go_to_top_projects() {
        let mut app = App::new(PathBuf::from("/tmp"), false);
        app.projects.items = vec![
            super::project::Project {
                name: "project1".to_string(),
                last_modified: chrono::Utc::now(),
                chat_count: 1,
            },
            super::project::Project {
                name: "project2".to_string(),
                last_modified: chrono::Utc::now(),
                chat_count: 1,
            },
            super::project::Project {
                name: "project3".to_string(),
                last_modified: chrono::Utc::now(),
                chat_count: 1,
            },
        ];
        app.projects.select(Some(2));
        app.projects.set_offset(1);

        app.go_to_top();

        assert_eq!(app.projects.selected(), Some(0));
        assert_eq!(app.projects.offset(), 0);
    }

    #[test]
    fn test_go_to_bottom_projects() {
        let mut app = App::new(PathBuf::from("/tmp"), false);
        app.projects.items = vec![
            super::project::Project {
                name: "project1".to_string(),
                last_modified: chrono::Utc::now(),
                chat_count: 1,
            },
            super::project::Project {
                name: "project2".to_string(),
                last_modified: chrono::Utc::now(),
                chat_count: 1,
            },
            super::project::Project {
                name: "project3".to_string(),
                last_modified: chrono::Utc::now(),
                chat_count: 1,
            },
        ];
        app.projects.select(Some(0));

        app.go_to_bottom();

        assert_eq!(app.projects.selected(), Some(2));
    }

    #[test]
    fn test_select_middle_of_screen() {
        let mut app = App::new(PathBuf::from("/tmp"), false);
        app.projects.items = (0..10)
            .map(|i| super::project::Project {
                name: format!("project{}", i),
                last_modified: chrono::Utc::now(),
                chat_count: 1,
            })
            .collect();
        app.projects.select(Some(0));
        app.projects.set_offset(2);

        app.select_middle_of_screen(4);

        assert_eq!(app.projects.selected(), Some(4));
        assert_eq!(app.projects.offset(), 2);
    }

    #[test]
    fn test_select_top_of_screen() {
        let mut app = App::new(PathBuf::from("/tmp"), false);
        app.projects.items = (0..10)
            .map(|i| super::project::Project {
                name: format!("project{}", i),
                last_modified: chrono::Utc::now(),
                chat_count: 1,
            })
            .collect();
        app.projects.select(Some(5));
        app.projects.set_offset(2);

        app.select_top_of_screen();

        assert_eq!(app.projects.selected(), Some(2));
        assert_eq!(app.projects.offset(), 2);
    }

    #[test]
    fn test_select_bottom_of_screen() {
        let mut app = App::new(PathBuf::from("/tmp"), false);
        app.projects.items = (0..10)
            .map(|i| super::project::Project {
                name: format!("project{}", i),
                last_modified: chrono::Utc::now(),
                chat_count: 1,
            })
            .collect();
        app.projects.select(Some(2));
        app.projects.set_offset(2);

        app.select_bottom_of_screen(4);

        assert_eq!(app.projects.selected(), Some(5));
        assert_eq!(app.projects.offset(), 2);
    }

    #[test]
    fn test_initial_message_navigation() {
        let mut app = App::new(PathBuf::from("/tmp"), false);
        app.screen = super::app::Screen::Messages;

        // Create test messages with mixed initial and non-initial messages
        app.messages.items = vec![
            super::project::HierarchicalMessage::new(
                super::project::Message {
                    msg_type: "user".to_string(),
                    message: None,
                    timestamp: chrono::Utc::now(),
                    uuid: "msg1".to_string(),
                    parent_uuid: None,
                    is_sidechain: None,
                    session_id: None,
                    role: None,
                    content: None,
                    user_type: None,
                    cwd: None,
                    version: None,
                    git_branch: None,
                    is_meta: None,
                    request_id: None,
                    model: None,
                    usage: None,
                    stop_reason: None,
                    stop_sequence: None,
                },
                true, // is_initial
                0,
            ),
            super::project::HierarchicalMessage::new(
                super::project::Message {
                    msg_type: "assistant".to_string(),
                    message: None,
                    timestamp: chrono::Utc::now(),
                    uuid: "msg2".to_string(),
                    parent_uuid: Some("msg1".to_string()),
                    is_sidechain: None,
                    session_id: None,
                    role: None,
                    content: None,
                    user_type: None,
                    cwd: None,
                    version: None,
                    git_branch: None,
                    is_meta: None,
                    request_id: None,
                    model: None,
                    usage: None,
                    stop_reason: None,
                    stop_sequence: None,
                },
                false, // not initial
                1,
            ),
            super::project::HierarchicalMessage::new(
                super::project::Message {
                    msg_type: "user".to_string(),
                    message: None,
                    timestamp: chrono::Utc::now(),
                    uuid: "msg3".to_string(),
                    parent_uuid: None,
                    is_sidechain: None,
                    session_id: None,
                    role: None,
                    content: None,
                    user_type: None,
                    cwd: None,
                    version: None,
                    git_branch: None,
                    is_meta: None,
                    request_id: None,
                    model: None,
                    usage: None,
                    stop_reason: None,
                    stop_sequence: None,
                },
                true, // is_initial
                0,
            ),
        ];

        // Start at first message
        app.messages.select(Some(0));

        // Go to next initial message
        app.go_to_next_initial_message();
        assert_eq!(app.messages.selected(), Some(2)); // Should go to msg3

        // Go to previous initial message
        app.go_to_previous_initial_message();
        assert_eq!(app.messages.selected(), Some(0)); // Should go back to msg1

        // Test from non-initial message
        app.messages.select(Some(1)); // Select msg2 (not initial)
        app.go_to_next_initial_message();
        assert_eq!(app.messages.selected(), Some(2)); // Should go to msg3
    }

    #[test]
    fn test_parse_sample_messages() {
        let test_chat =
            std::path::Path::new("tests/sample-projects/test-project-1/basic-conversation.jsonl");
        if test_chat.exists() {
            let messages =
                super::project::load_messages(test_chat).expect("Should parse test messages");
            assert!(!messages.is_empty(), "Should have at least one message");

            // Check that the first message has extended fields
            let first_msg = &messages[0];
            assert!(first_msg.user_type.is_some(), "Should have user_type field");
            assert!(first_msg.cwd.is_some(), "Should have cwd field");
            assert!(first_msg.version.is_some(), "Should have version field");
        }
    }

    #[test]
    fn test_parse_tool_usage_messages() {
        let test_chat =
            std::path::Path::new("tests/sample-projects/test-project-1/tool-usage-example.jsonl");
        if test_chat.exists() {
            let messages =
                super::project::load_messages(test_chat).expect("Should parse tool usage messages");
            assert!(!messages.is_empty(), "Should have at least one message");

            // Find an assistant message with usage info
            let assistant_msg = messages
                .iter()
                .find(|m| m.get_role() == "assistant" && m.usage.is_some());
            assert!(
                assistant_msg.is_some(),
                "Should have assistant message with usage info"
            );

            if let Some(msg) = assistant_msg {
                let usage = msg.usage.as_ref().unwrap();
                assert!(
                    usage.input_tokens.is_some() || usage.output_tokens.is_some(),
                    "Should have token usage information"
                );
            }
        }
    }

    #[test]
    fn test_parse_messages_with_missing_fields() {
        let test_chat =
            std::path::Path::new("tests/sample-projects/test-project-1/broken-message.jsonl");
        if test_chat.exists() {
            let messages = super::project::load_messages(test_chat)
                .expect("Should parse messages even with missing fields");
            assert!(!messages.is_empty(), "Should have at least one message");
            assert_eq!(messages.len(), 5, "Should have parsed all 5 messages");

            // Check that messages without timestamp get a default timestamp
            let no_timestamp_msg = messages.iter().find(|m| m.uuid == "no-timestamp-msg");
            assert!(
                no_timestamp_msg.is_some(),
                "Should have parsed message without timestamp"
            );

            if let Some(msg) = no_timestamp_msg {
                // Should have a default timestamp (epoch time)
                assert_eq!(
                    msg.timestamp.timestamp(),
                    0,
                    "Should have default epoch timestamp"
                );
            }

            // Check that messages without UUID get a generated UUID
            let generated_uuid_msgs: Vec<_> = messages
                .iter()
                .filter(|m| m.uuid.starts_with("generated-uuid-"))
                .collect();
            assert!(
                !generated_uuid_msgs.is_empty(),
                "Should have messages with generated UUIDs"
            );

            // Check that each generated UUID is unique
            let mut seen_uuids = std::collections::HashSet::new();
            for msg in &generated_uuid_msgs {
                assert!(
                    seen_uuids.insert(&msg.uuid),
                    "Generated UUIDs should be unique"
                );
            }
        }
    }

    #[test]
    fn test_navigate_with_filtering() {
        let mut app = App::new(PathBuf::from("tests/sample-projects"), false);

        // Load test projects
        if app.load_projects().is_ok() {
            assert!(!app.projects.is_empty());

            // Select first project and open it
            app.projects.select(Some(0));
            if let Ok(_) = app.open_project() {
                assert_eq!(app.screen, super::app::Screen::Chats);
                assert!(!app.chats.is_empty());

                // Select first chat and try to open it
                app.chats.select(Some(0));
                let result = app.open_chat();

                // This should succeed with test data
                if let Err(ref e) = result {
                    println!("Error opening chat: {:?}", e);
                }
                assert!(result.is_ok(), "Should be able to open chat with test data");
                assert_eq!(app.screen, super::app::Screen::Messages);
            }
        }
    }

    #[test]
    fn test_search_functionality() {
        let mut app = App::new(PathBuf::from("/tmp"), false);

        // Set up test projects
        app.projects.items = vec![
            super::project::Project {
                name: "test-project".to_string(),
                last_modified: chrono::Utc::now(),
                chat_count: 1,
            },
            super::project::Project {
                name: "another-project".to_string(),
                last_modified: chrono::Utc::now(),
                chat_count: 1,
            },
            super::project::Project {
                name: "debug-session".to_string(),
                last_modified: chrono::Utc::now(),
                chat_count: 1,
            },
        ];

        // Test initial state
        assert!(!app.search_mode);
        assert_eq!(app.search_query, "");
        assert_eq!(app.projects.active_items().len(), 3);
        assert!(!app.projects.is_filtered());

        // Enter search mode
        app.enter_search_mode();
        assert!(app.search_mode);
        assert_eq!(app.search_query, "");
        assert_eq!(app.projects.active_items().len(), 3); // Empty query shows all

        // Add search query
        app.add_to_search_query('t');
        app.add_to_search_query('e');
        app.add_to_search_query('s');
        app.add_to_search_query('t');
        assert_eq!(app.search_query, "test");
        assert_eq!(app.projects.active_items().len(), 1); // Only "test-project" matches
        assert_eq!(app.projects.active_items()[0].name, "test-project");
        assert!(app.projects.is_filtered());

        // Remove from search query
        app.remove_from_search_query();
        assert_eq!(app.search_query, "tes");
        assert_eq!(app.projects.active_items().len(), 1); // Still matches "test-project"

        // Clear search query completely
        app.remove_from_search_query();
        app.remove_from_search_query();
        app.remove_from_search_query();
        assert_eq!(app.search_query, "");
        assert_eq!(app.projects.active_items().len(), 3); // Shows all again

        // Search for another pattern
        app.add_to_search_query('d');
        app.add_to_search_query('e');
        app.add_to_search_query('b');
        app.add_to_search_query('u');
        app.add_to_search_query('g');
        assert_eq!(app.search_query, "debug");
        assert_eq!(app.projects.active_items().len(), 1); // Only "debug-session" matches
        assert_eq!(app.projects.active_items()[0].name, "debug-session");

        // Exit search mode (clear filter)
        app.exit_search_mode();
        assert!(!app.search_mode);
        assert_eq!(app.search_query, "");
        assert_eq!(app.projects.active_items().len(), 3); // Shows all
        assert!(!app.projects.is_filtered());

        // Test exit search mode keep filter
        app.enter_search_mode();
        app.add_to_search_query('a');
        app.add_to_search_query('n');
        app.add_to_search_query('o');
        app.add_to_search_query('t');
        app.add_to_search_query('h');
        app.add_to_search_query('e');
        app.add_to_search_query('r');
        assert_eq!(app.search_query, "another");
        assert_eq!(app.projects.active_items().len(), 1); // Only "another-project" matches

        app.exit_search_mode_keep_filter();
        assert!(!app.search_mode);
        assert_eq!(app.search_query, "another");
        assert_eq!(app.projects.active_items().len(), 1); // Filter still applied
        assert!(app.projects.is_filtered());
    }

    #[test]
    fn test_message_numbering_with_filtering() {
        let mut app = App::new(PathBuf::from("/tmp"), false);
        app.screen = super::app::Screen::Messages;

        // Create test messages
        app.messages.items = vec![
            super::project::HierarchicalMessage::new(
                super::project::Message {
                    msg_type: "user".to_string(),
                    message: None,
                    timestamp: chrono::Utc::now(),
                    uuid: "msg1".to_string(),
                    parent_uuid: None,
                    is_sidechain: None,
                    session_id: None,
                    role: None,
                    content: Some(serde_json::Value::String("Hello world".to_string())),
                    user_type: None,
                    cwd: None,
                    version: None,
                    git_branch: None,
                    is_meta: None,
                    request_id: None,
                    model: None,
                    usage: None,
                    stop_reason: None,
                    stop_sequence: None,
                },
                true,
                0,
            ),
            super::project::HierarchicalMessage::new(
                super::project::Message {
                    msg_type: "assistant".to_string(),
                    message: None,
                    timestamp: chrono::Utc::now(),
                    uuid: "msg2".to_string(),
                    parent_uuid: Some("msg1".to_string()),
                    is_sidechain: None,
                    session_id: None,
                    role: None,
                    content: Some(serde_json::Value::String("Hi there".to_string())),
                    user_type: None,
                    cwd: None,
                    version: None,
                    git_branch: None,
                    is_meta: None,
                    request_id: None,
                    model: None,
                    usage: None,
                    stop_reason: None,
                    stop_sequence: None,
                },
                false,
                1,
            ),
            super::project::HierarchicalMessage::new(
                super::project::Message {
                    msg_type: "user".to_string(),
                    message: None,
                    timestamp: chrono::Utc::now(),
                    uuid: "msg3".to_string(),
                    parent_uuid: None,
                    is_sidechain: None,
                    session_id: None,
                    role: None,
                    content: Some(serde_json::Value::String("Another message".to_string())),
                    user_type: None,
                    cwd: None,
                    version: None,
                    git_branch: None,
                    is_meta: None,
                    request_id: None,
                    model: None,
                    usage: None,
                    stop_reason: None,
                    stop_sequence: None,
                },
                true,
                0,
            ),
        ];

        // Test original indexing without filtering
        assert_eq!(app.messages.active_items().len(), 3);
        assert_eq!(app.messages.original_index(0), 0);
        assert_eq!(app.messages.original_index(1), 1);
        assert_eq!(app.messages.original_index(2), 2);

        // Apply search filter for "Hi" (should match message #2)
        app.apply_message_filter("hi");
        assert_eq!(app.messages.active_items().len(), 1);
        assert_eq!(app.messages.original_index(0), 1); // Filtered position 0 maps to original position 1

        // Search by message number "3" (should match message #3)
        app.apply_message_filter("3");
        assert_eq!(app.messages.active_items().len(), 1);
        assert_eq!(app.messages.original_index(0), 2); // Filtered position 0 maps to original position 2

        // Search for "message" (should only match message #3 which contains "Another message")
        app.apply_message_filter("message");
        assert_eq!(app.messages.active_items().len(), 1);
        assert_eq!(app.messages.original_index(0), 2); // Filtered position 0 maps to original position 2

        // Clear filter
        app.messages.clear_filter();
        assert_eq!(app.messages.active_items().len(), 3);
        assert!(!app.messages.is_filtered());
    }

    #[test]
    fn test_selection_preservation_during_filtering() {
        let mut app = App::new(PathBuf::from("/tmp"), false);
        app.screen = super::app::Screen::Messages;

        // Create test messages
        app.messages.items = vec![
            super::project::HierarchicalMessage::new(
                super::project::Message {
                    msg_type: "user".to_string(),
                    message: None,
                    timestamp: chrono::Utc::now(),
                    uuid: "msg1".to_string(),
                    parent_uuid: None,
                    is_sidechain: None,
                    session_id: None,
                    role: None,
                    content: Some(serde_json::Value::String("First message".to_string())),
                    user_type: None,
                    cwd: None,
                    version: None,
                    git_branch: None,
                    is_meta: None,
                    request_id: None,
                    model: None,
                    usage: None,
                    stop_reason: None,
                    stop_sequence: None,
                },
                true,
                0,
            ),
            super::project::HierarchicalMessage::new(
                super::project::Message {
                    msg_type: "assistant".to_string(),
                    message: None,
                    timestamp: chrono::Utc::now(),
                    uuid: "msg2".to_string(),
                    parent_uuid: Some("msg1".to_string()),
                    is_sidechain: None,
                    session_id: None,
                    role: None,
                    content: Some(serde_json::Value::String("Second message".to_string())),
                    user_type: None,
                    cwd: None,
                    version: None,
                    git_branch: None,
                    is_meta: None,
                    request_id: None,
                    model: None,
                    usage: None,
                    stop_reason: None,
                    stop_sequence: None,
                },
                false,
                1,
            ),
            super::project::HierarchicalMessage::new(
                super::project::Message {
                    msg_type: "user".to_string(),
                    message: None,
                    timestamp: chrono::Utc::now(),
                    uuid: "msg3".to_string(),
                    parent_uuid: None,
                    is_sidechain: None,
                    session_id: None,
                    role: None,
                    content: Some(serde_json::Value::String("Third message".to_string())),
                    user_type: None,
                    cwd: None,
                    version: None,
                    git_branch: None,
                    is_meta: None,
                    request_id: None,
                    model: None,
                    usage: None,
                    stop_reason: None,
                    stop_sequence: None,
                },
                true,
                0,
            ),
        ];

        // Select the second message (index 1)
        app.messages.select(Some(1));
        assert_eq!(app.messages.selected(), Some(1));
        assert_eq!(app.messages.original_index(1), 1);

        // Apply a filter that includes the selected message
        app.apply_message_filter("message");
        assert_eq!(app.messages.active_items().len(), 3); // All messages contain "message"
        assert_eq!(app.messages.selected(), Some(1)); // Should still be selected
        assert_eq!(app.messages.original_index(1), 1); // Should still be the same original message

        // Apply a filter that excludes the selected message (only "Second" matches)
        app.apply_message_filter("second");
        assert_eq!(app.messages.active_items().len(), 1); // Only message #2 matches
        assert_eq!(app.messages.selected(), Some(0)); // Should select the first (and only) result
        assert_eq!(app.messages.original_index(0), 1); // Which is the original message #2

        // Go back to a broader search that includes multiple messages
        app.apply_message_filter("m"); // Should match all messages (they all have "m" in "message")
        assert_eq!(app.messages.active_items().len(), 3);
        assert_eq!(app.messages.selected(), Some(1)); // Should still have message #2 selected
        assert_eq!(app.messages.original_index(1), 1); // Verify it's still original message #2

        // Select the third message (index 2 in filtered, original index 2)
        app.messages.select(Some(2));
        assert_eq!(app.messages.original_index(2), 2);

        // Apply a filter that excludes the third message but includes the first
        app.apply_message_filter("first");
        assert_eq!(app.messages.active_items().len(), 1); // Only "First message" matches
        assert_eq!(app.messages.selected(), Some(0)); // Should select the first result
        assert_eq!(app.messages.original_index(0), 0); // Which is original message #1 (comes before #3)
    }

    #[test]
    fn test_selection_preservation_on_search_mode_changes() {
        let mut app = App::new(PathBuf::from("/tmp"), false);
        app.screen = super::app::Screen::Messages;

        // Create test messages
        app.messages.items = vec![
            super::project::HierarchicalMessage::new(
                super::project::Message {
                    msg_type: "user".to_string(),
                    message: None,
                    timestamp: chrono::Utc::now(),
                    uuid: "msg1".to_string(),
                    parent_uuid: None,
                    is_sidechain: None,
                    session_id: None,
                    role: None,
                    content: Some(serde_json::Value::String("First message".to_string())),
                    user_type: None,
                    cwd: None,
                    version: None,
                    git_branch: None,
                    is_meta: None,
                    request_id: None,
                    model: None,
                    usage: None,
                    stop_reason: None,
                    stop_sequence: None,
                },
                true,
                0,
            ),
            super::project::HierarchicalMessage::new(
                super::project::Message {
                    msg_type: "assistant".to_string(),
                    message: None,
                    timestamp: chrono::Utc::now(),
                    uuid: "msg2".to_string(),
                    parent_uuid: Some("msg1".to_string()),
                    is_sidechain: None,
                    session_id: None,
                    role: None,
                    content: Some(serde_json::Value::String("Second message".to_string())),
                    user_type: None,
                    cwd: None,
                    version: None,
                    git_branch: None,
                    is_meta: None,
                    request_id: None,
                    model: None,
                    usage: None,
                    stop_reason: None,
                    stop_sequence: None,
                },
                false,
                1,
            ),
            super::project::HierarchicalMessage::new(
                super::project::Message {
                    msg_type: "user".to_string(),
                    message: None,
                    timestamp: chrono::Utc::now(),
                    uuid: "msg3".to_string(),
                    parent_uuid: None,
                    is_sidechain: None,
                    session_id: None,
                    role: None,
                    content: Some(serde_json::Value::String("Third message".to_string())),
                    user_type: None,
                    cwd: None,
                    version: None,
                    git_branch: None,
                    is_meta: None,
                    request_id: None,
                    model: None,
                    usage: None,
                    stop_reason: None,
                    stop_sequence: None,
                },
                true,
                0,
            ),
        ];

        // Select the second message (index 1)
        app.messages.select(Some(1));
        assert_eq!(app.messages.selected(), Some(1));

        // Enter search mode - should preserve selection
        app.enter_search_mode();
        assert!(app.search_mode);
        assert_eq!(app.search_query, "");
        assert_eq!(app.messages.selected(), Some(1)); // Should preserve selection
        assert!(!app.messages.is_filtered()); // No filter applied yet

        // Add some search text
        app.add_to_search_query('s'); // "s"
        app.add_to_search_query('e'); // "se"
        app.add_to_search_query('c'); // "sec"
        assert_eq!(app.messages.active_items().len(), 1); // Only "Second message" matches
        assert_eq!(app.messages.selected(), Some(0)); // Should select the only result
        assert_eq!(app.messages.original_index(0), 1); // Which is original message #2

        // Exit search mode - should preserve selection (go back to message #2)
        app.exit_search_mode();
        assert!(!app.search_mode);
        assert_eq!(app.search_query, "");
        assert_eq!(app.messages.selected(), Some(1)); // Should preserve selection at original index 1
        assert!(!app.messages.is_filtered()); // Filter should be cleared

        // Test with a different starting selection
        app.messages.select(Some(2)); // Select third message

        // Enter search mode again
        app.enter_search_mode();
        assert_eq!(app.messages.selected(), Some(2)); // Should preserve selection

        // Search for something that excludes the selected item
        app.add_to_search_query('f'); // "f" - only "First message" matches
        assert_eq!(app.messages.active_items().len(), 1);
        assert_eq!(app.messages.selected(), Some(0)); // Should select the only result
        assert_eq!(app.messages.original_index(0), 0); // Which is original message #1

        // Exit search mode - should preserve selection (go back to message #1)
        app.exit_search_mode();
        assert_eq!(app.messages.selected(), Some(0)); // Should be at original message #1
    }

    #[test]
    fn test_selection_preservation_on_navigation() {
        let mut app = App::new(PathBuf::from("/tmp"), false);

        // Create test projects
        app.projects.items = vec![
            super::project::Project {
                name: "project1".to_string(),
                last_modified: chrono::Utc::now(),
                chat_count: 1,
            },
            super::project::Project {
                name: "project2".to_string(),
                last_modified: chrono::Utc::now(),
                chat_count: 2,
            },
            super::project::Project {
                name: "project3".to_string(),
                last_modified: chrono::Utc::now(),
                chat_count: 3,
            },
        ];

        // Create test chats for project2
        let test_chats = vec![
            super::project::Chat {
                name: "chat1".to_string(),
                last_modified: chrono::Utc::now(),
                message_count: 10,
            },
            super::project::Chat {
                name: "chat2".to_string(),
                last_modified: chrono::Utc::now(),
                message_count: 20,
            },
            super::project::Chat {
                name: "chat3".to_string(),
                last_modified: chrono::Utc::now(),
                message_count: 30,
            },
        ];

        // Select project2 (index 1) and simulate opening it
        app.screen = super::app::Screen::Projects;
        app.projects.select(Some(1));
        assert_eq!(app.projects.selected(), Some(1));

        // Simulate opening the project (manually set up the state)
        app.current_project = Some(app.projects.items[1].clone());
        app.chats.items = test_chats;
        app.screen = super::app::Screen::Chats;
        app.chats.select(Some(0)); // Select first chat by default

        // Select chat2 (index 1)
        app.chats.select(Some(1));
        assert_eq!(app.chats.selected(), Some(1));

        // Simulate opening the chat
        app.current_chat = Some(app.chats.items[1].clone());
        app.screen = super::app::Screen::Messages;

        // Now go back from Messages to Chats
        app.go_back();
        assert_eq!(app.screen, super::app::Screen::Chats);
        assert_eq!(app.chats.selected(), Some(1)); // Should preserve chat2 selection

        // Go back from Chats to Projects
        app.go_back();
        assert_eq!(app.screen, super::app::Screen::Projects);
        assert_eq!(app.projects.selected(), Some(1)); // Should preserve project2 selection
        assert!(app.current_project.is_none()); // Should clear current project
        assert!(app.current_chat.is_none()); // Should clear current chat
    }
}
