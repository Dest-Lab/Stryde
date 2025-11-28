use std::{ffi::OsStr, fs, path::PathBuf};

use freedesktop_file_parser::EntryType;

use crate::core::apps::{model::AppList, scanner::scan_desktop_files};

pub fn parse_data() -> Vec<AppList> {
    let desktops_paths: Vec<PathBuf> = scan_desktop_files();
    let mut apps_info: Vec<AppList> = Vec::new();
    for entry in &desktops_paths {

        if entry.extension() != Some(OsStr::new("desktop")) || entry.is_dir()
        {
            continue;
        }
        // Move to the next if current is not a .desktop file
        
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

        let icon = desktop_file.entry.icon;
        // Get icon of the app in .desktop file

        if let EntryType::Application(app) = &desktop_file.entry.entry_type {
            let mut exec = match &app.exec {
                Some(exec) => exec.clone(),
                None => continue,
            };
            // Get exec command of the app in .desktop file

            for arg in ["%u", "%f", "%U", "%F", "%i", "%c", "%k"] {
                exec = exec.replace(arg, "");
            }
            // Remove freedesktop exec placeholders

            apps_info.push(
                AppList {
                    name: name,
                    description: description.unwrap_or_default().default,
                    exec: exec,
                    icon_path: icon.unwrap_or_default().content.into(),
                    type_file: "Application".to_string()
                }
            );
            // Push app in list of apps
        }
    }
    apps_info
}
