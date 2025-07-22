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
        assert_eq!(app.projects_list_state.selected(), None);
    }

    #[test]
    fn test_go_to_top_projects() {
        let mut app = App::new(PathBuf::from("/tmp"));
        app.projects = vec![
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
        app.projects_list_state.select(Some(2));
        *app.projects_list_state.offset_mut() = 1;

        app.go_to_top();

        assert_eq!(app.projects_list_state.selected(), Some(0));
        assert_eq!(app.projects_list_state.offset(), 0);
    }

    #[test]
    fn test_go_to_bottom_projects() {
        let mut app = App::new(PathBuf::from("/tmp"));
        app.projects = vec![
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
        app.projects_list_state.select(Some(0));

        app.go_to_bottom();

        assert_eq!(app.projects_list_state.selected(), Some(2));
    }

    #[test]
    fn test_select_middle_of_screen() {
        let mut app = App::new(PathBuf::from("/tmp"));
        app.projects = (0..10)
            .map(|i| super::project::Project {
                name: format!("project{}", i),
                last_modified: chrono::Utc::now(),
                chat_count: 1,
            })
            .collect();
        app.projects_list_state.select(Some(0));
        *app.projects_list_state.offset_mut() = 2;

        app.select_middle_of_screen(4);

        assert_eq!(app.projects_list_state.selected(), Some(4));
        assert_eq!(app.projects_list_state.offset(), 2);
    }

    #[test]
    fn test_select_top_of_screen() {
        let mut app = App::new(PathBuf::from("/tmp"));
        app.projects = (0..10)
            .map(|i| super::project::Project {
                name: format!("project{}", i),
                last_modified: chrono::Utc::now(),
                chat_count: 1,
            })
            .collect();
        app.projects_list_state.select(Some(5));
        *app.projects_list_state.offset_mut() = 2;

        app.select_top_of_screen();

        assert_eq!(app.projects_list_state.selected(), Some(2));
        assert_eq!(app.projects_list_state.offset(), 2);
    }

    #[test]
    fn test_select_bottom_of_screen() {
        let mut app = App::new(PathBuf::from("/tmp"));
        app.projects = (0..10)
            .map(|i| super::project::Project {
                name: format!("project{}", i),
                last_modified: chrono::Utc::now(),
                chat_count: 1,
            })
            .collect();
        app.projects_list_state.select(Some(2));
        *app.projects_list_state.offset_mut() = 2;

        app.select_bottom_of_screen(4);

        assert_eq!(app.projects_list_state.selected(), Some(5));
        assert_eq!(app.projects_list_state.offset(), 2);
    }
}
