use std::{collections::HashMap, ffi::OsStr, fs::{self, OpenOptions}, io::Write};

use iced::{Color, Theme, keyboard::key::Named, theme::{Palette, palette::Warning}};
use serde::{Deserialize, Serialize};
use toml::{from_str, to_string};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub theme: String,
    pub antialiasing: bool,
    pub window: WindowConfig,
    pub text: TextConfig,
    pub layout: LayoutConfig,
    pub behavior: BehaviorConfig,
    pub keybinds: KeybindsConfig,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WindowConfig {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TextConfig {
    pub font_name: String,
    pub list_text_size: u16,
    pub input_text_size: u16,
    pub placeholder: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LayoutConfig {
    pub icon_size: u16,
    pub padding_vertical: f32,
    pub spacing: u16,
    pub divider: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BehaviorConfig {
    pub show_apps: bool,
    pub close_on_launch: bool,
    pub highlight_style_text: bool,
    pub default_terminal: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct KeybindsConfig {
    pub close: String,
    pub open: String,
    pub navigation: Vec<String>
}

#[derive(Debug, Clone)]
pub struct Keybinds {
    pub close: Named,
    pub open: Named,
    pub navigation: Vec<Named>
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

            window: WindowConfig { width: 774, height: 500 },

            text: TextConfig { font_name: " ".into(), list_text_size: 16, input_text_size: 18, placeholder: "Type commands, search...".into() },

            layout: LayoutConfig { icon_size: 37, padding_vertical: 0.0, spacing: 5, divider: true },

            behavior: BehaviorConfig { show_apps: true, close_on_launch: true, highlight_style_text: false, default_terminal: "kitty".into() },

            keybinds: KeybindsConfig { close: "escape".into(), open: "enter".into(), navigation: vec!["arrowup".into(), "arrowdown".into()] }
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

                let warning_colors: Warning = Warning::generate(
                hex_to_rgb(&theme.primary).unwrap_or(Color::from_rgb(137.0/255.0, 180.0/255.0, 250.0/255.0)), // base
                hex_to_rgb(&theme.background).unwrap_or
                (Color::from_rgb(0.063, 0.063, 0.071)), // background
                hex_to_rgb(&theme.text).unwrap_or(Color::WHITE), // text
                );
                // Generate warning colors

                return Some(
                    iced::Theme::custom(
                       "Stryde",
                       Palette {
                        background: hex_to_rgb(&theme.background).unwrap_or(Color::from_rgb(0.063, 0.063, 0.071)),

                        text: hex_to_rgb(&theme.text).unwrap_or(Color::WHITE),

                        primary: hex_to_rgb(&theme.primary).unwrap_or(iced::Color::from_rgb(137.0/255.0, 180.0/255.0, 250.0/255.0)),

                        success: hex_to_rgb(&theme.secondary).unwrap_or(Color::from_rgb(0.306, 0.306, 0.318)),

                        danger: hex_to_rgb(&theme.selected).unwrap_or(Color::from_rgb(25.0/255.0, 25.0/255.0, 28.0/255.0)),

                        warning: warning_colors.weak.text
                       }
                    )
                );
                // Get theme from config
            }
        }
    }
    // If config theme is Stryde-Dark (default one)
    Some(
        Theme::custom(
            "Stryde-Dark".to_string(),
            Palette {
                background: Color::from_rgb(0.063, 0.063, 0.071),
                text: Color::WHITE,
                primary: iced::Color::from_rgb(137.0/255.0, 180.0/255.0, 250.0/255.0),
                success: Color::from_rgb(0.306, 0.306, 0.318),
                danger: Color::from_rgb(25.0/255.0, 25.0/255.0, 28.0/255.0),
                warning: Color::from_rgb(216.0/255.0, 68.0/255.0, 52.0/255.0)
            },
        )
    )
    // Return default theme
}

fn hex_to_rgb(hex: &str) -> Option<Color> {
    let hex = hex.trim_start_matches('#');
    
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16);
    let g = u8::from_str_radix(&hex[2..4], 16);
    let b = u8::from_str_radix(&hex[4..6], 16);



    return Some(Color::from_rgb(r.ok()? as f32 / 255.0, g.ok()? as f32 / 255.0, b.ok()? as f32 / 255.0))
}

pub fn string_to_named_key(string_keybinds: &KeybindsConfig) -> Keybinds {
    let mut map = HashMap::new();
    map.insert("enter", Named::Enter);
    map.insert("escape", Named::Escape);
    map.insert("tab", Named::Tab);
    map.insert("arrowup", Named::ArrowUp);
    map.insert("arrowdown", Named::ArrowDown);
    map.insert("arrowleft", Named::ArrowLeft);
    map.insert("arrowright", Named::ArrowRight);
    map.insert("capslock", Named::CapsLock);
    map.insert("f1", Named::F1);
    map.insert("f2", Named::F2);
    map.insert("f3", Named::F3);
    map.insert("f4", Named::F4);
    map.insert("f5", Named::F5);
    map.insert("f6", Named::F6);
    map.insert("f7", Named::F7);
    map.insert("f8", Named::F8);
    map.insert("f9", Named::F9);
    map.insert("f10", Named::F10);
    map.insert("f11", Named::F11);
    map.insert("f12", Named::F12);
    map.insert("print", Named::Print);
    map.insert("delete", Named::Delete);
    map.insert("alt", Named::Alt);
    map.insert("numlock", Named::NumLock);
    map.insert("fn", Named::Fn);
    map.insert("control", Named::Control);
    map.insert("shift", Named::Shift);
    map.insert("super", Named::Super);
    map.insert("backspace", Named::Backspace);
    map.insert("space", Named::Space);
    map.insert("home", Named::Home);
    map.insert("end", Named::End);
    map.insert("pageup", Named::PageUp);
    map.insert("pagedown", Named::PageDown);
    map.insert("insert", Named::Insert);

    // Map of keys
    
    let close = map.get(&string_keybinds.close as &str).copied().unwrap_or(Named::Escape);
    // Get close keybind
    let open = map.get(&string_keybinds.open as &str).copied().unwrap_or(Named::Enter);
    // Get open keybind
    let navigation= vec![map.get(&string_keybinds.navigation[0] as &str).copied().unwrap_or(Named::ArrowUp), map.get(&string_keybinds.navigation[1] as &str).copied().unwrap_or(Named::ArrowDown)];
    // Get keybind for navigation

    Keybinds { close: close, open: open, navigation: navigation, }
}