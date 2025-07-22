use crate::project::Project;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Screen {
    Projects,
}

#[derive(Debug)]
pub struct App {
    pub screen: Screen,
    pub projects: Vec<Project>,
    pub selected_project_index: usize,
    pub projects_dir: PathBuf,
    pub should_quit: bool,
}

impl App {
    pub fn new(projects_dir: PathBuf) -> Self {
        Self {
            screen: Screen::Projects,
            projects: Vec::new(),
            selected_project_index: 0,
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

    pub fn move_selection_up(&mut self) {
        if self.selected_project_index > 0 {
            self.selected_project_index -= 1;
        }
    }

    pub fn move_selection_down(&mut self) {
        if self.selected_project_index + 1 < self.projects.len() {
            self.selected_project_index += 1;
        }
    }
}
