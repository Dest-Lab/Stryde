use iced::{Color, Element, Padding, Pixels, Settings, Size, Task, Theme, theme::Palette, widget::Column, window::{self, settings::PlatformSpecific}};

use crate::{core::apps::model::AppList, ui::widgets::{list_apps::list_apps, text_input::input_box}};

pub fn run_ui(apps: Vec<AppList>) -> iced::Result{

    let window = window::Settings {
        size: Size {
            width: 774.0,
            height: 500.0,
        },
        // Set window size
        position: window::Position::Centered,
        // put window in center
        resizable: false,
        decorations: false,
        // no title bar
        level: window::Level::AlwaysOnTop,
        // window always on top
        platform_specific: PlatformSpecific {
            application_id: "flux".into(),
            override_redirect: false,
        },
        exit_on_close_request: true,
        // close window = exit program
        ..Default::default()
    };

    iced::application("Flux", FluxUI::update, FluxUI::view).settings(Settings {
        id: Some("flux".into()),
        default_text_size: Pixels::from(16),
        antialiasing: false,
        // simple text render
        fonts: vec![],
        default_font: Default::default(),
    })
    .window(window)
    .theme(FluxUI::theme)
    .run_with(move || {
        let flux = FluxUI::new(apps);
        (flux, Task::none())
    })
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchChanged(String),
    // Submit,
    // Open(AppList)
}

#[derive(Default)]
pub struct FluxUI {
    text: String,
    app_list: Vec<AppList>
}

impl FluxUI {

    fn new(app_list: Vec<AppList>) -> Self {
        // make new app state with list of apps
        Self {
            text: "".into(),
            app_list,
        }
    }

    fn theme(&self) -> Theme {
        // custom dark theme
        Theme::custom(
            "Dark".to_string(),
            Palette {
                background: Color::from_rgb(0.063, 0.063, 0.071),
                text: Color::WHITE,
                primary: Color::from_rgb(0.055, 0.122, 0.165),
                success: Color::from_rgb(0.306, 0.306, 0.318),
                danger: Color::from_rgb(0.839, 0.141, 0.153),
            },
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SearchChanged(text) => {
                self.text = text; // Change the text_input text
            },
        }
    }
    fn view(&self) -> iced::Element<'_, Message> {
        let mut list_column = Column::new().spacing(5).padding(
            Padding {
                top: 0.0,
                left: 0.0,
                right: 0.0,
                bottom: 10.0
            }
        );

        for entry in &self.app_list {
            list_column = list_column.push(Element::from(list_apps(entry.name.clone(), entry.exec.clone(), Some(entry.icon_path.clone()))))
        }
        
        input_box(list_column, &self.text, &self.theme())
    }
}