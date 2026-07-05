use crate::app::{self, App};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn render(f: &mut Frame, app: &mut App) {
    let size = f.area();

    let parts = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(size);

    let items: Vec<ListItem> = app
        .commits
        .iter()
        .map(|c| {
            ListItem::new(format!("[{}] {}", c.short_id, c.summary))
                .style(Style::default().fg(Color::Cyan))
        })
        .collect();

    let list_border_style = if app.active_panel == app::ActivePanel::CommitListPanel {
        Style::default().fg(Color::Blue)
    } else {
        Style::default()
    };

    let list = List::new(items)
        .block(
            Block::default()
                .border_style(list_border_style)
                .borders(Borders::ALL)
                .title("Commit Log"),
        )
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    f.render_stateful_widget(list, parts[0], &mut app.list_state);

    let mut diff_lines: Vec<ratatui::text::Line> = Vec::new();
    let mut last_file: String = String::new();

    if let Some(selected_index) = app.list_state.selected() {
        if let Some(curr_commit) = app.commits.get(selected_index) {
            diff_lines.push(text::Line::from(text::Span::styled(
                format!("👤 Author: {}", curr_commit.author),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )));

            diff_lines.push(text::Line::from(text::Span::styled(
                format!("🆔 Commit: {}", curr_commit.id),
                Style::default().fg(Color::DarkGray),
            )));

            diff_lines.push(text::Line::from(text::Span::styled(
                "─".repeat(parts[1].width as usize - 2),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    for line in &app.current_diff {
        if line.file_name != last_file {
            last_file = line.file_name.clone();

            diff_lines.push(text::Line::from(text::Span::styled(
                format!("📁 File: {}", last_file),
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )));
        }

        let style = match line.line_type {
            '+' => Style::default().fg(Color::Green),
            '-' => Style::default().fg(Color::Red),
            'H' => Style::default().fg(Color::Cyan),
            _ => Style::default().fg(Color::Gray),
        };

        diff_lines.push(text::Line::from(text::Span::styled(&line.content, style)));
    }

    let diff_border_style = if app.active_panel == app::ActivePanel::DiffDetailPanel {
        Style::default().fg(Color::Blue)
    } else {
        Style::default()
    };

    let detail_panel = Paragraph::new(diff_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(diff_border_style)
                .title("Details"),
        )
        .scroll((app.diff_scroll, app.diff_horizontal_scroll));

    f.render_widget(detail_panel, parts[1]);
}
