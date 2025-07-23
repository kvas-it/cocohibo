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
    pub filtered_items: Vec<T>,
    pub filtered_indices: Vec<usize>, // Maps filtered position to original position
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
            filtered_items: Vec::new(),
            filtered_indices: Vec::new(),
            state: ListState::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.active_items().len()
    }

    pub fn is_empty(&self) -> bool {
        self.active_items().is_empty()
    }

    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn selected_item(&self) -> Option<&T> {
        self.state
            .selected()
            .and_then(|i| self.active_items().get(i))
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
        let max_index = self.active_items().len().saturating_sub(1);
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
        let new_selection =
            (new_offset + selection_offset).min(self.active_items().len().saturating_sub(1));

        *self.state.offset_mut() = new_offset;
        self.state.select(Some(new_selection));
    }

    pub fn page_down(&mut self, page_size: usize) {
        let current_selection = self.state.selected().unwrap_or(0);
        let current_offset = self.state.offset();
        let max_offset = self.active_items().len().saturating_sub(page_size);
        let new_offset = (current_offset + page_size).min(max_offset);
        let selection_offset = current_selection.saturating_sub(current_offset);
        let new_selection =
            (new_offset + selection_offset).min(self.active_items().len().saturating_sub(1));

        *self.state.offset_mut() = new_offset;
        self.state.select(Some(new_selection));
    }

    pub fn go_to_top(&mut self) {
        if !self.active_items().is_empty() {
            self.state.select(Some(0));
            *self.state.offset_mut() = 0;
        }
    }

    pub fn go_to_bottom(&mut self) {
        if !self.active_items().is_empty() {
            let last_index = self.active_items().len() - 1;
            self.state.select(Some(last_index));
        }
    }

    pub fn select_middle_of_screen(&mut self, page_size: usize) {
        if !self.active_items().is_empty() {
            let offset = self.state.offset();
            let middle_index = (offset + page_size / 2).min(self.active_items().len() - 1);
            self.state.select(Some(middle_index));
        }
    }

    pub fn select_top_of_screen(&mut self) {
        if !self.active_items().is_empty() {
            let offset = self.state.offset();
            let top_index = offset.min(self.active_items().len() - 1);
            self.state.select(Some(top_index));
        }
    }

    pub fn select_bottom_of_screen(&mut self, page_size: usize) {
        if !self.active_items().is_empty() {
            let offset = self.state.offset();
            let bottom_index = (offset + page_size - 1).min(self.active_items().len() - 1);
            self.state.select(Some(bottom_index));
        }
    }

    pub fn scroll_selected_to_top(&mut self) {
        if let Some(selected) = self.state.selected() {
            *self.state.offset_mut() = selected;
        }
    }

    pub fn scroll_selected_to_center(&mut self, page_size: usize) {
        if let Some(selected) = self.state.selected() {
            let new_offset = selected.saturating_sub(page_size / 2);
            let max_offset = self.active_items().len().saturating_sub(page_size);
            *self.state.offset_mut() = new_offset.min(max_offset);
        }
    }

    pub fn scroll_selected_to_bottom(&mut self, page_size: usize) {
        if let Some(selected) = self.state.selected() {
            let new_offset = selected.saturating_sub(page_size - 1);
            let max_offset = self.active_items().len().saturating_sub(page_size);
            *self.state.offset_mut() = new_offset.min(max_offset);
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

    pub fn active_items(&self) -> &Vec<T> {
        if !self.filtered_items.is_empty() {
            &self.filtered_items
        } else {
            &self.items
        }
    }

    pub fn is_filtered(&self) -> bool {
        !self.filtered_items.is_empty()
    }

    pub fn original_index(&self, filtered_index: usize) -> usize {
        if self.is_filtered() {
            self.filtered_indices
                .get(filtered_index)
                .copied()
                .unwrap_or(filtered_index)
        } else {
            filtered_index
        }
    }

    pub fn find_original_index_in_filtered(&self, original_index: usize) -> Option<usize> {
        if self.is_filtered() {
            self.filtered_indices
                .iter()
                .position(|&idx| idx == original_index)
        } else {
            Some(original_index)
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
    pub vertical_split: bool,
    pub should_quit: bool,
    pub search_mode: bool,
    pub search_query: String,
    pub current_project: Option<Project>,
}

impl App {
    pub fn new(projects_dir: PathBuf, vertical_split: bool) -> Self {
        Self {
            screen: Screen::Projects,
            projects: ListManager::new(),
            chats: ListManager::new(),
            messages: ListManager::new(),
            projects_dir,
            vertical_split,
            should_quit: false,
            search_mode: false,
            search_query: String::new(),
            current_project: None,
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
        if self.screen == Screen::Projects {
            self.projects.selected_item()
        } else {
            self.current_project.as_ref()
        }
    }

    pub fn selected_chat(&self) -> Option<&Chat> {
        self.chats.selected_item()
    }

    pub fn selected_message(&self) -> Option<&HierarchicalMessage> {
        self.messages.selected_item()
    }

    pub fn open_project(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(project) = self.projects.selected_item() {
            let project_path = self.projects_dir.join(&project.name);

            // Store the current project before clearing filters
            self.current_project = Some(project.clone());

            self.chats.items = crate::project::discover_chats(&project_path)?;
            self.chats.state = ListState::default();
            if !self.chats.is_empty() {
                self.chats.select(Some(0));
            }
            self.screen = Screen::Chats;
            self.search_mode = false;
            self.search_query.clear();
            self.clear_search_filter();
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
            self.search_mode = false;
            self.search_query.clear();
            self.clear_search_filter();
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
                self.current_project = None; // Clear current project when going back to projects
            }
            Screen::Projects => {
                self.quit();
            }
        }
        self.search_mode = false;
        self.search_query.clear();
        self.clear_search_filter();
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

    pub fn scroll_selected_to_top(&mut self) {
        match self.screen {
            Screen::Projects => self.projects.scroll_selected_to_top(),
            Screen::Chats => self.chats.scroll_selected_to_top(),
            Screen::Messages => self.messages.scroll_selected_to_top(),
        }
    }

    pub fn scroll_selected_to_center(&mut self, page_size: usize) {
        match self.screen {
            Screen::Projects => self.projects.scroll_selected_to_center(page_size),
            Screen::Chats => self.chats.scroll_selected_to_center(page_size),
            Screen::Messages => self.messages.scroll_selected_to_center(page_size),
        }
    }

    pub fn scroll_selected_to_bottom(&mut self, page_size: usize) {
        match self.screen {
            Screen::Projects => self.projects.scroll_selected_to_bottom(page_size),
            Screen::Chats => self.chats.scroll_selected_to_bottom(page_size),
            Screen::Messages => self.messages.scroll_selected_to_bottom(page_size),
        }
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

    pub fn toggle_split(&mut self) {
        self.vertical_split = !self.vertical_split;
    }

    pub fn enter_search_mode(&mut self) {
        self.search_mode = true;
        self.search_query.clear();
        // Preserve selection when entering search mode
        self.clear_search_filter_with_preservation(true);
    }

    pub fn exit_search_mode(&mut self) {
        self.search_mode = false;
        self.search_query.clear();
        // Preserve selection when exiting search mode
        self.clear_search_filter_with_preservation(true);
    }

    pub fn exit_search_mode_keep_filter(&mut self) {
        self.search_mode = false;
    }

    pub fn add_to_search_query(&mut self, c: char) {
        self.search_query.push(c);
        self.apply_search_filter();
    }

    pub fn remove_from_search_query(&mut self) {
        self.search_query.pop();
        self.apply_search_filter();
    }

    fn apply_search_filter(&mut self) {
        self.apply_search_filter_with_preservation(true);
    }

    fn apply_search_filter_with_preservation(&mut self, preserve_selection: bool) {
        if self.search_query.is_empty() {
            self.clear_search_filter_with_preservation(preserve_selection);
            return;
        }

        let query = self.search_query.to_lowercase();
        match self.screen {
            Screen::Projects => {
                self.projects.apply_filter_with_selection_preservation(
                    |project| project.name.to_lowercase().contains(&query),
                    preserve_selection,
                );
            }
            Screen::Chats => {
                self.chats.apply_filter_with_selection_preservation(
                    |chat| chat.name.to_lowercase().contains(&query),
                    preserve_selection,
                );
            }
            Screen::Messages => {
                self.apply_message_filter(&query);
            }
        }
    }

    pub fn apply_message_filter(&mut self, query: &str) {
        // Remember the currently selected original index
        let current_original_index = self
            .messages
            .selected()
            .map(|filtered_idx| self.messages.original_index(filtered_idx));

        self.messages.filtered_items.clear();
        self.messages.filtered_indices.clear();

        for (original_index, message) in self.messages.items.iter().enumerate() {
            let message_number = (original_index + 1).to_string();
            let content_text = message.message.get_content_text().to_lowercase();

            // Search in both message number and content
            if message_number.contains(query) || content_text.contains(query) {
                self.messages.filtered_items.push(message.clone());
                self.messages.filtered_indices.push(original_index);
            }
        }

        // Try to preserve selection
        let new_selection = if !self.messages.filtered_items.is_empty() {
            if let Some(original_idx) = current_original_index {
                // Check if the originally selected item is still in the filtered results
                if let Some(new_filtered_idx) =
                    self.messages.find_original_index_in_filtered(original_idx)
                {
                    Some(new_filtered_idx)
                } else {
                    // Find the previous item that matches (largest original index < current)
                    let mut best_previous = None;
                    for (filtered_idx, &filtered_original_idx) in
                        self.messages.filtered_indices.iter().enumerate()
                    {
                        if filtered_original_idx < original_idx {
                            best_previous = Some(filtered_idx);
                        } else {
                            break; // Since filtered_indices is ordered, we can stop here
                        }
                    }
                    best_previous.or(Some(0)) // Fall back to first item if no previous
                }
            } else {
                Some(0) // No previous selection, select first
            }
        } else {
            None // No items to select
        };

        self.messages.state.select(new_selection);
        *self.messages.state.offset_mut() = 0;
    }

    fn clear_search_filter(&mut self) {
        self.clear_search_filter_with_preservation(false);
    }

    fn clear_search_filter_with_preservation(&mut self, preserve_selection: bool) {
        self.projects.clear_filter_with_preservation(preserve_selection);
        self.chats.clear_filter_with_preservation(preserve_selection);
        self.messages.clear_filter_with_preservation(preserve_selection);
    }
}

impl<T: Clone> ListManager<T> {
    pub fn apply_filter<F>(&mut self, predicate: F)
    where
        F: Fn(&T) -> bool,
    {
        self.apply_filter_with_selection_preservation(predicate, true);
    }

    pub fn apply_filter_with_selection_preservation<F>(
        &mut self,
        predicate: F,
        preserve_selection: bool,
    ) where
        F: Fn(&T) -> bool,
    {
        // Remember the currently selected original index
        let current_original_index = if preserve_selection {
            self.selected()
                .map(|filtered_idx| self.original_index(filtered_idx))
        } else {
            None
        };

        self.filtered_items.clear();
        self.filtered_indices.clear();

        for (original_index, item) in self.items.iter().enumerate() {
            if predicate(item) {
                self.filtered_items.push(item.clone());
                self.filtered_indices.push(original_index);
            }
        }

        // Try to preserve selection
        let new_selection = if preserve_selection && !self.filtered_items.is_empty() {
            if let Some(original_idx) = current_original_index {
                // Check if the originally selected item is still in the filtered results
                if let Some(new_filtered_idx) = self.find_original_index_in_filtered(original_idx) {
                    Some(new_filtered_idx)
                } else {
                    // Find the previous item that matches (largest original index < current)
                    let mut best_previous = None;
                    for (filtered_idx, &filtered_original_idx) in
                        self.filtered_indices.iter().enumerate()
                    {
                        if filtered_original_idx < original_idx {
                            best_previous = Some(filtered_idx);
                        } else {
                            break; // Since filtered_indices is ordered, we can stop here
                        }
                    }
                    best_previous.or(Some(0)) // Fall back to first item if no previous
                }
            } else {
                Some(0) // No previous selection, select first
            }
        } else if !self.filtered_items.is_empty() {
            Some(0) // Default behavior: select first item
        } else {
            None // No items to select
        };

        self.state.select(new_selection);
        *self.state.offset_mut() = 0;
    }

    pub fn clear_filter(&mut self) {
        self.clear_filter_with_preservation(false);
    }

    pub fn clear_filter_with_preservation(&mut self, preserve_selection: bool) {
        // Remember the currently selected original index if preserving selection
        let current_original_index = if preserve_selection {
            self.selected().map(|idx| {
                if self.is_filtered() {
                    self.original_index(idx)
                } else {
                    idx // If not filtered, the index is already the original index
                }
            })
        } else {
            None
        };

        self.filtered_items.clear();
        self.filtered_indices.clear();

        // Restore selection
        let new_selection = if preserve_selection {
            if let Some(original_idx) = current_original_index {
                // The original index should be valid in the full list
                if original_idx < self.items.len() {
                    Some(original_idx)
                } else {
                    Some(0).filter(|_| !self.items.is_empty())
                }
            } else {
                // No previous selection, select first item if available
                Some(0).filter(|_| !self.items.is_empty())
            }
        } else {
            // Default behavior: select first item if available
            Some(0).filter(|_| !self.items.is_empty())
        };

        self.state.select(new_selection);
        *self.state.offset_mut() = 0;
    }
}
