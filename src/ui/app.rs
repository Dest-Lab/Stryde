use iced::{Color, Element, Padding, Pixels, Settings, Size, Subscription, Task, Theme, event, keyboard::{self, Key, key::Named}, theme::Palette, widget::{Column, scrollable::{self, AbsoluteOffset, Id}, text_input}, window::{self, settings::PlatformSpecific}};

use crate::{core::apps::{model::AppList, utils::open_app}, ui::widgets::{input_with_list::{input_with_list}, list_apps::list_apps}};

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
            application_id: "stryde".into(),
            override_redirect: false,
        },
        exit_on_close_request: true,
        transparent: true,
        ..Default::default()
    };

    iced::application("Stryde", StrydeUI::update, StrydeUI::view).settings(Settings {
        id: Some("stryde".into()),
        default_text_size: Pixels::from(16),
        antialiasing: false,
        // simple text render
        fonts: vec![],
        default_font: Default::default(),
    })
    .window(window)
    .theme(StrydeUI::theme)
    .subscription(StrydeUI::subscription)
    .run_with(move || {
        let stryde = StrydeUI::new(apps);

        let focus_task = text_input::focus::<Message>("input");
        // Auto focus to input_text

        let task = Task::batch(vec![
            window::get_latest().and_then(window::gain_focus),
            // Auto focus to app
            focus_task
        ]);

        (stryde, task)
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
pub struct StrydeUI {
    text: String,
    app_list: Vec<AppList>,
    save_list: Vec<AppList>,
    selected: usize,
}

impl StrydeUI {
    fn new(app_list: Vec<AppList>) -> Self {
        // make new app state with list of apps
        Self {
            text: "".into(),
            save_list: Vec::new(),
            app_list,
            selected: 0,
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
                success: Color::from_rgb(0.306, 0.306, 0.318), // This is for divider or placeholder,
                danger: Color::from_rgb(25.0/255.0, 25.0/255.0, 28.0/255.0), // This is for selected app,
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

                if self.selected != 0 {
                    self.selected = 0;
                    return scrollable::scroll_to(Id::new("scrollable"), AbsoluteOffset { x: 0.0, y: 0.0 });
                } 
                    return Task::none();
                
            }
            Message::Open(entry_exec) => {
                    open_app(entry_exec)
            }
            Message::Submit => {
                open_app(self.app_list[self.selected].exec.clone())
            }
            Message::KeyEvent(key) => {
                match key {
                    keyboard::Key::Named(Named::Escape) => return window::get_latest().and_then(window::close),
                    // If user pressed Escape, close window
                    keyboard::Key::Named(Named::ArrowDown) => {
                        if self.selected+1 < self.app_list.len() {
                            self.selected += 1;
                            return scrollable::scroll_to(Id::new("scrollable"), AbsoluteOffset {
                                x: 0.0,
                                y: self.selected as f32 * 55.0
                            });
                        }
                        Task::none()
                    }
                    // If user pressed Arrow Down, move to the next app
                    keyboard::Key::Named(Named::ArrowUp) => {
                        if self.selected > 0 {
                            self.selected -= 1;
                            return scrollable::scroll_to(Id::new("scrollable"), AbsoluteOffset {
                                x: 0.0,
                                y:  self.selected as f32 * 55.0
                            });
                        }
                        Task::none()
                    }
                    // If user pressed Arrow Up, move to the previous app
                    _ => Task::none()
                }
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

        for (index, entry) in self.app_list.iter().enumerate() {
            list_column = list_column.push(
                Element::from(
                    list_apps(
                        entry.name.clone(),
                         entry.exec.clone(),
                          Some(entry.icon_path.clone()),
                          self.theme().clone(),
                          self.selected == index,
                        ).on_press(Message::Open(entry.exec.clone()))))
        } // Make a list with all apps
        
        input_with_list(list_column, &self.text, &self.theme())
        // Make a input, divider, list
    }
}
