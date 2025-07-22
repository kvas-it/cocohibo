use crate::project::{Chat, Message, Project};
use ratatui::widgets::ListState;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Screen {
    Projects,
    Chats,
    Messages,
}

#[derive(Debug)]
pub struct App {
    pub screen: Screen,
    pub projects: Vec<Project>,
    pub projects_list_state: ListState,
    pub chats: Vec<Chat>,
    pub chats_list_state: ListState,
    pub messages: Vec<Message>,
    pub messages_list_state: ListState,
    pub projects_dir: PathBuf,
    pub should_quit: bool,
}

impl App {
    pub fn new(projects_dir: PathBuf) -> Self {
        Self {
            screen: Screen::Projects,
            projects: Vec::new(),
            projects_list_state: ListState::default(),
            chats: Vec::new(),
            chats_list_state: ListState::default(),
            messages: Vec::new(),
            messages_list_state: ListState::default(),
            projects_dir,
            should_quit: false,
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn load_projects(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.projects = crate::project::discover_projects(&self.projects_dir)?;
        self.projects_list_state = ListState::default();
        if !self.projects.is_empty() {
            self.projects_list_state.select(Some(0));
        }
        Ok(())
    }

    pub fn selected_project(&self) -> Option<&Project> {
        self.projects_list_state
            .selected()
            .and_then(|i| self.projects.get(i))
    }

    pub fn selected_chat(&self) -> Option<&Chat> {
        self.chats_list_state
            .selected()
            .and_then(|i| self.chats.get(i))
    }

    pub fn selected_message(&self) -> Option<&Message> {
        self.messages_list_state
            .selected()
            .and_then(|i| self.messages.get(i))
    }

    pub fn open_project(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(project) = self.selected_project() {
            let project_path = self.projects_dir.join(&project.name);
            self.chats = crate::project::discover_chats(&project_path)?;
            self.chats_list_state = ListState::default();
            if !self.chats.is_empty() {
                self.chats_list_state.select(Some(0));
            }
            self.screen = Screen::Chats;
        }
        Ok(())
    }

    pub fn open_chat(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let (Some(project), Some(chat)) = (self.selected_project(), self.selected_chat()) {
            let chat_path = self
                .projects_dir
                .join(&project.name)
                .join(format!("{}.jsonl", chat.name));
            self.messages = crate::project::load_messages(&chat_path)?;
            self.messages_list_state = ListState::default();
            if !self.messages.is_empty() {
                self.messages_list_state.select(Some(0));
            }
            self.screen = Screen::Messages;
        }
        Ok(())
    }

    pub fn go_back(&mut self) {
        match self.screen {
            Screen::Messages => {
                self.screen = Screen::Chats;
                self.messages.clear();
            }
            Screen::Chats => {
                self.screen = Screen::Projects;
                self.chats.clear();
            }
            Screen::Projects => {
                self.quit();
            }
        }
    }

    pub fn move_selection_up_with_size(&mut self, page_size: usize) {
        match self.screen {
            Screen::Projects => {
                let current_selection = self.projects_list_state.selected().unwrap_or(0);
                if current_selection > 0 {
                    self.projects_list_state.select(Some(current_selection - 1));
                    self.scroll_to_selected_with_size(page_size);
                }
            }
            Screen::Chats => {
                let current_selection = self.chats_list_state.selected().unwrap_or(0);
                if current_selection > 0 {
                    self.chats_list_state.select(Some(current_selection - 1));
                    self.scroll_to_selected_with_size(page_size);
                }
            }
            Screen::Messages => {
                let current_selection = self.messages_list_state.selected().unwrap_or(0);
                if current_selection > 0 {
                    self.messages_list_state.select(Some(current_selection - 1));
                    self.scroll_to_selected_with_size(page_size);
                }
            }
        }
    }

    pub fn move_selection_down_with_size(&mut self, page_size: usize) {
        match self.screen {
            Screen::Projects => {
                let current_selection = self.projects_list_state.selected().unwrap_or(0);
                let max_index = self.projects.len().saturating_sub(1);
                if current_selection < max_index {
                    self.projects_list_state.select(Some(current_selection + 1));
                    self.scroll_to_selected_with_size(page_size);
                }
            }
            Screen::Chats => {
                let current_selection = self.chats_list_state.selected().unwrap_or(0);
                let max_index = self.chats.len().saturating_sub(1);
                if current_selection < max_index {
                    self.chats_list_state.select(Some(current_selection + 1));
                    self.scroll_to_selected_with_size(page_size);
                }
            }
            Screen::Messages => {
                let current_selection = self.messages_list_state.selected().unwrap_or(0);
                let max_index = self.messages.len().saturating_sub(1);
                if current_selection < max_index {
                    self.messages_list_state.select(Some(current_selection + 1));
                    self.scroll_to_selected_with_size(page_size);
                }
            }
        }
    }

    pub fn page_up(&mut self, page_size: usize) {
        match self.screen {
            Screen::Projects => {
                let current_selection = self.projects_list_state.selected().unwrap_or(0);
                let current_offset = self.projects_list_state.offset();
                let new_offset = current_offset.saturating_sub(page_size);
                let selection_offset = current_selection.saturating_sub(current_offset);
                let new_selection =
                    (new_offset + selection_offset).min(self.projects.len().saturating_sub(1));

                // Set offset directly
                *self.projects_list_state.offset_mut() = new_offset;
                self.projects_list_state.select(Some(new_selection));
            }
            Screen::Chats => {
                let current_selection = self.chats_list_state.selected().unwrap_or(0);
                let current_offset = self.chats_list_state.offset();
                let new_offset = current_offset.saturating_sub(page_size);
                let selection_offset = current_selection.saturating_sub(current_offset);
                let new_selection =
                    (new_offset + selection_offset).min(self.chats.len().saturating_sub(1));

                *self.chats_list_state.offset_mut() = new_offset;
                self.chats_list_state.select(Some(new_selection));
            }
            Screen::Messages => {
                let current_selection = self.messages_list_state.selected().unwrap_or(0);
                let current_offset = self.messages_list_state.offset();
                let new_offset = current_offset.saturating_sub(page_size);
                let selection_offset = current_selection.saturating_sub(current_offset);
                let new_selection =
                    (new_offset + selection_offset).min(self.messages.len().saturating_sub(1));

                *self.messages_list_state.offset_mut() = new_offset;
                self.messages_list_state.select(Some(new_selection));
            }
        }
    }

    pub fn page_down(&mut self, page_size: usize) {
        match self.screen {
            Screen::Projects => {
                let current_selection = self.projects_list_state.selected().unwrap_or(0);
                let current_offset = self.projects_list_state.offset();
                let max_offset = self.projects.len().saturating_sub(page_size);
                let new_offset = (current_offset + page_size).min(max_offset);
                let selection_offset = current_selection.saturating_sub(current_offset);
                let new_selection =
                    (new_offset + selection_offset).min(self.projects.len().saturating_sub(1));

                *self.projects_list_state.offset_mut() = new_offset;
                self.projects_list_state.select(Some(new_selection));
            }
            Screen::Chats => {
                let current_selection = self.chats_list_state.selected().unwrap_or(0);
                let current_offset = self.chats_list_state.offset();
                let max_offset = self.chats.len().saturating_sub(page_size);
                let new_offset = (current_offset + page_size).min(max_offset);
                let selection_offset = current_selection.saturating_sub(current_offset);
                let new_selection =
                    (new_offset + selection_offset).min(self.chats.len().saturating_sub(1));

                *self.chats_list_state.offset_mut() = new_offset;
                self.chats_list_state.select(Some(new_selection));
            }
            Screen::Messages => {
                let current_selection = self.messages_list_state.selected().unwrap_or(0);
                let current_offset = self.messages_list_state.offset();
                let max_offset = self.messages.len().saturating_sub(page_size);
                let new_offset = (current_offset + page_size).min(max_offset);
                let selection_offset = current_selection.saturating_sub(current_offset);
                let new_selection =
                    (new_offset + selection_offset).min(self.messages.len().saturating_sub(1));

                *self.messages_list_state.offset_mut() = new_offset;
                self.messages_list_state.select(Some(new_selection));
            }
        }
    }

    fn scroll_to_selected_with_size(&mut self, visible_area: usize) {
        match self.screen {
            Screen::Projects => {
                if let Some(selected) = self.projects_list_state.selected() {
                    let offset = self.projects_list_state.offset();
                    // visible_area parameter is passed in

                    if selected < offset {
                        *self.projects_list_state.offset_mut() = selected;
                    } else if selected >= offset + visible_area {
                        *self.projects_list_state.offset_mut() =
                            selected.saturating_sub(visible_area - 1);
                    }
                }
            }
            Screen::Chats => {
                if let Some(selected) = self.chats_list_state.selected() {
                    let offset = self.chats_list_state.offset();
                    // visible_area parameter is passed in

                    if selected < offset {
                        *self.chats_list_state.offset_mut() = selected;
                    } else if selected >= offset + visible_area {
                        *self.chats_list_state.offset_mut() =
                            selected.saturating_sub(visible_area - 1);
                    }
                }
            }
            Screen::Messages => {
                if let Some(selected) = self.messages_list_state.selected() {
                    let offset = self.messages_list_state.offset();
                    // visible_area parameter is passed in

                    if selected < offset {
                        *self.messages_list_state.offset_mut() = selected;
                    } else if selected >= offset + visible_area {
                        *self.messages_list_state.offset_mut() =
                            selected.saturating_sub(visible_area - 1);
                    }
                }
            }
        }
    }
}
