use std::{ffi::OsStr, fs::{self, OpenOptions}, io::Write};

use iced::{Color, Theme, theme::Palette};
use serde::{Deserialize, Serialize};
use toml::{from_str, to_string};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub theme: String,
    pub antialiasing: bool,
    pub list_text_size: u16,
    pub input_text_size: u16,
    pub app_width: f32,
    pub app_height: f32,
    pub icon_size: u16,
    pub show_apps: bool,
    pub close_on_launch: bool,
    pub font_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CurrentTheme {
    background: String,
    text: String,
    primary: String,
    secondary: String,
    selected: String
}

pub fn settings() -> Option<Config>{
    let config_dir = dirs::config_dir()?.join("stryde");  
    // Stryde config dir
    let config_path = config_dir.join("config.toml");
    // Stryde config file
    let themes_path = config_dir.join("themes");
    // Stryde themes dir
    if !themes_path.exists() {
        let _ = fs::create_dir_all(themes_path);
        // If themes dir doesn't exists, create themes dir
    }
    if !config_path.exists() {
        // If config file doesn't exists
        let config = Config {
            theme: "Stryde-Dark".into(),
            antialiasing: false,
            list_text_size: 16,
            input_text_size: 18,
            app_width: 774.0,
            app_height: 500.0,
            icon_size: 37,
            show_apps: true,
            close_on_launch: true,
            font_name: " ".into()
        };
        // Default settings

        let toml_string = to_string(&config).ok()?;

        let mut file = OpenOptions::new().write(true).truncate(true).create(true).open(config_path).ok()?;
        file.write_all(toml_string.as_bytes()).ok()?;
        // Create file with this config
        Some(config)
    }else {
        // If config file exists
        let content = fs::read_to_string(config_path).ok()?;
        // Get content
        let config: Config = from_str(&content).ok()?;
        // Transform in Struct
        Some(config)
    }
}

pub fn read_theme(using_theme: &str) -> Option<iced::Theme>{
    let using = std::path::PathBuf::from(using_theme);
    // Theme that set in config file
    let themes_path = dirs::config_dir()?.join("stryde/themes");
    // path of theme dir
    if using_theme != "Stryde-Dark"{
        // If config theme is not Stryde-Dark
        for entry in fs::read_dir(themes_path).ok()? {
            // every file that is in theme dir
            let entry = entry.ok()?;
            if entry.path().is_file() && entry.file_name() == OsStr::new(&using.file_name().unwrap_or_default()) && entry.path().extension() == using.extension(){
                // If entry is file and entry file name is equal to config and both has extension .toml
                let content = fs::read_to_string(entry.path()).ok()?;
                // Get content
                let theme: CurrentTheme = from_str(&content).ok()?;
                // Transform in Struct
                return Some(
                    iced::Theme::custom(
                       "Stryde".into(),
                       Palette {
                        background: Color::parse(&theme.background).unwrap_or(Color::from_rgb(0.063, 0.063, 0.071)),
                        text: Color::parse(&theme.text).unwrap_or(Color::WHITE),
                        primary: Color::parse(&theme.primary).unwrap_or(
                            Color::from_rgb(0.055, 0.122, 0.165)
                        ),
                        success: Color::parse(&theme.secondary).unwrap_or(Color::from_rgb(0.306, 0.306, 0.318)),
                        danger: Color::parse(&theme.selected).unwrap_or(
                            Color::from_rgb(25.0/255.0, 25.0/255.0, 28.0/255.0)
                        ),
                       }
                    )
                );
                // Return theme
            }
        }
    }
    // If config theme is tryde-Dark
    Some(
        Theme::custom(
            "Stryde-Dark".to_string(),
            Palette {
                background: Color::from_rgb(0.063, 0.063, 0.071),
                text: Color::WHITE,
                primary: Color::from_rgb(0.055, 0.122, 0.165),
                success: Color::from_rgb(0.306, 0.306, 0.318),
                danger: Color::from_rgb(25.0/255.0, 25.0/255.0, 28.0/255.0),
            },
        )
    )
    // Return Stryde default theme
}