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
    // Calculate page size: terminal height - status bar (1) - borders (2)
    let page_size = (terminal_area.height as usize).saturating_sub(3).max(1);

    match key.code {
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => app.quit(),
        KeyCode::Char('q') | KeyCode::Esc => app.go_back(),
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
        KeyCode::Enter => {
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
        KeyCode::Char('z') => app.select_middle_of_screen(page_size),
        KeyCode::Char('t') => app.select_top_of_screen(),
        KeyCode::Char('b') => app.select_bottom_of_screen(page_size),
        KeyCode::Char('J') => app.go_to_next_initial_message(),
        KeyCode::Char('K') => app.go_to_previous_initial_message(),
        _ => {}
    }
}
