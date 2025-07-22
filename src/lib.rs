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
}
