mod core;
use std::{collections::HashMap, path::PathBuf};

use crate::{core::apps::{indexer::indexing, model::Handler, utils::resize_icon}, toml_files::{Config, read_theme, settings}, ui::app::run_ui};
mod ui;
mod toml_files;
fn main() -> iced::Result
{
    let apps = indexing().unwrap_or_default();
    let config: Config = settings().unwrap_or(
        Config {
            theme: "Stryde-Dark".to_string(),
            antialiasing: false,
            list_text_size: 16,
            input_text_size: 18,
            app_width: 774.0,
            app_height: 500.0,
            icon_size: 37,
            show_apps: true,
            close_on_launch: true,
            font_name: "".into(),
            placeholder: "Type commands, search...".into(),
            default_terminal: "kitty".into(),
            highlight_style_text: false,
            divider: true,
            padding_vertical: 0.0,
            spacing: 5
        }
    );
    let mut icons: HashMap<PathBuf, Handler> = HashMap::new();
    for entry in &apps {
        let ext = entry.icon_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        // Get icon extension like svg or png

        if ext == "svg" {
            let svg_handler = iced::widget::svg::Handle::from_path(entry.icon_path.clone());
            icons.insert(entry.icon_path.clone(), Handler { image_handler: None, svg_handler: Some(svg_handler) });
        }else {
            if let Some(img) = resize_icon(entry.icon_path.as_path().to_str().unwrap_or_default(), config.icon_size.into()) {
                icons.insert(entry.icon_path.clone(), Handler { image_handler: Some(img), svg_handler: None });
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
                primary: iced::Color::from_rgb(0.055, 0.122, 0.165),
                success: iced::Color::from_rgb(0.306, 0.306, 0.318),
                danger: iced::Color::from_rgb(25.0/255.0, 25.0/255.0, 28.0/255.0),
            },
        )
    );
    // Get theme if get any errors put the default one
    run_ui(apps, config, theme, icons)
}
