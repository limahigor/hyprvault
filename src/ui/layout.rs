use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState, Paragraph},
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let root = frame.area();

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(root);

    let content = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(38), Constraint::Percentage(62)])
        .split(vertical[1]);

    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            "HyprVault",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("  mock explorer", Style::default().fg(Color::DarkGray)),
    ]));
    frame.render_widget(header, vertical[0]);

    let items: Vec<ListItem> = app
        .items()
        .iter()
        .map(|item| {
            ListItem::new(Line::from(vec![
                Span::styled(item.name.as_str(), Style::default().fg(Color::White)),
                Span::styled("  ", Style::default()),
                Span::styled(item.kind.as_str(), Style::default().fg(Color::DarkGray)),
            ]))
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(app.selected_index()));

    let list = List::new(items)
        .highlight_style(
            Style::default()
                .fg(Color::White)
                .bg(Color::Rgb(28, 32, 40))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("› ");

    let list_block = Block::default();
    frame.render_widget(list_block, content[0]);
    frame.render_stateful_widget(list, content[0].inner(Margin::new(1, 0)), &mut list_state);

    let details = if let Some(item) = app.selected_item() {
        vec![
            Line::from(vec![
                Span::styled("name", Style::default().fg(Color::DarkGray)),
                Span::raw("\n"),
            ]),
            Line::from(Span::styled(
                item.name.as_str(),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("type", Style::default().fg(Color::DarkGray)),
                Span::raw("  "),
                Span::raw(item.kind.as_str()),
            ]),
            Line::from(vec![
                Span::styled("source", Style::default().fg(Color::DarkGray)),
                Span::raw("  "),
                Span::raw(item.source.as_str()),
            ]),
            Line::from(vec![
                Span::styled("updated", Style::default().fg(Color::DarkGray)),
                Span::raw("  "),
                Span::raw(item.updated_at.as_str()),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("tags", Style::default().fg(Color::DarkGray)),
                Span::raw("  "),
                Span::styled(item.tags.join(" • "), Style::default().fg(Color::Cyan)),
            ]),
        ]
    } else {
        vec![Line::from(Span::styled(
            "No secrets available",
            Style::default().fg(Color::DarkGray),
        ))]
    };

    let details_panel = Paragraph::new(details).block(Block::default());
    frame.render_widget(details_panel, content[1].inner(Margin::new(1, 0)));

    let footer = Paragraph::new(Line::from(vec![
        Span::styled("j/k", Style::default().fg(Color::White)),
        Span::styled(" move  ", Style::default().fg(Color::DarkGray)),
        Span::styled("q", Style::default().fg(Color::White)),
        Span::styled(" quit", Style::default().fg(Color::DarkGray)),
    ]));
    frame.render_widget(footer, vertical[2]);
}
