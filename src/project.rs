use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn default_timestamp() -> DateTime<Utc> {
    DateTime::<Utc>::from_timestamp(0, 0).unwrap_or_else(Utc::now)
}

fn default_uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    format!("generated-uuid-{}", COUNTER.fetch_add(1, Ordering::SeqCst))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub last_modified: DateTime<Utc>,
    pub chat_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    pub name: String,
    pub last_modified: DateTime<Utc>,
    pub message_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageInner {
    #[serde(default)]
    pub role: String,
    pub content: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    #[serde(default)]
    pub input_tokens: Option<u32>,
    #[serde(default)]
    pub output_tokens: Option<u32>,
    #[serde(default)]
    pub cache_creation_input_tokens: Option<u32>,
    #[serde(default)]
    pub cache_read_input_tokens: Option<u32>,
    #[serde(default)]
    pub service_tier: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "type")]
    pub msg_type: String,
    #[serde(default)]
    pub message: Option<MessageInner>,
    #[serde(default = "default_timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(default = "default_uuid")]
    pub uuid: String,
    #[serde(rename = "parentUuid")]
    pub parent_uuid: Option<String>,
    #[serde(default)]
    pub is_sidechain: Option<bool>,
    #[serde(rename = "sessionId", default)]
    pub session_id: Option<String>,
    // For messages that don't have nested message structure
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub content: Option<Value>,
    // Additional metadata fields
    #[serde(rename = "userType", default)]
    pub user_type: Option<String>,
    #[serde(default)]
    pub cwd: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(rename = "gitBranch", default)]
    pub git_branch: Option<String>,
    #[serde(rename = "isMeta", default)]
    pub is_meta: Option<bool>,
    // Assistant-specific fields
    #[serde(rename = "requestId", default)]
    pub request_id: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub usage: Option<Usage>,
    #[serde(rename = "stop_reason", default)]
    pub stop_reason: Option<String>,
    #[serde(rename = "stop_sequence", default)]
    pub stop_sequence: Option<String>,
}

impl Message {
    pub fn get_detailed_content(&self) -> String {
        std::panic::catch_unwind(|| {
            let content_value = if let Some(ref inner_message) = self.message {
                &inner_message.content
            } else if let Some(ref content) = self.content {
                content
            } else {
                return format!("[{}]", self.msg_type);
            };

            match content_value {
                Value::String(s) => s.clone(),
                Value::Array(arr) => {
                    let mut result = String::new();
                    for item in arr {
                        if let Some(item_type) = item.get("type") {
                            if let Some(type_str) = item_type.as_str() {
                                match type_str {
                                    "text" => {
                                        if let Some(text) = item.get("text") {
                                            if let Some(text_str) = text.as_str() {
                                                if !result.is_empty() {
                                                    result.push_str("\n\n");
                                                }
                                                result.push_str(text_str);
                                            }
                                        }
                                    }
                                    "tool_use" => {
                                        if !result.is_empty() {
                                            result.push_str("\n\n");
                                        }
                                        if let Some(name) = item.get("name") {
                                            if let Some(name_str) = name.as_str() {
                                                result.push_str(&format!("[Tool: {}]\n", name_str));
                                            }
                                        }
                                        if let Some(input) = item.get("input") {
                                            let input_str = serde_json::to_string_pretty(input)
                                                .unwrap_or_else(|_| format!("{:?}", input));
                                            result.push_str(&format!("Parameters:\n{}", input_str));
                                        }
                                    }
                                    "thinking" => {
                                        if let Some(thinking) = item.get("thinking") {
                                            if let Some(thinking_text) = thinking.as_str() {
                                                if !result.is_empty() {
                                                    result.push_str("\n\n");
                                                }
                                                result.push_str(&format!(
                                                    "[Thinking]\n{}",
                                                    thinking_text
                                                ));
                                            }
                                        }
                                    }
                                    "tool_result" => {
                                        if let Some(content) = item.get("content") {
                                            if let Some(content_str) = content.as_str() {
                                                if !result.is_empty() {
                                                    result.push_str("\n\n");
                                                }
                                                result.push_str(&format!(
                                                    "[Tool Result]\n{}",
                                                    content_str
                                                ));
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    if result.is_empty() {
                        format!("[{} content]", self.msg_type)
                    } else {
                        result
                    }
                }
                other => {
                    let display_str = format!("{:?}", other);
                    format!("[{}: {}]", self.msg_type, display_str)
                }
            }
        })
        .unwrap_or_else(|_| format!("[Error parsing {} content]", self.msg_type))
    }

    pub fn get_content_text(&self) -> String {
        std::panic::catch_unwind(|| {
            let content_value = if let Some(ref inner_message) = self.message {
                &inner_message.content
            } else if let Some(ref content) = self.content {
                content
            } else {
                return format!("[{}]", self.msg_type);
            };

            match content_value {
                Value::String(s) => s.clone(),
                Value::Array(arr) => {
                    let mut result = String::new();
                    for item in arr {
                        // Handle content blocks with type field
                        if let Some(item_type) = item.get("type") {
                            if let Some(type_str) = item_type.as_str() {
                                match type_str {
                                    "text" => {
                                        if let Some(text) = item.get("text") {
                                            if let Some(text_str) = text.as_str() {
                                                if !result.is_empty() {
                                                    result.push(' ');
                                                }
                                                let safe_text =
                                                    text_str.chars().take(1000).collect::<String>();
                                                result.push_str(&safe_text);
                                            }
                                        }
                                    }
                                    "tool_use" => {
                                        if let Some(name) = item.get("name") {
                                            if let Some(name_str) = name.as_str() {
                                                if !result.is_empty() {
                                                    result.push(' ');
                                                }
                                                result.push_str(&format!("[tool: {}]", name_str));
                                            }
                                        }
                                    }
                                    "thinking" => {
                                        if let Some(thinking) = item.get("thinking") {
                                            if let Some(thinking_text) = thinking.as_str() {
                                                if !result.is_empty() {
                                                    result.push(' ');
                                                }
                                                let safe_thinking = thinking_text
                                                    .chars()
                                                    .take(50)
                                                    .collect::<String>();
                                                result.push_str(&format!(
                                                    "[thinking: {}...]",
                                                    safe_thinking
                                                ));
                                            }
                                        }
                                    }
                                    "tool_result" => {
                                        if let Some(content) = item.get("content") {
                                            if let Some(content_str) = content.as_str() {
                                                if !result.is_empty() {
                                                    result.push(' ');
                                                }
                                                let safe_content = content_str
                                                    .chars()
                                                    .take(200)
                                                    .collect::<String>();
                                                result.push_str(&format!(
                                                    "[tool result: {}]",
                                                    safe_content
                                                ));
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        // Fallback to original logic for backwards compatibility
                        else if let Some(text) = item.get("text") {
                            if let Some(text_str) = text.as_str() {
                                if !result.is_empty() {
                                    result.push(' ');
                                }
                                let safe_text = text_str.chars().take(1000).collect::<String>();
                                result.push_str(&safe_text);
                            }
                        } else if let Some(content_str) = item.get("content") {
                            if let Some(content_text) = content_str.as_str() {
                                if !result.is_empty() {
                                    result.push(' ');
                                }
                                let safe_content =
                                    content_text.chars().take(1000).collect::<String>();
                                result.push_str(&safe_content);
                            }
                        } else if let Some(thinking) = item.get("thinking") {
                            if let Some(thinking_text) = thinking.as_str() {
                                if !result.is_empty() {
                                    result.push(' ');
                                }
                                let safe_thinking =
                                    thinking_text.chars().take(50).collect::<String>();
                                result.push_str(&format!("[thinking: {}]", safe_thinking));
                            }
                        }
                    }
                    if result.is_empty() {
                        format!("[{} content]", self.msg_type)
                    } else {
                        result
                    }
                }
                other => {
                    // Safely serialize without panicking on complex structures
                    let display_str = format!("{:?}", other);
                    let safe_display = display_str.chars().take(100).collect::<String>();
                    format!("[{}: {}]", self.msg_type, safe_display)
                }
            }
        })
        .unwrap_or_else(|_| format!("[Error parsing {} content]", self.msg_type))
    }

    pub fn get_role(&self) -> &str {
        if let Some(ref inner_message) = self.message {
            if !inner_message.role.is_empty() {
                &inner_message.role
            } else {
                &self.msg_type
            }
        } else if let Some(ref role) = self.role {
            if !role.is_empty() {
                role
            } else {
                &self.msg_type
            }
        } else {
            &self.msg_type
        }
    }
}

#[derive(Debug, Clone)]
pub struct HierarchicalMessage {
    pub message: Message,
    pub is_initial: bool,
    pub chain_depth: usize,
    pub has_continuation: bool,
}

impl HierarchicalMessage {
    pub fn new(message: Message, is_initial: bool, chain_depth: usize) -> Self {
        Self {
            message,
            is_initial,
            chain_depth,
            has_continuation: false,
        }
    }
}

pub fn build_message_hierarchy(messages: Vec<Message>) -> Vec<HierarchicalMessage> {
    // Build a map of message UUID to message for quick lookup
    let mut message_map: HashMap<String, Message> = HashMap::new();
    let mut children_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut root_messages = Vec::new();

    // First pass: build lookup maps
    for message in messages {
        message_map.insert(message.uuid.clone(), message.clone());

        if let Some(parent_uuid) = &message.parent_uuid {
            children_map
                .entry(parent_uuid.clone())
                .or_default()
                .push(message.uuid.clone());
        } else {
            root_messages.push(message.uuid.clone());
        }
    }

    // Sort root messages by timestamp
    root_messages.sort_by(|a, b| {
        let msg_a = message_map.get(a).unwrap();
        let msg_b = message_map.get(b).unwrap();
        msg_a.timestamp.cmp(&msg_b.timestamp)
    });

    let mut hierarchical_messages = Vec::new();

    // Build chains starting from each root message
    for root_uuid in root_messages {
        if let Some(root_message) = message_map.get(&root_uuid) {
            // Add the root message as initial
            let mut root_hierarchical = HierarchicalMessage::new(root_message.clone(), true, 0);
            root_hierarchical.has_continuation = children_map.contains_key(&root_uuid);
            hierarchical_messages.push(root_hierarchical);

            // Add all messages in the chain stemming from this root
            add_chain_messages(
                &root_uuid,
                1,
                &message_map,
                &children_map,
                &mut hierarchical_messages,
            );
        }
    }

    hierarchical_messages
}

fn add_chain_messages(
    parent_uuid: &str,
    chain_depth: usize,
    message_map: &HashMap<String, Message>,
    children_map: &HashMap<String, Vec<String>>,
    result: &mut Vec<HierarchicalMessage>,
) {
    if let Some(child_uuids) = children_map.get(parent_uuid) {
        // Sort children by timestamp
        let mut sorted_children: Vec<_> = child_uuids
            .iter()
            .filter_map(|uuid| message_map.get(uuid).map(|msg| (uuid.clone(), msg.clone())))
            .collect();
        sorted_children.sort_by(|(_, a), (_, b)| a.timestamp.cmp(&b.timestamp));

        for (child_uuid, child_message) in sorted_children {
            let mut child_hierarchical =
                HierarchicalMessage::new(child_message, false, chain_depth);
            child_hierarchical.has_continuation = children_map.contains_key(&child_uuid);
            result.push(child_hierarchical);

            // Continue the chain recursively
            add_chain_messages(
                &child_uuid,
                chain_depth + 1,
                message_map,
                children_map,
                result,
            );
        }
    }
}

pub fn discover_projects(projects_dir: &Path) -> Result<Vec<Project>, Box<dyn std::error::Error>> {
    if !projects_dir.exists() {
        return Err(format!(
            "Projects directory does not exist: {}",
            projects_dir.display()
        )
        .into());
    }

    let mut projects = Vec::new();

    for entry in fs::read_dir(projects_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            let metadata = fs::metadata(&path)?;
            let modified = metadata.modified()?;
            let last_modified = DateTime::<Utc>::from(modified);

            let chat_count = count_chats(&path)?;

            projects.push(Project {
                name,
                last_modified,
                chat_count,
            });
        }
    }

    projects.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));
    Ok(projects)
}

pub fn discover_chats(project_dir: &Path) -> Result<Vec<Chat>, Box<dyn std::error::Error>> {
    if !project_dir.exists() {
        return Err(format!(
            "Project directory does not exist: {}",
            project_dir.display()
        )
        .into());
    }

    let mut chats = Vec::new();

    for entry in fs::read_dir(project_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|ext| ext == "jsonl") {
            let name = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            let metadata = fs::metadata(&path)?;
            let modified = metadata.modified()?;
            let last_modified = DateTime::<Utc>::from(modified);

            let message_count = count_messages(&path)?;

            chats.push(Chat {
                name,
                last_modified,
                message_count,
            });
        }
    }

    chats.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));
    Ok(chats)
}

fn count_chats(project_dir: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    let mut count = 0;

    for entry in fs::read_dir(project_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|ext| ext == "jsonl") {
            count += 1;
        }
    }

    Ok(count)
}

pub fn load_messages(chat_file: &Path) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
    if !chat_file.exists() {
        return Err(format!("Chat file does not exist: {}", chat_file.display()).into());
    }

    let content = fs::read_to_string(chat_file)?;
    let mut messages = Vec::new();
    let mut error_count = 0;

    for (line_num, line) in content.lines().enumerate() {
        if !line.trim().is_empty() {
            match serde_json::from_str::<Message>(line) {
                Ok(message) => messages.push(message),
                Err(e) => {
                    error_count += 1;
                    eprintln!(
                        "Warning: Failed to parse message at line {}: {}",
                        line_num + 1,
                        e
                    );
                    // Continue processing other messages instead of failing completely
                }
            }
        }
    }

    if messages.is_empty() && error_count > 0 {
        return Err(format!(
            "Failed to parse any messages from {} ({} errors)",
            chat_file.display(),
            error_count
        )
        .into());
    }

    Ok(messages)
}

fn count_messages(chat_file: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(chat_file)?;
    Ok(content.lines().count())
}
