use crate::app::App;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;

pub fn handle_events(app: &mut App) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                handle_key_event(app, key);
            }
        }
    }
    Ok(())
}

fn handle_key_event(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('c')
            if key
                .modifiers
                .contains(crossterm::event::KeyModifiers::CONTROL) =>
        {
            app.quit()
        }
        KeyCode::Char('q') | KeyCode::Esc => app.go_back(),
        KeyCode::Up | KeyCode::Char('k') => app.move_selection_up(),
        KeyCode::Down | KeyCode::Char('j') => app.move_selection_down(),
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
        _ => {}
    }
}
