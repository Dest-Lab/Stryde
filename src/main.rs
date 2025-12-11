mod core;
use std::{collections::HashMap, path::PathBuf};

use crate::{core::apps::{indexer::indexing, model::Handler, utils::resize_icon}, toml_files::{BehaviorConfig, Config, Keybinds, KeybindsConfig, LayoutConfig, TextConfig, WindowConfig, read_theme, settings, string_to_named_key}, ui::app::run_ui};
mod ui;
mod toml_files;
fn main() -> iced::Result
{
    let apps = indexing().unwrap_or_default();
    let config: Config = settings().unwrap_or(
        Config {
            theme: "Stryde-Dark".into(),

            antialiasing: false,

            window: WindowConfig { width: 774, height: 500 },

            text: TextConfig { font_name: " ".into(), list_text_size: 16, input_text_size: 18, placeholder: "Type commands, search...".into() },

            layout: LayoutConfig { icon_size: 37, padding_vertical: 0.0, spacing: 5, divider: true },

            behavior: BehaviorConfig { show_apps: true, close_on_launch: true, highlight_style_text: false, default_terminal: "kitty".into() },
            
            keybinds: KeybindsConfig { close: "escape".into(), open: "enter".into(), navigation: vec!["arrowup".into(), "arrowdown".into()] }
        }
    );
    let keybinds: Keybinds = string_to_named_key(&config.keybinds);
    let mut icons: HashMap<PathBuf, Handler> = HashMap::new();
    for entry in &apps {
        let ext = entry.icon_path.as_ref().map(|p| p.extension().and_then(|e| e.to_str()).unwrap_or("")).unwrap_or("");
        // Get icon extension like svg or png

        if ext == "svg" {
            if let Some(path) = entry.icon_path.as_ref() {
                let svg_handler = iced::widget::svg::Handle::from_path(path.clone());
                icons.insert(path.clone(), Handler { image_handler: None, svg_handler: Some(svg_handler) });
            }
        }else {
            if let Some(path) = entry.icon_path.as_ref() {
                if let Some(img) = resize_icon(path.as_path().to_str().unwrap_or_default(), config.layout.icon_size.into()) {
                    icons.insert(path.clone(), Handler { image_handler: Some(img), svg_handler: None });
                }
            }
        }
    }
    // Get settings if get any errors put the default one
    let theme = read_theme(&config.theme).unwrap_or(
        iced::Theme::custom(
            "Stryde-Dark".to_string(),
            iced::theme::Palette {
                background: iced::Color::from_rgb(0.063, 0.063, 0.071),
                text: iced::Color::WHITE,
                primary: iced::Color::from_rgb(137.0/255.0, 180.0/255.0, 250.0/255.0),
                success: iced::Color::from_rgb(0.306, 0.306, 0.318),
                danger: iced::Color::from_rgb(25.0/255.0, 25.0/255.0, 28.0/255.0),
                warning: iced::Color::from_rgb(216.0/255.0, 68.0/255.0, 52.0/255.0)
            },
        )
    );
    // Get theme if get any errors put the default one
    run_ui(apps, config, theme, icons, keybinds)
}
