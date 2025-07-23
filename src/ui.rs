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

    // Calculate available width for project names
    let reserved_width = 20 + 5 + 6; // 6 for spacing and borders
    let available_name_width = (chunks[0].width as usize).saturating_sub(reserved_width);

    // First pass: truncate all project names and find max width
    let active_projects = app.projects.active_items();
    let truncated_names: Vec<String> = active_projects
        .iter()
        .map(|project| truncate_from_beginning(&project.name, available_name_width))
        .collect();

    let max_name_width = truncated_names
        .iter()
        .map(|name| name.chars().count())
        .max()
        .unwrap_or(0);

    // Second pass: create list items with consistent padding
    let projects: Vec<ListItem> = active_projects
        .iter()
        .enumerate()
        .map(|(i, project)| {
            let date_str = project.last_modified.format("%Y-%m-%d %H:%M").to_string();
            let padded_name = format!("{:<width$}", truncated_names[i], width = max_name_width);

            let content = format!("{} {:<20} {:>5}", padded_name, date_str, project.chat_count);
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
                    Span::styled("Enter/l", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to select, "),
                    Span::styled("/", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" search, "),
                    Span::styled("h/Esc", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" back, "),
                    Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" quit"),
                ])),
        )
        .highlight_style(
            Style::default()
                .bg(Color::LightBlue)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, chunks[0], &mut app.projects.state);

    let status_text = if app.search_mode {
        format!("/{}", app.search_query)
    } else if app.projects.is_empty() {
        "No projects found".to_string()
    } else {
        "Project list".to_string()
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

    // Calculate available width for chat names
    let reserved_width = 20 + 8 + 6; // 20 for date, 8 for message count, 6 for spacing and borders
    let available_name_width = (chunks[0].width as usize).saturating_sub(reserved_width);

    // First pass: truncate all chat names and find max width
    let active_chats = app.chats.active_items();
    let truncated_names: Vec<String> = active_chats
        .iter()
        .map(|chat| truncate_string(&chat.name, available_name_width))
        .collect();

    let max_name_width = truncated_names
        .iter()
        .map(|name| name.chars().count())
        .max()
        .unwrap_or(0);

    // Second pass: create list items with consistent padding
    let chats: Vec<ListItem> = active_chats
        .iter()
        .enumerate()
        .map(|(i, chat)| {
            let date_str = chat.last_modified.format("%Y-%m-%d %H:%M").to_string();
            let padded_name = format!("{:<width$}", truncated_names[i], width = max_name_width);

            let content = format!("{} {:<20} {:>8}", padded_name, date_str, chat.message_count);
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
                    Span::styled("Enter/l", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to select, "),
                    Span::styled("/", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" search, "),
                    Span::styled("h/Esc", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" back, "),
                    Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" quit"),
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

    let status_text = if app.search_mode {
        format!("/{}", app.search_query)
    } else if app.chats.is_empty() {
        "No chats found".to_string()
    } else {
        format!("{} > Chat list", project_name)
    };

    let status =
        Paragraph::new(status_text).style(Style::default().fg(Color::White).bg(Color::Blue));
    f.render_widget(status, chunks[1]);
}

fn render_messages(f: &mut Frame, app: &mut App) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(f.area());

    // Split the main content area into two panes: list and details
    let direction = if app.vertical_split {
        Direction::Vertical
    } else {
        Direction::Horizontal
    };
    let content_chunks = Layout::default()
        .direction(direction)
        .constraints([Constraint::Min(30), Constraint::Min(30)])
        .split(main_chunks[0]);

    // Render message list in first pane (left or top)
    render_message_list(f, app, content_chunks[0]);

    // Render message details in second pane (right or bottom)
    render_message_details(f, app, content_chunks[1]);

    // Render status line
    let project_name = app
        .selected_project()
        .map(|p| p.name.as_str())
        .unwrap_or("Unknown");
    let chat_name = app
        .selected_chat()
        .map(|c| c.name.as_str())
        .unwrap_or("Unknown");

    let status_text = if app.search_mode {
        format!("/{}", app.search_query)
    } else if app.messages.is_empty() {
        "No messages found".to_string()
    } else {
        format!("{} > {} > Messages", project_name, chat_name)
    };

    let status =
        Paragraph::new(status_text).style(Style::default().fg(Color::White).bg(Color::Blue));
    f.render_widget(status, main_chunks[1]);
}

fn render_message_list(f: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let active_messages = app.messages.active_items();
    let messages: Vec<ListItem> = active_messages
        .iter()
        .enumerate()
        .filter_map(|(filtered_index, hierarchical_message)| {
            // Safely handle each message individually
            std::panic::catch_unwind(|| {
                let message = &hierarchical_message.message;
                let role = message.get_role();
                let content_text = message.get_content_text();
                let role_display = match role {
                    "user" => "U",
                    "assistant" => "A",
                    "system" => "S",
                    _ if role.contains("PostToolUse") || role.contains("Hook") => "H",
                    _ if content_text.contains("PostToolUse") || content_text.contains("Hook") => {
                        "H"
                    }
                    _ => "?",
                };

                // Create indentation: initial messages at 0, related messages at 2 spaces
                let (indent, connector) = if hierarchical_message.is_initial {
                    (String::new(), String::new())
                } else {
                    (
                        "  ".to_string(),
                        if hierarchical_message.has_continuation {
                            "├─".to_string()
                        } else {
                            "└─".to_string()
                        },
                    )
                };

                // Calculate available width for message content
                let total_prefix_len = indent.len() + connector.len();
                // Reserve space for: message number (3), role (1), spacing (4)
                let reserved_width = 3 + 1 + 4 + total_prefix_len;
                let available_width = (area.width as usize).saturating_sub(reserved_width);

                let original_message_number = app.messages.original_index(filtered_index) + 1;
                let content_with_indent = format!(
                    "{:<3} {} {}{}{}",
                    original_message_number,
                    role_display,
                    indent,
                    connector,
                    truncate_string(&content_text, available_width)
                );

                // Style initial messages in bold
                if hierarchical_message.is_initial {
                    ListItem::new(Line::from(vec![Span::styled(
                        content_with_indent,
                        Style::default().add_modifier(Modifier::BOLD),
                    )]))
                } else {
                    ListItem::new(Line::from(vec![Span::raw(content_with_indent)]))
                }
            })
            .unwrap_or_else(|_| {
                // If there's a panic, create an error message item
                let original_message_number = app.messages.original_index(filtered_index) + 1;
                ListItem::new(Line::from(vec![Span::raw(format!(
                    "{:<3} E [Error rendering message]",
                    original_message_number
                ))]))
            })
            .into()
        })
        .collect();

    let list = List::new(messages)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Messages")
                .title_bottom(Line::from(vec![
                    Span::raw("Use "),
                    Span::styled("↑↓/jk", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" navigate, "),
                    Span::styled("J/K", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" initial msgs, "),
                    Span::styled("/", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" search, "),
                    Span::styled("s", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" toggle split, "),
                    Span::styled("h/Esc", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" back, "),
                    Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" quit"),
                ])),
        )
        .highlight_style(
            Style::default()
                .bg(Color::LightBlue)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, area, &mut app.messages.state);
}

fn render_message_details(f: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    if let Some(selected_message) = app.selected_message() {
        let message = &selected_message.message;

        let mut details = Vec::new();

        // Header with basic info
        details.push(Line::from(vec![Span::styled(
            "Message Details",
            Style::default().add_modifier(Modifier::BOLD),
        )]));
        details.push(Line::from("".to_string()));

        // UUID and timestamp
        details.push(Line::from(format!("UUID: {}", message.uuid)));
        details.push(Line::from(format!(
            "Time: {}",
            message.timestamp.format("%Y-%m-%d %H:%M:%S")
        )));
        details.push(Line::from(format!("Type: {}", message.msg_type)));
        details.push(Line::from(format!("Role: {}", message.get_role())));

        // Hierarchical info
        if selected_message.is_initial {
            details.push(Line::from("Status: Initial message"));
        } else {
            details.push(Line::from(format!(
                "Depth: {} (chain message)",
                selected_message.chain_depth
            )));
        }

        if let Some(parent) = &message.parent_uuid {
            details.push(Line::from(format!(
                "Parent: {}",
                truncate_string(parent, 36)
            )));
        }

        details.push(Line::from("".to_string()));

        // Metadata section
        if message.user_type.is_some() || message.cwd.is_some() || message.version.is_some() {
            details.push(Line::from(vec![Span::styled(
                "Metadata:",
                Style::default().add_modifier(Modifier::BOLD),
            )]));

            if let Some(user_type) = &message.user_type {
                details.push(Line::from(format!("User Type: {}", user_type)));
            }
            if let Some(cwd) = &message.cwd {
                details.push(Line::from(format!("Working Dir: {}", cwd)));
            }
            if let Some(version) = &message.version {
                details.push(Line::from(format!("Version: {}", version)));
            }
            if let Some(git_branch) = &message.git_branch {
                details.push(Line::from(format!("Git Branch: {}", git_branch)));
            }
            if let Some(is_meta) = &message.is_meta {
                details.push(Line::from(format!("Meta: {}", is_meta)));
            }
            details.push(Line::from("".to_string()));
        }

        // Assistant-specific info
        if message.model.is_some() || message.usage.is_some() {
            details.push(Line::from(vec![Span::styled(
                "Assistant Info:",
                Style::default().add_modifier(Modifier::BOLD),
            )]));

            if let Some(model) = &message.model {
                details.push(Line::from(format!("Model: {}", model)));
            }
            if let Some(request_id) = &message.request_id {
                details.push(Line::from(format!(
                    "Request ID: {}",
                    truncate_string(request_id, 30)
                )));
            }
            if let Some(stop_reason) = &message.stop_reason {
                details.push(Line::from(format!("Stop Reason: {}", stop_reason)));
            }
            if let Some(usage) = &message.usage {
                details.push(Line::from("Token Usage:"));
                if let Some(input) = usage.input_tokens {
                    details.push(Line::from(format!("  Input: {}", input)));
                }
                if let Some(output) = usage.output_tokens {
                    details.push(Line::from(format!("  Output: {}", output)));
                }
                if let Some(cache_create) = usage.cache_creation_input_tokens {
                    details.push(Line::from(format!("  Cache Create: {}", cache_create)));
                }
                if let Some(cache_read) = usage.cache_read_input_tokens {
                    details.push(Line::from(format!("  Cache Read: {}", cache_read)));
                }
                if let Some(tier) = &usage.service_tier {
                    details.push(Line::from(format!("  Service Tier: {}", tier)));
                }
            }
            details.push(Line::from("".to_string()));
        }

        // Content section
        details.push(Line::from(vec![Span::styled(
            "Content:",
            Style::default().add_modifier(Modifier::BOLD),
        )]));
        let content_text = message.get_detailed_content();
        let content_lines: Vec<Line> = content_text
            .lines()
            .take(50) // Increased limit for detailed view
            .map(|line| Line::from(line.to_string())) // Remove truncation to allow wrapping
            .collect();
        details.extend(content_lines);

        let paragraph = Paragraph::new(details)
            .block(Block::default().borders(Borders::ALL).title("Details"))
            .wrap(ratatui::widgets::Wrap { trim: true });

        f.render_widget(paragraph, area);
    } else {
        // Show placeholder when no message is selected
        let placeholder = Paragraph::new("No message selected")
            .block(Block::default().borders(Borders::ALL).title("Details"))
            .style(Style::default().fg(Color::DarkGray));

        f.render_widget(placeholder, area);
    }
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

fn truncate_from_beginning(s: &str, max_len: usize) -> String {
    let char_count = s.chars().count();
    if char_count <= max_len {
        s.to_string()
    } else if max_len <= 3 {
        // If max_len is very small, just return dots
        "...".chars().take(max_len).collect()
    } else {
        // Safely truncate from the beginning by character boundaries
        let chars: Vec<char> = s.chars().collect();
        let start_index = char_count - (max_len - 3);
        let truncated: String = chars[start_index..].iter().collect();
        format!("...{}", truncated)
    }
}
