use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct AppList {
    pub name: String,
    pub description: Option<String>,
    pub exec: Option<String>,
    pub icon_path: Option<std::path::PathBuf>,
    pub type_file: Option<String>,
    pub terminal: Option<bool>
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
