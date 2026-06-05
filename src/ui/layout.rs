use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState, Paragraph},
};

const ITEM_NAME_MAX_CHARS: usize = 24;

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

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(22),
            Constraint::Percentage(33),
            Constraint::Percentage(45),
        ])
        .split(vertical[1]);

    let header = Paragraph::new(Line::from(vec![Span::styled(
        "HyprVault",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )]));

    frame.render_widget(header, vertical[0]);

    let collection_rows: Vec<ListItem> = app
        .collections()
        .iter()
        .map(|collection| ListItem::new(collection.name.as_str()))
        .collect();

    let collections_list = List::new(collection_rows)
        .highlight_style(
            Style::default()
                .fg(Color::White)
                .bg(Color::Rgb(28, 32, 40))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("› ");

    let mut collections_state = ListState::default();
    collections_state.select(Some(app.selected_collection_index()));

    let collections_header = Paragraph::new(Span::styled(
        "Collections",
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    ));

    let collections_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)])
        .split(columns[0].inner(Margin::new(1, 0)));

    frame.render_widget(collections_header, collections_layout[0]);
    frame.render_stateful_widget(
        collections_list,
        collections_layout[1],
        &mut collections_state,
    );

    let filtered_items = app.filtered_items();

    let item_rows: Vec<ListItem> = filtered_items
        .iter()
        .map(|item| {
            let truncated_name = truncate_text(item.name.as_str(), ITEM_NAME_MAX_CHARS);

            ListItem::new(Line::from(vec![
                Span::styled(truncated_name, Style::default().fg(Color::White)),
                Span::raw("  "),
                Span::styled(item.kind.as_str(), Style::default().fg(Color::DarkGray)),
            ]))
        })
        .collect();

    let mut items_state = ListState::default();
    items_state.select(Some(app.selected_item_index()));

    let items_list = List::new(item_rows)
        .highlight_style(
            Style::default()
                .fg(Color::White)
                .bg(Color::Rgb(28, 32, 40))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("› ");

    let items_header = Paragraph::new(Span::styled(
        "Items",
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    ));

    let items_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)])
        .split(columns[1].inner(Margin::new(1, 0)));

    frame.render_widget(items_header, items_layout[0]);
    frame.render_stateful_widget(items_list, items_layout[1], &mut items_state);

    let details_lines = if let Some(item) = app.selected_item() {
        let collection_name = app
            .selected_collection()
            .map(|collection| collection.name.as_str())
            .unwrap_or("Unknown");

        vec![
            Line::from(vec![
                Span::styled("collection", Style::default().fg(Color::DarkGray)),
                Span::raw("  "),
                Span::raw(collection_name),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("name", Style::default().fg(Color::DarkGray)),
                Span::raw("  "),
                Span::styled(
                    item.name.as_str(),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
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

    let details_header = Paragraph::new(Span::styled(
        "Details",
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    ));

    let details_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)])
        .split(columns[2].inner(Margin::new(1, 0)));

    let details_panel = Paragraph::new(details_lines).block(Block::default());

    frame.render_widget(details_header, details_layout[0]);
    frame.render_widget(details_panel, details_layout[1]);

    let footer = Paragraph::new(Line::from(vec![
        Span::styled("h/l", Style::default().fg(Color::White)),
        Span::styled(" collections  ", Style::default().fg(Color::DarkGray)),
        Span::styled("j/k", Style::default().fg(Color::White)),
        Span::styled(" items  ", Style::default().fg(Color::DarkGray)),
        Span::styled("q", Style::default().fg(Color::White)),
        Span::styled(" quit", Style::default().fg(Color::DarkGray)),
    ]));

    frame.render_widget(footer, vertical[2]);
}

fn truncate_text(value: &str, max_chars: usize) -> String {
    let char_count = value.chars().count();

    if char_count <= max_chars {
        return String::from(value);
    }

    if max_chars <= 1 {
        return String::from("…");
    }

    let truncated: String = value.chars().take(max_chars - 1).collect();
    format!("{}…", truncated)
}
