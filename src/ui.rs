use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
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

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Commit Log"))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    f.render_stateful_widget(list, parts[0], &mut app.list_state);

    let diff_lines: Vec<ratatui::text::Line> = app
        .current_diff
        .iter()
        .map(|line| {
            let style = match line.line_type {
                '+' => Style::default().fg(Color::Green),
                '-' => Style::default().fg(Color::Red),
                'H' => Style::default().fg(Color::Magenta),
                _ => Style::default().fg(Color::Gray),
            };

            text::Line::from(text::Span::styled(&line.content, style))
        })
        .collect();

    let detail_panel =
        Paragraph::new(diff_lines).block(Block::default().borders(Borders::ALL).title("Details"));

    f.render_widget(detail_panel, parts[1]);
}
