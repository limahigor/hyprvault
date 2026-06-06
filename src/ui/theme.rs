use ratatui::style::{Color, Modifier, Style};

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub app_bg: Color,
    pub panel_bg: Color,
    pub panel_border: Color,
    pub panel_border_active: Color,
    pub title: Color,
    pub subtitle: Color,
    pub text: Color,
    pub text_muted: Color,
    pub text_dim: Color,
    pub accent: Color,
    pub accent_soft: Color,
    pub success_bg: Color,
    pub success_badge_bg: Color,
    pub success_badge_fg: Color,
    pub success_text_fg: Color,
    pub warning: Color,
    pub masked_secret: Color,
    pub revealed_secret: Color,
    pub revealed_secret_bg: Color,
    pub selected_fg: Color,
    pub selected_bg: Color,
    pub value: Color,
    pub attribute_value: Color,
}

pub fn screen_style(theme: &Theme) -> Style {
    Style::default().bg(theme.app_bg)
}

pub fn header_style(theme: &Theme) -> Style {
    Style::default().fg(theme.text).bg(theme.panel_bg)
}

pub fn header_title_style(theme: &Theme) -> Style {
    Style::default()
        .fg(theme.title)
        .bg(theme.panel_bg)
        .add_modifier(Modifier::BOLD)
}

pub fn header_version_style(theme: &Theme) -> Style {
    Style::default()
        .fg(theme.accent)
        .bg(theme.panel_bg)
        .add_modifier(Modifier::BOLD)
}

pub fn header_subtitle_style(theme: &Theme) -> Style {
    Style::default().fg(theme.subtitle).bg(theme.panel_bg)
}

pub fn header_meta_style(theme: &Theme) -> Style {
    Style::default().fg(theme.accent_soft).bg(theme.panel_bg)
}

pub fn panel_style(theme: &Theme) -> Style {
    Style::default().fg(theme.text).bg(theme.panel_bg)
}

pub fn panel_title_style(theme: &Theme, is_active: bool) -> Style {
    let color = if is_active {
        theme.accent
    } else {
        theme.text_muted
    };

    Style::default()
        .fg(color)
        .bg(theme.panel_bg)
        .add_modifier(Modifier::BOLD)
}

pub fn panel_border_style(theme: &Theme, is_active: bool) -> Style {
    let color = if is_active {
        theme.panel_border_active
    } else {
        theme.panel_border
    };

    Style::default().fg(color).bg(theme.panel_bg)
}

pub fn text_style(theme: &Theme) -> Style {
    Style::default().fg(theme.text).bg(theme.panel_bg)
}

pub fn secondary_text_style(theme: &Theme) -> Style {
    Style::default().fg(theme.text_muted).bg(theme.panel_bg)
}

pub fn dim_text_style(theme: &Theme) -> Style {
    Style::default().fg(theme.text_dim).bg(theme.panel_bg)
}

pub fn value_style(theme: &Theme) -> Style {
    Style::default().fg(theme.value).bg(theme.panel_bg)
}

pub fn attribute_value_style(theme: &Theme) -> Style {
    Style::default()
        .fg(theme.attribute_value)
        .bg(theme.panel_bg)
}

pub fn selected_item_style(theme: &Theme) -> Style {
    Style::default()
        .fg(theme.selected_fg)
        .bg(theme.selected_bg)
        .add_modifier(Modifier::BOLD)
}

pub fn empty_state_style(theme: &Theme) -> Style {
    Style::default()
        .fg(theme.text_dim)
        .bg(theme.panel_bg)
        .add_modifier(Modifier::ITALIC)
}

pub fn masked_secret_style(theme: &Theme) -> Style {
    Style::default()
        .fg(theme.masked_secret)
        .bg(theme.panel_bg)
        .add_modifier(Modifier::BOLD)
}

pub fn revealed_secret_style(theme: &Theme) -> Style {
    Style::default()
        .fg(theme.revealed_secret)
        .bg(theme.revealed_secret_bg)
        .add_modifier(Modifier::BOLD)
}

pub fn success_badge_style(theme: &Theme) -> Style {
    Style::default()
        .fg(theme.success_badge_fg)
        .bg(theme.success_badge_bg)
        .add_modifier(Modifier::BOLD)
}

pub fn success_text_style(theme: &Theme) -> Style {
    Style::default()
        .fg(theme.success_text_fg)
        .bg(theme.success_bg)
        .add_modifier(Modifier::BOLD)
}

pub fn footer_key_style(theme: &Theme) -> Style {
    Style::default()
        .fg(theme.accent_soft)
        .bg(theme.panel_bg)
        .add_modifier(Modifier::BOLD)
}

pub fn footer_hint_style(theme: &Theme) -> Style {
    Style::default().fg(theme.text_muted).bg(theme.panel_bg)
}

pub fn warning_style(theme: &Theme) -> Style {
    Style::default()
        .fg(theme.warning)
        .bg(theme.panel_bg)
        .add_modifier(Modifier::BOLD)
}
