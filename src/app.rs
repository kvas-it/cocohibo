use crate::project::{Chat, HierarchicalMessage, Project};
use ratatui::widgets::ListState;
use std::path::PathBuf;

pub trait ListManagerTrait {
    fn move_up(&mut self, page_size: usize);
    fn move_down(&mut self, page_size: usize);
    fn page_up(&mut self, page_size: usize);
    fn page_down(&mut self, page_size: usize);
    fn go_to_top(&mut self);
    fn go_to_bottom(&mut self);
    fn select_middle_of_screen(&mut self, page_size: usize);
    fn select_top_of_screen(&mut self);
    fn select_bottom_of_screen(&mut self, page_size: usize);
}

#[derive(Debug)]
pub struct ListManager<T> {
    pub items: Vec<T>,
    pub state: ListState,
}

impl<T> Default for ListManager<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ListManager<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            state: ListState::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn selected_item(&self) -> Option<&T> {
        self.state.selected().and_then(|i| self.items.get(i))
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.state.select(index);
    }

    pub fn offset(&self) -> usize {
        self.state.offset()
    }

    pub fn set_offset(&mut self, offset: usize) {
        *self.state.offset_mut() = offset;
    }

    pub fn move_up(&mut self, page_size: usize) {
        let current_selection = self.state.selected().unwrap_or(0);
        if current_selection > 0 {
            self.state.select(Some(current_selection - 1));
            self.scroll_to_selected(page_size);
        }
    }

    pub fn move_down(&mut self, page_size: usize) {
        let current_selection = self.state.selected().unwrap_or(0);
        let max_index = self.items.len().saturating_sub(1);
        if current_selection < max_index {
            self.state.select(Some(current_selection + 1));
            self.scroll_to_selected(page_size);
        }
    }

    pub fn page_up(&mut self, page_size: usize) {
        let current_selection = self.state.selected().unwrap_or(0);
        let current_offset = self.state.offset();
        let new_offset = current_offset.saturating_sub(page_size);
        let selection_offset = current_selection.saturating_sub(current_offset);
        let new_selection = (new_offset + selection_offset).min(self.items.len().saturating_sub(1));

        *self.state.offset_mut() = new_offset;
        self.state.select(Some(new_selection));
    }

    pub fn page_down(&mut self, page_size: usize) {
        let current_selection = self.state.selected().unwrap_or(0);
        let current_offset = self.state.offset();
        let max_offset = self.items.len().saturating_sub(page_size);
        let new_offset = (current_offset + page_size).min(max_offset);
        let selection_offset = current_selection.saturating_sub(current_offset);
        let new_selection = (new_offset + selection_offset).min(self.items.len().saturating_sub(1));

        *self.state.offset_mut() = new_offset;
        self.state.select(Some(new_selection));
    }

    pub fn go_to_top(&mut self) {
        if !self.items.is_empty() {
            self.state.select(Some(0));
            *self.state.offset_mut() = 0;
        }
    }

    pub fn go_to_bottom(&mut self) {
        if !self.items.is_empty() {
            let last_index = self.items.len() - 1;
            self.state.select(Some(last_index));
        }
    }

    pub fn select_middle_of_screen(&mut self, page_size: usize) {
        if !self.items.is_empty() {
            let offset = self.state.offset();
            let middle_index = (offset + page_size / 2).min(self.items.len() - 1);
            self.state.select(Some(middle_index));
        }
    }

    pub fn select_top_of_screen(&mut self) {
        if !self.items.is_empty() {
            let offset = self.state.offset();
            let top_index = offset.min(self.items.len() - 1);
            self.state.select(Some(top_index));
        }
    }

    pub fn select_bottom_of_screen(&mut self, page_size: usize) {
        if !self.items.is_empty() {
            let offset = self.state.offset();
            let bottom_index = (offset + page_size - 1).min(self.items.len() - 1);
            self.state.select(Some(bottom_index));
        }
    }

    fn scroll_to_selected(&mut self, visible_area: usize) {
        if let Some(selected) = self.state.selected() {
            let offset = self.state.offset();

            if selected < offset {
                *self.state.offset_mut() = selected;
            } else if selected >= offset + visible_area {
                *self.state.offset_mut() = selected.saturating_sub(visible_area - 1);
            }
        }
    }
}

impl<T> ListManagerTrait for ListManager<T> {
    fn move_up(&mut self, page_size: usize) {
        self.move_up(page_size);
    }

    fn move_down(&mut self, page_size: usize) {
        self.move_down(page_size);
    }

    fn page_up(&mut self, page_size: usize) {
        self.page_up(page_size);
    }

    fn page_down(&mut self, page_size: usize) {
        self.page_down(page_size);
    }

    fn go_to_top(&mut self) {
        self.go_to_top();
    }

    fn go_to_bottom(&mut self) {
        self.go_to_bottom();
    }

    fn select_middle_of_screen(&mut self, page_size: usize) {
        self.select_middle_of_screen(page_size);
    }

    fn select_top_of_screen(&mut self) {
        self.select_top_of_screen();
    }

    fn select_bottom_of_screen(&mut self, page_size: usize) {
        self.select_bottom_of_screen(page_size);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Projects,
    Chats,
    Messages,
}

#[derive(Debug)]
pub struct App {
    pub screen: Screen,
    pub projects: ListManager<Project>,
    pub chats: ListManager<Chat>,
    pub messages: ListManager<HierarchicalMessage>,
    pub projects_dir: PathBuf,
    pub should_quit: bool,
}

impl App {
    pub fn new(projects_dir: PathBuf) -> Self {
        Self {
            screen: Screen::Projects,
            projects: ListManager::new(),
            chats: ListManager::new(),
            messages: ListManager::new(),
            projects_dir,
            should_quit: false,
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn load_projects(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.projects.items = crate::project::discover_projects(&self.projects_dir)?;
        self.projects.state = ListState::default();
        if !self.projects.is_empty() {
            self.projects.select(Some(0));
        }
        Ok(())
    }

    pub fn selected_project(&self) -> Option<&Project> {
        self.projects.selected_item()
    }

    pub fn selected_chat(&self) -> Option<&Chat> {
        self.chats.selected_item()
    }

    pub fn selected_message(&self) -> Option<&HierarchicalMessage> {
        self.messages.selected_item()
    }

    pub fn open_project(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(project) = self.selected_project() {
            let project_path = self.projects_dir.join(&project.name);
            self.chats.items = crate::project::discover_chats(&project_path)?;
            self.chats.state = ListState::default();
            if !self.chats.is_empty() {
                self.chats.select(Some(0));
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
            let messages = crate::project::load_messages(&chat_path)?;
            self.messages.items = crate::project::build_message_hierarchy(messages);
            self.messages.state = ListState::default();
            if !self.messages.is_empty() {
                self.messages.select(Some(0));
            }
            self.screen = Screen::Messages;
        }
        Ok(())
    }

    pub fn go_back(&mut self) {
        match self.screen {
            Screen::Messages => {
                self.screen = Screen::Chats;
                self.messages.items.clear();
            }
            Screen::Chats => {
                self.screen = Screen::Projects;
                self.chats.items.clear();
            }
            Screen::Projects => {
                self.quit();
            }
        }
    }

    fn current_list_mut(&mut self) -> &mut dyn ListManagerTrait {
        match self.screen {
            Screen::Projects => &mut self.projects,
            Screen::Chats => &mut self.chats,
            Screen::Messages => &mut self.messages,
        }
    }

    pub fn move_selection_up_with_size(&mut self, page_size: usize) {
        self.current_list_mut().move_up(page_size);
    }

    pub fn move_selection_down_with_size(&mut self, page_size: usize) {
        self.current_list_mut().move_down(page_size);
    }

    pub fn page_up(&mut self, page_size: usize) {
        self.current_list_mut().page_up(page_size);
    }

    pub fn page_down(&mut self, page_size: usize) {
        self.current_list_mut().page_down(page_size);
    }

    pub fn go_to_top(&mut self) {
        self.current_list_mut().go_to_top();
    }

    pub fn go_to_bottom(&mut self) {
        self.current_list_mut().go_to_bottom();
    }

    pub fn select_middle_of_screen(&mut self, page_size: usize) {
        self.current_list_mut().select_middle_of_screen(page_size);
    }

    pub fn select_top_of_screen(&mut self) {
        self.current_list_mut().select_top_of_screen();
    }

    pub fn select_bottom_of_screen(&mut self, page_size: usize) {
        self.current_list_mut().select_bottom_of_screen(page_size);
    }

    pub fn go_to_next_initial_message(&mut self) {
        if self.screen != Screen::Messages {
            return;
        }

        let current_selection = self.messages.selected().unwrap_or(0);

        // Find the next initial message after current selection
        for (i, hierarchical_message) in self
            .messages
            .items
            .iter()
            .enumerate()
            .skip(current_selection + 1)
        {
            if hierarchical_message.is_initial {
                self.messages.select(Some(i));
                return;
            }
        }
    }

    pub fn go_to_previous_initial_message(&mut self) {
        if self.screen != Screen::Messages {
            return;
        }

        let current_selection = self.messages.selected().unwrap_or(0);

        // Find the previous initial message before current selection
        for (i, hierarchical_message) in self.messages.items.iter().enumerate().rev() {
            if i >= current_selection {
                continue;
            }
            if hierarchical_message.is_initial {
                self.messages.select(Some(i));
                return;
            }
        }
    }
}
