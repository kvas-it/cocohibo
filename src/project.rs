use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;

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
pub struct Message {
    #[serde(rename = "type")]
    pub msg_type: String,
    #[serde(default)]
    pub message: Option<MessageInner>,
    pub timestamp: DateTime<Utc>,
    pub uuid: String,
    // For messages that don't have nested message structure
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub content: Option<Value>,
}

impl Message {
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
                        if let Some(text) = item.get("text") {
                            if let Some(text_str) = text.as_str() {
                                if !result.is_empty() {
                                    result.push(' ');
                                }
                                // Safely handle potentially invalid UTF-8 or very long strings
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
