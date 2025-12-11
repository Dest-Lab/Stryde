use std::{collections::{HashMap}, path::PathBuf};


use iced::{Element, Font, Padding, Pixels, Settings, Size, Subscription, Task, Theme, event, keyboard::{self, Key}, theme::Palette, widget::{Column, Id, operation::{focus, scroll_to}, scrollable::{AbsoluteOffset}}, window::{self, settings::PlatformSpecific}};

use crate::{core::apps::{model::{AppList, Handler}, utils::open_app}, toml_files::{Config, Keybinds}, ui::widgets::{input_with_list::input_with_list, list_apps::list_apps}};

pub fn run_ui(apps: Vec<AppList>, settings: Config, theme: Theme, handlers: HashMap<PathBuf, Handler>, keybinds: Keybinds) -> iced::Result{
    let font_name = if !settings.text.font_name.is_empty() {
        Box::leak(settings.text.font_name.clone().into_boxed_str())
        // 0.03-0.05 KB memory leak :(
    }else {
        "" // Use default font and no memory leak :)
    };
    let window = window::Settings {
        size: Size {
            width: settings.window.width as f32,
            height: settings.window.height as f32,
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
            #[cfg(target_os = "linux")]
            application_id: "stryde".into(),
            #[cfg(target_os = "linux")]
            override_redirect: false,

            ..Default::default()
        },
        exit_on_close_request: true,
        transparent: true,
        blur: true,
        min_size: Some(Size {
            width: 774.0,
            height: 500.0,
        }),
        ..Default::default()
    };

    let antialiasing = settings.antialiasing.clone();
    let list_text_size = settings.text.list_text_size.clone() as u32;

    iced::application(
         move || {
            let stryde = StrydeUI::new(apps.to_owned(), theme.to_owned(), settings.to_owned(), handlers.to_owned(), keybinds.to_owned());

        let focus_task = focus::<Message>("input");
        // Auto focus to input_text

        let task = Task::batch(vec![
            window::latest().and_then(window::gain_focus),
            // Auto focus to app
            focus_task
        ]);
        (stryde, task)
        },
        StrydeUI::update, StrydeUI::view).settings(Settings {
        id: Some("stryde".into()),
        default_text_size: Pixels::from(list_text_size),
        antialiasing: antialiasing,
        vsync: true,
        // simple text render
        fonts: vec![],
        default_font: Font::with_name(font_name)})
    .window(window)
    .theme(StrydeUI::theme)
    .subscription(StrydeUI::subscription)
    .title("Stryde")
    .run()
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchChanged(String),
    Open(String, bool, String, bool),
    KeyEvent(Key)
}

pub struct StrydeUI {
    text: String,
    app_list: Vec<AppList>,
    selected: usize,
    theme: Theme,
    config: Config,
    handlers: HashMap<PathBuf, Handler>,
    keybinds_custom: Keybinds
}

impl StrydeUI {
    fn new(app_list: Vec<AppList>, theme: Theme, config: Config, handlers: HashMap<PathBuf, Handler>, keybinds: Keybinds) -> Self {
        // make new app state with list of apps
        Self {
            text: "".into(),
            app_list,
            selected: 0,
            theme: theme,
            config: config,
            handlers: handlers,
            keybinds_custom: keybinds
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
                warning: pallete.warning
            },
        )
    }

    fn update(&mut self, message: Message) -> Task<Message>{
        match message {
            Message::SearchChanged(text) => {
                self.text = text;
                if self.selected != 0 {
                    self.selected = 0;
                    return scroll_to(Id::new("scrollable"), AbsoluteOffset { x: 0.0, y: 0.0 });
                } 
                    return Task::none();
                
            }
            Message::Open(entry_exec, close_after_launch, default_terminal, terminal) => {
                    open_app(entry_exec, close_after_launch, default_terminal, terminal)
            }
            Message::KeyEvent(key) => {
                match key {
                    keyboard::Key::Named(named_key) => {
                        if named_key == self.keybinds_custom.close {
                            return window::latest().and_then(window::close)
                        }
                        if named_key == self.keybinds_custom.navigation[0]{
                            if self.selected > 0 {
                                self.selected -= 1;
                                return scroll_to(Id::new("scrollable"), AbsoluteOffset {
                                x: 0.0,
                                y:  self.selected as f32 * (50.0 + self.config.layout.spacing as f32)
                            });
                            }
                        }
                        if named_key == self.keybinds_custom.navigation[1] {
                            if self.selected+1 < self.app_list.len() {
                                self.selected += 1;
                                return scroll_to(Id::new("scrollable"), AbsoluteOffset {
                                x: 0.0,
                                y: self.selected as f32 * (50.0 + self.config.layout.spacing as f32)
                            });
                            }
                        }
                        if named_key == self.keybinds_custom.open {
                            let filtered: Vec<&AppList> = self.app_list.iter().filter(|app| {
                                app.name.to_lowercase().contains(&self.text.to_lowercase())
                            }).collect();

                            return open_app(filtered[self.selected].exec.as_ref().expect("").clone(), self.config.behavior.close_on_launch, self.config.behavior.default_terminal.clone(), filtered[self.selected].terminal.clone().unwrap_or(false));
                        }
                        return Task::none()
                    },
                    _ => Task::none()
                }
            }
        }
    }
    fn view(&self) -> iced::Element<'_, Message> {
        let mut list_column = Column::new().spacing(self.config.layout.spacing as u32).padding(
            Padding {
                top: self.config.layout.padding_vertical,
                left: 0.0,
                right: 0.0,
                bottom: self.config.layout.padding_vertical
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
                         entry.exec.as_ref().expect("").clone(),
                          self.theme().clone(),
                          self.selected == index,
                          self.config.behavior.highlight_style_text,
                          self.handlers.get(entry.icon_path.as_ref().unwrap_or(&PathBuf::from("")).as_path()).unwrap_or(&Handler { image_handler: None, svg_handler: None }).clone(),
                          self.config.layout.icon_size,
                        ).on_press(Message::Open(entry.exec.as_ref().expect("").clone(), self.config.behavior.close_on_launch, self.config.behavior.default_terminal.clone(), entry.terminal.unwrap_or(false).clone()))))
        } // Make a list with all apps
        
        input_with_list(list_column, &self.text, &self.theme(), &self.config)
        // Make a input, divider, list
    }
}