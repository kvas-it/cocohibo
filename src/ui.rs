use crate::app::{App, Screen};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) {
    match app.screen {
        Screen::Projects => render_projects(f, app),
    }
}

fn render_projects(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(f.area());

    let projects: Vec<ListItem> = app
        .projects
        .iter()
        .map(|project| {
            let date_str = project.last_modified.format("%Y-%m-%d %H:%M").to_string();
            let content = format!(
                "{:<30} {:<20} {:>5}",
                truncate_string(&project.name, 30),
                date_str,
                project.chat_count
            );
            ListItem::new(Line::from(vec![Span::raw(content)]))
        })
        .collect();

    let list = List::new(projects)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Cocohibo - Claude Code History Browser")
                .title_bottom(Line::from(vec![
                    Span::raw("Use "),
                    Span::styled("↑↓/jk", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to navigate, "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to select, "),
                    Span::styled("q/Esc", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to quit"),
                ])),
        )
        .highlight_style(
            Style::default()
                .bg(Color::LightBlue)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

    let mut list_state = ListState::default();
    if !app.projects.is_empty() {
        list_state.select(Some(app.selected_project_index));
    }

    f.render_stateful_widget(list, chunks[0], &mut list_state);

    let status_text = if app.projects.is_empty() {
        "No projects found"
    } else {
        "Project list"
    };

    let status =
        Paragraph::new(status_text).style(Style::default().fg(Color::White).bg(Color::Blue));
    f.render_widget(status, chunks[1]);
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
