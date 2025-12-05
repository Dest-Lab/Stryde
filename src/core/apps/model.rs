use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppList {
    pub name: String,
    pub description: String,
    pub exec: String,
    pub icon_path: std::path::PathBuf,
    pub type_file: String,
}

#[derive(Clone)]
pub struct Handler {
    pub image_handler: Option<iced::widget::image::Handle>,
    pub svg_handler: Option<iced::widget::svg::Handle>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CacheFile {
    pub hash: u64,
    pub apps: Vec<AppList>
}