use std::{fs, path::{Path, PathBuf}, time::UNIX_EPOCH};

use freedesktop_icons::lookup;
use iced::{Task, widget, window};
use image::ImageReader;

use crate::{ui::app::{ Message}};

pub fn open_app(entry_exec: String) -> Task<Message> {
    let exec = entry_exec.clone();
    let path = std::path::Path::new(&exec);
    if path.exists() && path.is_file() {
        // If file exists, try run it
        if let Err(e) = std::process::Command::new(path).spawn() {
            println!("Failed to open {:?}: {}",path, e);
            // Print error is cannot open
        }
    } else {
        if let Err(e) = std::process::Command::new("sh").arg("-c").arg(&exec).spawn() {
           println!("Failed to open {:?}: {}", path, e);
            // Print error is cannot open 
        }
    }
    window::get_latest().and_then(window::close)
}

pub fn last_modified(path: &Path) -> Option<u64> {
    let metadata = fs::metadata(path).ok()?;
    let modified = metadata.modified().ok()?;
    let duration = modified.duration_since(UNIX_EPOCH).ok()?;
    Some(duration.as_secs())
}

pub fn get_icon_path(icon_name: &str) -> Option<PathBuf> {
    // Search direct in breeze for apps
    for size in [32, 48, 64] {
        let direct_path = PathBuf::from(format!(
            "/usr/share/icons/breeze/apps/{}/{}.svg",
            size, icon_name
        ));
        if direct_path.exists() {
            return Some(direct_path);
        }
    }

    // Then specific themes
    for theme in ["Breeze", "Papirus", "Adwaita"] {
        for size in [64, 48, 32] {
            if let Some(path) = lookup(icon_name)
                .with_theme(theme)
                .with_size(size)
                .find() {
                return Some(path);
            }
        }
    }
    
    // Try symbolic
    let symbolic = format!("{}-symbolic", icon_name);
    if let Some(path) = lookup(&symbolic).with_size(32).find() {
        return Some(path);
    }
    // Generic fallback
    lookup("application-x-executable").with_size(48).find()
}

pub fn resize_icon(path: &str, size: u32) -> iced::widget::image::Handle {
    let img = ImageReader::open(path)
        .unwrap()
        .decode()
        .unwrap_or_default()
        .to_rgba8();

    let resized = image::imageops::resize(&img, size, size, image::imageops::FilterType::Lanczos3);

    widget::image::Handle::from_rgba(resized.width(), resized.height(), resized.into_raw())
}