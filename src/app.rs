use crate::project::{Chat, Project};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Screen {
    Projects,
    Chats,
}

#[derive(Debug)]
pub struct App {
    pub screen: Screen,
    pub projects: Vec<Project>,
    pub selected_project_index: usize,
    pub chats: Vec<Chat>,
    pub selected_chat_index: usize,
    pub projects_dir: PathBuf,
    pub should_quit: bool,
}

impl App {
    pub fn new(projects_dir: PathBuf) -> Self {
        Self {
            screen: Screen::Projects,
            projects: Vec::new(),
            selected_project_index: 0,
            chats: Vec::new(),
            selected_chat_index: 0,
            projects_dir,
            should_quit: false,
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn load_projects(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.projects = crate::project::discover_projects(&self.projects_dir)?;
        self.selected_project_index = 0;
        Ok(())
    }

    pub fn selected_project(&self) -> Option<&Project> {
        self.projects.get(self.selected_project_index)
    }

    pub fn selected_chat(&self) -> Option<&Chat> {
        self.chats.get(self.selected_chat_index)
    }

    pub fn open_project(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(project) = self.selected_project() {
            let project_path = self.projects_dir.join(&project.name);
            self.chats = crate::project::discover_chats(&project_path)?;
            self.selected_chat_index = 0;
            self.screen = Screen::Chats;
        }
        Ok(())
    }

    pub fn go_back(&mut self) {
        match self.screen {
            Screen::Chats => {
                self.screen = Screen::Projects;
                self.chats.clear();
            }
            Screen::Projects => {
                self.quit();
            }
        }
    }

    pub fn move_selection_up(&mut self) {
        match self.screen {
            Screen::Projects => {
                if self.selected_project_index > 0 {
                    self.selected_project_index -= 1;
                }
            }
            Screen::Chats => {
                if self.selected_chat_index > 0 {
                    self.selected_chat_index -= 1;
                }
            }
        }
    }

    pub fn move_selection_down(&mut self) {
        match self.screen {
            Screen::Projects => {
                if self.selected_project_index + 1 < self.projects.len() {
                    self.selected_project_index += 1;
                }
            }
            Screen::Chats => {
                if self.selected_chat_index + 1 < self.chats.len() {
                    self.selected_chat_index += 1;
                }
            }
        }
    }
}
