use crate::app::App;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::Rect;
use std::io;

pub fn handle_events(app: &mut App, terminal_area: Rect) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                handle_key_event(app, key, terminal_area);
            }
        }
    }
    Ok(())
}

fn handle_key_event(app: &mut App, key: KeyEvent, terminal_area: Rect) {
    // Calculate page size based on the current screen
    let page_size = match app.screen {
        crate::app::Screen::Messages => {
            // In messages view, we have split panes, so calculate based on the actual list area
            let main_area_height = terminal_area.height.saturating_sub(1); // Subtract status bar
            let list_area_height = if app.vertical_split {
                // Vertical split (Direction::Vertical): message list on top, half height
                (main_area_height / 2).saturating_sub(2) // Half height minus borders
            } else {
                // Horizontal split (Direction::Horizontal): message list on left, full height
                main_area_height.saturating_sub(2) // Full height minus borders
            };
            (list_area_height as usize).max(1)
        }
        _ => {
            // For Projects and Chats screens, use full terminal area
            (terminal_area.height as usize).saturating_sub(3).max(1)
        }
    };

    if app.search_mode {
        handle_search_mode_key(app, key);
    } else {
        handle_normal_mode_key(app, key, page_size);
    }
}

fn handle_search_mode_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => app.quit(),
        KeyCode::Esc => app.exit_search_mode(),
        KeyCode::Enter => app.exit_search_mode_keep_filter(),
        KeyCode::Backspace => app.remove_from_search_query(),
        KeyCode::Char(c) => app.add_to_search_query(c),
        _ => {}
    }
}

fn handle_normal_mode_key(app: &mut App, key: KeyEvent, page_size: usize) {
    match key.code {
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => app.quit(),
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('/') => app.enter_search_mode(),
        KeyCode::Esc | KeyCode::Char('h') => app.go_back(),
        KeyCode::Up | KeyCode::Char('k') => app.move_selection_up_with_size(page_size),
        KeyCode::Down | KeyCode::Char('j') => app.move_selection_down_with_size(page_size),
        KeyCode::PageUp => app.page_up(page_size),
        KeyCode::PageDown => app.page_down(page_size),
        KeyCode::Char(' ') => {
            if key.modifiers.contains(KeyModifiers::SHIFT) {
                app.page_up(page_size);
            } else {
                app.page_down(page_size);
            }
        }
        KeyCode::Enter | KeyCode::Char('l') => {
            let result = match app.screen {
                crate::app::Screen::Projects => app.open_project(),
                crate::app::Screen::Chats => app.open_chat(),
                crate::app::Screen::Messages => Ok(()), // No further navigation from messages
            };
            if let Err(e) = result {
                eprintln!("Error opening: {}", e);
            }
        }
        KeyCode::Char('g') => app.go_to_top(),
        KeyCode::Char('G') => app.go_to_bottom(),
        KeyCode::Char('z') => app.scroll_selected_to_center(page_size),
        KeyCode::Char('t') => app.scroll_selected_to_top(),
        KeyCode::Char('b') => app.scroll_selected_to_bottom(page_size),
        KeyCode::Char('J') => app.go_to_next_initial_message(),
        KeyCode::Char('K') => app.go_to_previous_initial_message(),
        KeyCode::Char('s') => {
            if matches!(app.screen, crate::app::Screen::Messages) {
                app.toggle_split();
            }
        }
        _ => {}
    }
}
