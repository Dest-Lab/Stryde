use std::{collections::HashMap, path::PathBuf};

use iced::{Element, Font, Padding, Pixels, Settings, Size, Subscription, Task, Theme, event, keyboard::{self, Key, key::Named}, theme::Palette, widget::{Column, scrollable::{self, AbsoluteOffset, Id}, text_input}, window::{self, settings::PlatformSpecific}};

use crate::{core::apps::{model::{AppList, Handler}, utils::open_app}, toml_files::Config, ui::widgets::{input_with_list::input_with_list, list_apps::list_apps}};

pub fn run_ui(apps: Vec<AppList>, settings: Config, theme: Theme, handlers: HashMap<PathBuf, Handler>,) -> iced::Result{
    let font_name = if !settings.font_name.is_empty() {
        Box::leak(settings.font_name.clone().into_boxed_str())
        // 0.03-0.05 KB memory leak :(
    }else {
        "" // Use default font and no memory leak :)
    };
    let window = window::Settings {
        size: Size {
            width: settings.app_width,
            height: settings.app_height,
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
        min_size: Some(Size {
            width: 774.0,
            height: 500.0,
        }),
        ..Default::default()
    };

    iced::application("Stryde", StrydeUI::update, StrydeUI::view).settings(Settings {
        id: Some("stryde".into()),
        default_text_size: Pixels::from(settings.list_text_size),
        antialiasing: settings.antialiasing,
        // simple text render
        fonts: vec![],
        default_font: Font {
                family: iced::font::Family::Name(font_name),
                weight: iced::font::Weight::Normal,
                stretch: iced::font::Stretch::Normal,
                style: iced::font::Style::Normal,
    }})
    .window(window)
    .theme(StrydeUI::theme)
    .subscription(StrydeUI::subscription)
    .run_with(move || {
        let stryde = StrydeUI::new(apps, theme, settings, handlers);

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
    Open(String, bool, String, bool),
    KeyEvent(Key)
}

#[derive(Default)]
pub struct StrydeUI {
    text: String,
    app_list: Vec<AppList>,
    selected: usize,
    theme: Theme,
    config: Config,
    handlers: HashMap<PathBuf, Handler>,
}

impl StrydeUI {
    fn new(app_list: Vec<AppList>, theme: Theme, config: Config, handlers: HashMap<PathBuf, Handler>) -> Self {
        // make new app state with list of apps
        Self {
            text: "".into(),
            app_list,
            selected: 0,
            theme: theme,
            config: config,
            handlers: handlers
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
        let pallete = self.theme.palette();
        // custom dark theme
        Theme::custom(
            "Stryde".to_string(),
            Palette {
                background: pallete.background,
                text: pallete.text,
                primary: pallete.primary,
                success: pallete.success,
                danger: pallete.danger,
            },
        )
    }

    fn update(&mut self, message: Message) -> Task<Message>{
        match message {
            Message::SearchChanged(text) => {
                self.text = text;
                if self.selected != 0 {
                    self.selected = 0;
                    return scrollable::scroll_to(Id::new("scrollable"), AbsoluteOffset { x: 0.0, y: 0.0 });
                } 
                    return Task::none();
                
            }
            Message::Open(entry_exec, close_after_launch, default_terminal, terminal) => {
                    open_app(entry_exec, close_after_launch, default_terminal, terminal)
            }
            Message::Submit => {

                let filtered: Vec<&AppList> = self.app_list.iter().filter(|app| {
                    app.name.to_lowercase().contains(&self.text.to_lowercase())
                }).collect();

                open_app(filtered[self.selected].exec.clone(), self.config.close_on_launch, self.config.default_terminal.clone(), filtered[self.selected].terminal.clone())
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
                                y: self.selected as f32 * (50.0 + self.config.spacing as f32)
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
                                y:  self.selected as f32 * (50.0 + self.config.spacing as f32)
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
        let mut list_column = Column::new().spacing(self.config.spacing).padding(
            Padding {
                top: self.config.padding_vertical,
                left: 0.0,
                right: 0.0,
                bottom: self.config.padding_vertical
            }
        );

        let filtered = self.app_list.iter().filter(|app| {
                app.name.to_lowercase().contains(&self.text.to_lowercase())
        });

        for (index, entry) in filtered.enumerate() {
            list_column = list_column.push(
                Element::from(
                    list_apps(
                        entry.name.clone(),
                         entry.exec.clone(),
                          self.theme().clone(),
                          self.selected == index,
                          self.config.highlight_style_text,
                          self.handlers.get(&entry.icon_path).unwrap_or(&Handler { image_handler: None, svg_handler: None }).clone(),
                          self.config.icon_size,
                        ).on_press(Message::Open(entry.exec.clone(), self.config.close_on_launch, self.config.default_terminal.clone(), entry.terminal.clone()))))
        } // Make a list with all apps
        
        input_with_list(list_column, &self.text, &self.theme(), &self.config)
        // Make a input, divider, list
    }
}