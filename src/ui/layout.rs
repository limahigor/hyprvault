use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph, Wrap,
    },
};

use crate::{
    app::{
        App,
        state::{SecretAttribute, SecretItem},
    },
    ui::themes::load_theme,
};

use super::theme::{
    Theme, attribute_value_style, dim_text_style, empty_state_style, footer_hint_style,
    footer_key_style, header_meta_style, header_style, header_subtitle_style, header_title_style,
    header_version_style, masked_secret_style, panel_border_style, panel_style, panel_title_style,
    revealed_secret_style, screen_style, secondary_text_style, selected_item_style,
    success_badge_style, success_text_style, text_style, value_style, warning_style,
};

const ITEM_NAME_MAX_CHARS: usize = 28;
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn render(frame: &mut Frame, app: &App) {
    let theme = load_theme();
    let root = frame.area();

    frame.render_widget(Block::default().style(screen_style(&theme)), root);

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(root);

    render_header(frame, vertical[0], app, &theme);

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(24),
            Constraint::Percentage(32),
            Constraint::Percentage(44),
        ])
        .split(vertical[1]);

    render_collections_panel(frame, columns[0], app, &theme);
    render_items_panel(frame, columns[1], app, &theme);
    render_details_panel(frame, columns[2], app, &theme);

    if app.show_clipboard_notice() {
        render_clipboard_notice(frame, vertical[2], &theme);
    } else {
        render_footer(frame, vertical[2], &theme);
    }
}

fn render_header(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let collection_count = app.collections().len();
    let item_count = app.filtered_items().len();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(header_meta_style(theme))
        .style(header_style(theme))
        .padding(Padding::new(2, 2, 0, 0));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(58), Constraint::Percentage(42)])
        .split(inner);

    let title = Paragraph::new(vec![
        Line::from(Span::styled(" ", header_style(theme))),
        Line::from(vec![
            Span::styled("  HyprVault", header_title_style(theme)),
            Span::styled("  ", header_style(theme)),
            Span::styled(format!("v{APP_VERSION}"), header_version_style(theme)),
        ]),
    ])
    .style(header_style(theme));

    let meta = Paragraph::new(vec![
        Line::from(vec![Span::styled("  ", header_style(theme))]),
        Line::from(vec![
            Span::styled(
                format!("{collection_count} keyrings"),
                header_meta_style(theme),
            ),
            Span::styled("  •  ", header_subtitle_style(theme)),
            Span::styled(
                format!("{item_count} visible items"),
                header_meta_style(theme),
            ),
        ]),
    ])
    .style(header_style(theme))
    .alignment(Alignment::Right);

    frame.render_widget(title, chunks[0]);
    frame.render_widget(meta, chunks[1]);
}

fn render_collections_panel(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let title = format!("Keyrings {}", app.collections().len());
    let block = panel_block(theme, title.as_str(), true);
    let inner = block.inner(area);

    frame.render_widget(block, area);

    let rows: Vec<ListItem> = if app.collections().is_empty() {
        vec![list_item(
            Line::from(Span::styled(
                "No keyrings available",
                empty_state_style(theme),
            )),
            theme,
        )]
    } else {
        app.collections()
            .iter()
            .map(|collection| {
                list_item(
                    Line::from(vec![
                        Span::styled("◆", dim_text_style(theme)),
                        panel_space(theme, "  "),
                        Span::styled(collection.name.clone(), text_style(theme)),
                    ]),
                    theme,
                )
            })
            .collect()
    };

    let list = List::new(rows)
        .style(panel_style(theme))
        .highlight_style(selected_item_style(theme))
        .highlight_symbol("› ")
        .highlight_spacing(HighlightSpacing::Always);

    let mut state = ListState::default();
    state.select((!app.collections().is_empty()).then_some(app.selected_collection_index()));

    frame.render_stateful_widget(list, inner, &mut state);
}

fn render_items_panel(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let filtered_items = app.filtered_items();
    let title = format!("Secrets {}", filtered_items.len());
    let block = panel_block(theme, title.as_str(), true);
    let inner = block.inner(area);

    frame.render_widget(block, area);

    let rows: Vec<ListItem> = if filtered_items.is_empty() {
        vec![list_item(
            Line::from(Span::styled(
                "No secrets in this keyring",
                empty_state_style(theme),
            )),
            theme,
        )]
    } else {
        filtered_items
            .iter()
            .map(|item| {
                let truncated_name = truncate_text(item.name.as_str(), ITEM_NAME_MAX_CHARS);

                list_item(
                    Line::from(vec![
                        Span::styled(secret_icon(item.kind.as_str()), dim_text_style(theme)),
                        panel_space(theme, "  "),
                        Span::styled(truncated_name, text_style(theme)),
                        panel_space(theme, "  "),
                        Span::styled(item.kind.clone(), secondary_text_style(theme)),
                    ]),
                    theme,
                )
            })
            .collect()
    };

    let list = List::new(rows)
        .style(panel_style(theme))
        .highlight_style(selected_item_style(theme))
        .highlight_symbol("› ")
        .highlight_spacing(HighlightSpacing::Always);

    let mut state = ListState::default();
    state.select((!filtered_items.is_empty()).then_some(app.selected_item_index()));

    frame.render_stateful_widget(list, inner, &mut state);
}

fn render_details_panel(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let block = panel_block(theme, "Inspector", true);
    let inner = block.inner(area);

    frame.render_widget(block, area);

    let details_lines = if let Some(item) = app.selected_item() {
        build_details_lines(app, item, theme)
    } else {
        vec![
            Line::from(Span::styled("No secret selected", empty_state_style(theme))),
            empty_panel_line(theme),
            Line::from(Span::styled(
                "Choose a keyring and select an entry to inspect metadata.",
                dim_text_style(theme),
            )),
        ]
    };

    let details_panel = Paragraph::new(details_lines)
        .style(panel_style(theme))
        .wrap(Wrap { trim: false });

    frame.render_widget(details_panel, inner);
}

fn build_details_lines(app: &App, item: &SecretItem, theme: &Theme) -> Vec<Line<'static>> {
    let collection_name = app
        .selected_collection()
        .map(|collection| collection.name.as_str())
        .unwrap_or("Unknown");

    let secret_style = if item.is_secret_visible {
        revealed_secret_style(theme)
    } else {
        masked_secret_style(theme)
    };

    let secret_state = if item.is_secret_visible {
        "visible"
    } else {
        "hidden"
    };

    let secret_hint = if item.is_secret_visible {
        "press s to hide"
    } else {
        "press s to reveal"
    };

    let mut lines = vec![
        Line::from(Span::styled(
            item.name.clone(),
            panel_title_style(theme, true),
        )),
        Line::from(vec![
            Span::styled(item.kind.clone(), secondary_text_style(theme)),
            Span::styled(" • ", dim_text_style(theme)),
            Span::styled(item.source.clone(), secondary_text_style(theme)),
        ]),
        empty_panel_line(theme),
        detail_line("Collection", collection_name, theme),
        detail_line("Updated", item.updated_at.as_str(), theme),
        detail_line("Type", item.kind.as_str(), theme),
        detail_line("Source", item.source.as_str(), theme),
        empty_panel_line(theme),
        section_title("Secret", theme),
        Line::from(vec![
            panel_space(theme, "  "),
            Span::styled(item.secret_preview.clone(), secret_style),
        ]),
        Line::from(vec![
            panel_space(theme, "  "),
            Span::styled(secret_state, secondary_text_style(theme)),
            Span::styled(" • ", dim_text_style(theme)),
            Span::styled(secret_hint, warning_style(theme)),
        ]),
        empty_panel_line(theme),
        section_title("Attributes", theme),
    ];

    if item.attributes.is_empty() {
        lines.push(empty_panel_line(theme));
        lines.push(Line::from(Span::styled(
            "No presentable attributes for this item.",
            empty_state_style(theme),
        )));
        return lines;
    }

    lines.push(empty_panel_line(theme));

    lines.extend(
        item.attributes
            .iter()
            .map(|attribute| attribute_line(attribute, theme)),
    );

    lines
}

fn section_title(title: &str, theme: &Theme) -> Line<'static> {
    Line::from(vec![
        Span::styled("▸ ", panel_title_style(theme, true)),
        Span::styled(title.to_string(), panel_title_style(theme, true)),
    ])
}

fn detail_line(label: &str, value: &str, theme: &Theme) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{label:<12}"), secondary_text_style(theme)),
        Span::styled(value.to_string(), value_style(theme)),
    ])
}

fn attribute_line(attribute: &SecretAttribute, theme: &Theme) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            format!("{:<14}", attribute.key.as_str()),
            secondary_text_style(theme),
        ),
        Span::styled(attribute.value.clone(), attribute_value_style(theme)),
    ])
}

fn render_clipboard_notice(frame: &mut Frame, area: Rect, theme: &Theme) {
    frame.render_widget(Block::default().style(success_text_style(theme)), area);

    let badge = " OKAY! ";
    let message = "  Copied to clipboard";
    let used_width = badge.chars().count() + message.chars().count();
    let padding = " ".repeat((area.width as usize).saturating_sub(used_width));

    let status = Paragraph::new(Line::from(vec![
        Span::styled(badge, success_badge_style(theme)),
        Span::styled(message, success_text_style(theme)),
        Span::styled(padding, success_text_style(theme)),
    ]))
    .style(success_text_style(theme));

    frame.render_widget(status, area);
}

fn render_footer(frame: &mut Frame, area: Rect, theme: &Theme) {
    frame.render_widget(Block::default().style(panel_style(theme)), area);

    let footer = Paragraph::new(Line::from(vec![
        Span::styled("h/l", footer_key_style(theme)),
        Span::styled(" keyrings  ", footer_hint_style(theme)),
        Span::styled("j/k", footer_key_style(theme)),
        Span::styled(" secrets  ", footer_hint_style(theme)),
        Span::styled("s", footer_key_style(theme)),
        Span::styled(" reveal  ", footer_hint_style(theme)),
        Span::styled("c", footer_key_style(theme)),
        Span::styled(" copy  ", footer_hint_style(theme)),
        Span::styled("q", footer_key_style(theme)),
        Span::styled(" quit", footer_hint_style(theme)),
    ]))
    .style(panel_style(theme));

    frame.render_widget(footer, area);
}

fn panel_block(theme: &Theme, title: &str, is_active: bool) -> Block<'static> {
    Block::default()
        .title(Line::from(Span::styled(
            format!(" {title} "),
            panel_title_style(theme, is_active),
        )))
        .borders(Borders::ALL)
        .border_style(panel_border_style(theme, is_active))
        .style(panel_style(theme))
        .padding(Padding::new(1, 1, 1, 0))
}

fn list_item(line: Line<'static>, theme: &Theme) -> ListItem<'static> {
    ListItem::new(line).style(panel_style(theme))
}

fn panel_space(theme: &Theme, value: &str) -> Span<'static> {
    Span::styled(value.to_string(), panel_style(theme))
}

fn empty_panel_line(theme: &Theme) -> Line<'static> {
    Line::from(Span::styled("", panel_style(theme)))
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
    format!("{truncated}…")
}

fn secret_icon(kind: &str) -> &'static str {
    match kind.to_ascii_lowercase().as_str() {
        "token" => "◇",
        "login" => "○",
        "network" => "◌",
        "ssh" => "◆",
        "password" => "●",
        _ => "◇",
    }
}
