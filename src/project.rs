use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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

fn count_messages(chat_file: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(chat_file)?;
    Ok(content.lines().count())
}
