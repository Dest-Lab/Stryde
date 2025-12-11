use std::{fs};

use crate::core::apps::{model::AppList};

#[cfg(target_os = "linux")]
use crate::core::apps::{utils::{flatpak_apps, get_icon_path}};
#[cfg(target_os = "linux")]
use freedesktop_file_parser::EntryType;
#[cfg(target_os = "linux")]
use crate::core::apps::platforms::linux::get_all_apps;

#[cfg(target_os = "windows")]
use crate::core::apps::platforms::windows::get_windows_apps;

pub fn parse_data() -> Vec<AppList> {
    let mut apps_info: Vec<AppList> = Vec::new();
    #[cfg(target_os = "linux")]
    if cfg!(target_os = "linux"){
        let mut apps = get_all_apps().unwrap_or_default();
        apps.extend(flatpak_apps().unwrap_or_default());
        for entry in &apps {
            let content = match fs::read_to_string(entry) {
                Ok(content) => content,
                Err(_) => continue,
            };
            // Get content from .desktop file
            let desktop_file = match freedesktop_file_parser::parse(&content) {
                Ok(file) => file,
                Err(_) => continue,
            };
            // Get all attr from desktop file
        
            if desktop_file.entry.hidden == Some(true) || desktop_file.entry.no_display == Some(true) {
                continue;
            }
            // If app in .desktop file is hidden, skip app
        
            let name = desktop_file.entry.name.default;
            // Get name of the app in .desktop file
        
            let description = desktop_file.entry.comment;
            // Get description of the app in .desktop file
        
            let icon_name = desktop_file.entry.icon.unwrap_or_default().content.trim().to_string();
            // Get icon of the app in .desktop file
        
            let icon_path = get_icon_path(&icon_name);
        
        
            if let EntryType::Application(app) = &desktop_file.entry.entry_type {
                let mut exec = match &app.exec {
                    Some(exec) => exec.clone(),
                    None => continue,
                };
                let terminal = match &app.terminal {
                    Some(terminal) => terminal.clone(),
                    None => false
                };
                    // Get exec command of the app in .desktop file
        
                for arg in ["%u", "%f", "%U", "%F", "%i", "%c", "%k"] {
                    exec = exec.replace(arg, "");
                }
                    // Remove freedesktop exec placeholders
        
                    apps_info.push(
                        AppList {
                            name: name,
                            description: Some(description.unwrap_or_default().default),
                            exec: Some(exec),
                            icon_path: Some(icon_path.unwrap_or_default()),
                            type_file: Some(desktop_file.entry.entry_type.to_string()),
                            terminal: Some(terminal)
                        }
                    );
                    // Push app in list of apps
                }
            }
    }
    #[cfg(target_os = "windows")]
    if cfg!(target_os = "windows") {
        apps_info = get_windows_apps().unwrap_or_default();
    }
    apps_info
}
