use iced::{Color, Element, Padding, Pixels, Settings, Size, Subscription, Task, Theme, event, keyboard::{self, Key, key::Named}, theme::Palette, widget::{Column, text_input}, window::{self, settings::PlatformSpecific}};

use crate::{core::apps::{model::AppList, utils::{open_app}}, ui::widgets::{list_apps::list_apps, text_input::input_box}};

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
        transparent: true,
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
    .subscription(FluxUI::subscription)
    .run_with(move || {
        let flux = FluxUI::new(apps);

        let focus_task = text_input::focus::<Message>("input");
        // Auto focus to input_text

        let task = Task::batch(vec![
            window::get_latest().and_then(window::gain_focus),
            // Auto focus to app
            focus_task
        ]);

        (flux, task)
    })
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchChanged(String),
    Submit,
    Open(String),
    KeyEvent(Key)
}

#[derive(Default)]
pub struct FluxUI {
    text: String,
    app_list: Vec<AppList>,
    save_list: Vec<AppList>
}

impl FluxUI {
    fn new(app_list: Vec<AppList>) -> Self {
        // make new app state with list of apps
        Self {
            text: "".into(),
            save_list: Vec::new(),
            app_list,
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        // listen for keyboard event
        event::listen_with(|event, _status, _| match event {
            iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. }) => {
                Some(Message::KeyEvent(key))
            }
            _ => None,
        })
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

    fn update(&mut self, message: Message) -> Task<Message>{
        match message {
            Message::SearchChanged(text) => {
                if !self.save_list.is_empty() {
                    self.app_list = self.save_list.clone();
                } // If save list is not empty load the save
                self.text = text;
                // Change the text_input text
                self.save_list = self.app_list.clone();
                // Save the current app list

                let mut new_list = Vec::new();
                // Make a empty list

                for entry in &self.app_list {
                    if entry.name.to_lowercase().contains(&self.text.trim().to_lowercase()) {
                        new_list.push(entry.to_owned());
                    }
                } // Push in list every app that contains input_text text
                self.app_list = new_list;
                Task::none()
            }
            Message::Open(entry_exec) => {
                open_app(entry_exec)
            }
            Message::Submit => {
                open_app(self.app_list[0].exec.clone())
            }
            Message::KeyEvent(key) => {
                // If user pressed Escape, close window
                if key == keyboard::Key::Named(Named::Escape) {
                    return window::get_latest().and_then(window::close)
                }
                Task::none()
            }
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
            list_column = list_column.push(
                Element::from(
                    list_apps(
                        entry.name.clone(),
                         entry.exec.clone(),
                          Some(entry.icon_path.clone())
                        ).on_press(Message::Open(entry.exec.clone()))))
        } // Make a list with all apps
        
        input_box(list_column, &self.text, &self.theme())
        // Make a input_text
    }
}