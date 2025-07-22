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
        let mut app = App::new(PathBuf::from("/tmp"));
        app.go_to_top();
        assert_eq!(app.projects.selected(), None);
    }

    #[test]
    fn test_go_to_top_projects() {
        let mut app = App::new(PathBuf::from("/tmp"));
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
        let mut app = App::new(PathBuf::from("/tmp"));
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
        let mut app = App::new(PathBuf::from("/tmp"));
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
        let mut app = App::new(PathBuf::from("/tmp"));
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
        let mut app = App::new(PathBuf::from("/tmp"));
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
        let mut app = App::new(PathBuf::from("/tmp"));
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
        let test_chat = std::path::Path::new("tests/sample-projects/test-project-1/basic-conversation.jsonl");
        if test_chat.exists() {
            let messages = super::project::load_messages(test_chat).expect("Should parse test messages");
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
        let test_chat = std::path::Path::new("tests/sample-projects/test-project-1/tool-usage-example.jsonl");
        if test_chat.exists() {
            let messages = super::project::load_messages(test_chat).expect("Should parse tool usage messages");
            assert!(!messages.is_empty(), "Should have at least one message");

            // Find an assistant message with usage info
            let assistant_msg = messages.iter().find(|m| m.get_role() == "assistant" && m.usage.is_some());
            assert!(assistant_msg.is_some(), "Should have assistant message with usage info");
            
            if let Some(msg) = assistant_msg {
                let usage = msg.usage.as_ref().unwrap();
                assert!(usage.input_tokens.is_some() || usage.output_tokens.is_some(), 
                       "Should have token usage information");
            }
        }
    }

    #[test]
    fn test_parse_messages_with_missing_fields() {
        let test_chat = std::path::Path::new("tests/sample-projects/test-project-1/broken-message.jsonl");
        if test_chat.exists() {
            let messages = super::project::load_messages(test_chat).expect("Should parse messages even with missing fields");
            assert!(!messages.is_empty(), "Should have at least one message");
            assert_eq!(messages.len(), 5, "Should have parsed all 5 messages");

            // Check that messages without timestamp get a default timestamp
            let no_timestamp_msg = messages.iter().find(|m| m.uuid == "no-timestamp-msg");
            assert!(no_timestamp_msg.is_some(), "Should have parsed message without timestamp");
            
            if let Some(msg) = no_timestamp_msg {
                // Should have a default timestamp (epoch time)
                assert_eq!(msg.timestamp.timestamp(), 0, "Should have default epoch timestamp");
            }

            // Check that messages without UUID get a generated UUID
            let generated_uuid_msgs: Vec<_> = messages.iter().filter(|m| m.uuid.starts_with("generated-uuid-")).collect();
            assert!(!generated_uuid_msgs.is_empty(), "Should have messages with generated UUIDs");

            // Check that each generated UUID is unique
            let mut seen_uuids = std::collections::HashSet::new();
            for msg in &generated_uuid_msgs {
                assert!(seen_uuids.insert(&msg.uuid), "Generated UUIDs should be unique");
            }
        }
    }
}
