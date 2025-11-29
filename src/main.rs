mod core;
use crate::{core::apps::indexer::indexing, ui::app::run_ui};
mod ui;
// Main function that run parser
fn main() -> iced::Result
{
    let apps = indexing().unwrap_or_default();
    // for entry in &apps {
    //     println!("App name: {}, icon path: {:?}", entry.name, entry.icon_path);
    // }
    run_ui(apps)
}
