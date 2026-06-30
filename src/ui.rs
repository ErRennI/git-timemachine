use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
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

    let detail = Paragraph::new("Detaylar yakında eklenecek...").block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Diff / Details "),
    );
    f.render_widget(detail, parts[1]);
}
