use crate::app::{App, Screen};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &mut App) {
    match app.screen {
        Screen::Projects => render_projects(f, app),
        Screen::Chats => render_chats(f, app),
        Screen::Messages => render_messages(f, app),
    }
}

fn render_projects(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(f.area());

    let projects: Vec<ListItem> = app
        .projects
        .items
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
                    Span::styled(
                        "PgUp/PgDn/Space",
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" to page, "),
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

    f.render_stateful_widget(list, chunks[0], &mut app.projects.state);

    let status_text = if app.projects.is_empty() {
        "No projects found"
    } else {
        "Project list"
    };

    let status =
        Paragraph::new(status_text).style(Style::default().fg(Color::White).bg(Color::Blue));
    f.render_widget(status, chunks[1]);
}

fn render_chats(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(f.area());

    let chats: Vec<ListItem> = app
        .chats
        .items
        .iter()
        .map(|chat| {
            let date_str = chat.last_modified.format("%Y-%m-%d %H:%M").to_string();
            let content = format!(
                "{:<30} {:<20} {:>8}",
                truncate_string(&chat.name, 30),
                date_str,
                chat.message_count
            );
            ListItem::new(Line::from(vec![Span::raw(content)]))
        })
        .collect();

    let list = List::new(chats)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Chats in project")
                .title_bottom(Line::from(vec![
                    Span::raw("Use "),
                    Span::styled("↑↓/jk", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to navigate, "),
                    Span::styled(
                        "PgUp/PgDn/Space",
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" to page, "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to select, "),
                    Span::styled("Esc/q", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to go back"),
                ])),
        )
        .highlight_style(
            Style::default()
                .bg(Color::LightBlue)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, chunks[0], &mut app.chats.state);

    let project_name = app
        .selected_project()
        .map(|p| p.name.as_str())
        .unwrap_or("Unknown");

    let status_text = if app.chats.is_empty() {
        "No chats found"
    } else {
        &format!("{} > Chat list", project_name)
    };

    let status =
        Paragraph::new(status_text).style(Style::default().fg(Color::White).bg(Color::Blue));
    f.render_widget(status, chunks[1]);
}

fn render_messages(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(f.area());

    let messages: Vec<ListItem> = app
        .messages
        .items
        .iter()
        .enumerate()
        .filter_map(|(i, message)| {
            // Safely handle each message individually
            std::panic::catch_unwind(|| {
                let _timestamp_str = message.timestamp.format("%H:%M:%S").to_string();
                let role_display = match message.get_role() {
                    "user" => "USER",
                    "assistant" => "ASST",
                    _ => "UNKN",
                };
                let content_text = message.get_content_text();
                let content = format!(
                    "{:<4} {:<8} {}",
                    i + 1,
                    format!("[{}]", role_display),
                    truncate_string(&content_text, 60)
                );
                ListItem::new(Line::from(vec![Span::raw(content)]))
            })
            .unwrap_or_else(|_| {
                // If there's a panic, create an error message item
                ListItem::new(Line::from(vec![Span::raw(format!(
                    "{:<4} {:<8} [Error rendering message]",
                    i + 1,
                    "[ERR]"
                ))]))
            })
            .into()
        })
        .collect();

    let list = List::new(messages)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Messages in chat")
                .title_bottom(Line::from(vec![
                    Span::raw("Use "),
                    Span::styled("↑↓/jk", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to navigate, "),
                    Span::styled(
                        "PgUp/PgDn/Space",
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" to page, "),
                    Span::styled("Esc/q", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to go back"),
                ])),
        )
        .highlight_style(
            Style::default()
                .bg(Color::LightBlue)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, chunks[0], &mut app.messages.state);

    let project_name = app
        .selected_project()
        .map(|p| p.name.as_str())
        .unwrap_or("Unknown");
    let chat_name = app
        .selected_chat()
        .map(|c| c.name.as_str())
        .unwrap_or("Unknown");

    let status_text = if app.messages.is_empty() {
        "No messages found"
    } else {
        &format!("{} > {} > Messages", project_name, chat_name)
    };

    let status =
        Paragraph::new(status_text).style(Style::default().fg(Color::White).bg(Color::Blue));
    f.render_widget(status, chunks[1]);
}

fn truncate_string(s: &str, max_len: usize) -> String {
    let char_count = s.chars().count();
    if char_count <= max_len {
        s.to_string()
    } else if max_len <= 3 {
        // If max_len is very small, just return dots
        "...".chars().take(max_len).collect()
    } else {
        // Safely truncate by character boundaries, not byte boundaries
        let truncated: String = s.chars().take(max_len.saturating_sub(3)).collect();
        format!("{}...", truncated)
    }
}
