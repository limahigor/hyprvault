use std::{fs, path::PathBuf};

use color_eyre::{Result, eyre::eyre};
use ratatui::style::Color;
use serde::Deserialize;

use crate::ui::theme::Theme;

const MIN_TEXT_CONTRAST: f32 = 3.0;

#[derive(Debug, Deserialize)]
struct OmarchyColors {
    background: String,
    foreground: String,
    accent: String,

    #[serde(default)]
    selection_foreground: Option<String>,

    #[serde(default)]
    selection_background: Option<String>,

    #[serde(default)]
    color0: Option<String>,

    #[serde(default)]
    color2: Option<String>,

    #[serde(default)]
    color3: Option<String>,

    #[serde(default)]
    color6: Option<String>,

    #[serde(default)]
    color8: Option<String>,

    #[serde(default)]
    color14: Option<String>,

    #[serde(default)]
    color15: Option<String>,
}

pub fn load_theme() -> Theme {
    match load_omarchy_theme() {
        Ok(theme) => theme,
        Err(error) => {
            eprintln!("HyprVault: failed to load Omarchy theme, using built-in fallback: {error}");
            hypr_theme()
        }
    }
}

pub fn hypr_theme() -> Theme {
    Theme {
        app_bg: Color::Rgb(15, 17, 21),
        panel_bg: Color::Rgb(22, 25, 31),
        panel_border: Color::Rgb(46, 52, 64),
        panel_border_active: Color::Rgb(67, 153, 167),

        title: Color::Rgb(232, 237, 243),
        subtitle: Color::Rgb(140, 151, 164),
        text: Color::Rgb(213, 220, 228),
        text_muted: Color::Rgb(140, 151, 164),
        text_dim: Color::Rgb(90, 102, 117),

        accent: Color::Rgb(67, 153, 167),
        accent_soft: Color::Rgb(47, 114, 125),

        success_bg: Color::Rgb(30, 68, 77),
        success_badge_bg: Color::Rgb(67, 153, 167),
        success_badge_fg: Color::Rgb(0, 0, 0),
        success_text_fg: Color::Rgb(215, 240, 245),

        warning: Color::Rgb(225, 175, 105),

        masked_secret: Color::Rgb(126, 105, 86),
        revealed_secret: Color::Rgb(67, 153, 167),
        revealed_secret_bg: Color::Rgb(22, 38, 43),

        selected_fg: Color::Rgb(255, 255, 255),
        selected_bg: Color::Rgb(37, 47, 56),

        value: Color::Rgb(200, 210, 220),
        attribute_value: Color::Rgb(115, 185, 195),
    }
}

pub fn load_omarchy_theme() -> Result<Theme> {
    let path = omarchy_current_colors_path()?;
    let raw = fs::read_to_string(&path).map_err(|error| {
        eyre!(
            "failed to read Omarchy theme file at {}: {error}",
            path.display()
        )
    })?;
    let colors: OmarchyColors = toml::from_str(&raw).map_err(|error| {
        eyre!(
            "failed to parse Omarchy theme file at {}: {error}",
            path.display()
        )
    })?;

    theme_from_omarchy_colors(colors)
}

fn omarchy_current_colors_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir().ok_or_else(|| eyre!("could not find config directory"))?;

    Ok(config_dir
        .join("omarchy")
        .join("current")
        .join("theme")
        .join("colors.toml"))
}

fn theme_from_omarchy_colors(colors: OmarchyColors) -> Result<Theme> {
    let background = parse_hex_color(&colors.background)?;
    let foreground = parse_hex_color(&colors.foreground)?;
    let accent = parse_hex_color(&colors.accent)?;

    let color0 = parse_optional_hex_color(colors.color0.as_deref()).unwrap_or(background);
    let color2 = parse_optional_hex_color(colors.color2.as_deref()).unwrap_or(accent);
    let color3 = parse_optional_hex_color(colors.color3.as_deref())
        .unwrap_or_else(|| mix(accent, foreground, 0.35));
    let color6 = parse_optional_hex_color(colors.color6.as_deref()).unwrap_or(accent);
    let color8 = parse_optional_hex_color(colors.color8.as_deref())
        .unwrap_or_else(|| mix(foreground, background, 0.55));
    let color14 = parse_optional_hex_color(colors.color14.as_deref()).unwrap_or(color6);
    let color15 = parse_optional_hex_color(colors.color15.as_deref()).unwrap_or(foreground);

    let selection_background =
        parse_optional_hex_color(colors.selection_background.as_deref()).unwrap_or(accent);

    let panel_bg = mix(background, foreground, 0.05);
    let selected_bg = mix(background, selection_background, 0.20);
    let success_bg = mix(background, color2, 0.15);
    let revealed_secret_bg = mix(background, color6, 0.15);

    let base_muted = mix(foreground, background, 0.45);
    let text_muted_candidate = mix(color8, foreground, 0.18);
    let text_muted = readable_or(text_muted_candidate, panel_bg, base_muted);
    let text_dim = mix(text_muted, background, 0.45);

    let title = readable_or(color15, panel_bg, foreground);
    let attribute_value = readable_or(color14, panel_bg, accent);
    let masked_secret = readable_or(mix(text_dim, color3, 0.35), panel_bg, text_muted);

    let selected_fg = parse_optional_hex_color(colors.selection_foreground.as_deref())
        .filter(|color| contrast_ratio(*color, selected_bg) >= MIN_TEXT_CONTRAST)
        .unwrap_or_else(|| best_contrast(selected_bg));

    Ok(Theme {
        app_bg: background,
        panel_bg,
        panel_border: mix(color0, foreground, 0.15),
        panel_border_active: accent,

        title,
        subtitle: text_muted,
        text: foreground,
        text_muted,
        text_dim,

        accent,
        accent_soft: mix(accent, background, 0.45),

        success_bg,
        success_badge_bg: color2,
        success_badge_fg: best_contrast(color2),
        success_text_fg: best_contrast(success_bg),

        warning: color3,

        masked_secret,
        revealed_secret: color6,
        revealed_secret_bg,

        selected_fg,
        selected_bg,

        value: foreground,
        attribute_value,
    })
}

fn parse_optional_hex_color(value: Option<&str>) -> Option<Color> {
    value.and_then(|value| parse_hex_color(value).ok())
}

fn parse_hex_color(value: &str) -> Result<Color> {
    let value = value.trim().trim_matches('"').trim_start_matches('#');

    if value.len() != 6 {
        return Err(eyre!("invalid hex color: {value}"));
    }

    let r = u8::from_str_radix(&value[0..2], 16)?;
    let g = u8::from_str_radix(&value[2..4], 16)?;
    let b = u8::from_str_radix(&value[4..6], 16)?;

    Ok(Color::Rgb(r, g, b))
}

fn mix(base: Color, overlay: Color, amount: f32) -> Color {
    let Color::Rgb(br, bg, bb) = base else {
        return base;
    };

    let Color::Rgb(or, og, ob) = overlay else {
        return overlay;
    };

    let amount = amount.clamp(0.0, 1.0);

    Color::Rgb(
        lerp(br, or, amount),
        lerp(bg, og, amount),
        lerp(bb, ob, amount),
    )
}

fn lerp(a: u8, b: u8, amount: f32) -> u8 {
    ((a as f32) + ((b as f32) - (a as f32)) * amount).round() as u8
}

fn readable_or(color: Color, background: Color, fallback: Color) -> Color {
    if contrast_ratio(color, background) >= MIN_TEXT_CONTRAST {
        return color;
    }

    if contrast_ratio(fallback, background) >= MIN_TEXT_CONTRAST {
        return fallback;
    }

    best_contrast(background)
}

fn contrast_ratio(foreground: Color, background: Color) -> f32 {
    let foreground_luminance = relative_luminance(foreground);
    let background_luminance = relative_luminance(background);

    let lighter = foreground_luminance.max(background_luminance);
    let darker = foreground_luminance.min(background_luminance);

    (lighter + 0.05) / (darker + 0.05)
}

fn relative_luminance(color: Color) -> f32 {
    let Color::Rgb(r, g, b) = color else {
        return 0.0;
    };

    let r = srgb_to_linear(r);
    let g = srgb_to_linear(g);
    let b = srgb_to_linear(b);

    0.2126 * r + 0.7152 * g + 0.0722 * b
}

fn srgb_to_linear(channel: u8) -> f32 {
    let channel = channel as f32 / 255.0;

    if channel <= 0.03928 {
        channel / 12.92
    } else {
        ((channel + 0.055) / 1.055).powf(2.4)
    }
}

fn best_contrast(color: Color) -> Color {
    let white = Color::Rgb(255, 255, 255);
    let black = Color::Rgb(0, 0, 0);

    if contrast_ratio(white, color) >= contrast_ratio(black, color) {
        white
    } else {
        black
    }
}
