use iced::{
    //font::Family,
    Color, Padding, Theme, widget::{
        Column, Rule, Scrollable, TextInput, column, container, rule::{FillMode, Style}, scrollable::{self, Id, Rail}, text_input::{self}
    }
};

use crate::{toml_files::Config, ui::app::Message};

pub fn input_with_list<'a>(
    list_column: Column<'a, Message>,
    text: &str,
    theme: &Theme,
    config: &Config,
) -> iced::Element<'a, Message> {

    let list_column = if config.show_apps == false && text.is_empty() {
       Column::new()
    }else {
        list_column
    };

    let palette = theme.palette();
    let placeholder: &String = if config.placeholder.is_empty() {
        &"Type commands, search...".to_string()
    } else {
        &config.placeholder
    };

    let divider_size: u16 = if config.divider {
        1
    }else {
        0
    };

    container(column![
            // input box on top
            TextInput::new(placeholder, text)
                .on_input(Message::SearchChanged)
                .on_submit(Message::Submit)
                .size(config.input_text_size)
                .id("input")
                .style(move |theme: &Theme, _| {
                    // custom style for input
                    let palette = theme.palette();
                    text_input::Style {
                        background: iced::Background::Color(palette.background),
                        border: iced::Border {
                            color: iced::Color::TRANSPARENT,
                            width: 0.0,
                            radius: iced::border::Radius::new(iced::Pixels(5.0)),
                        },
                        placeholder: palette.success,
                        icon: palette.text,
                        value: palette.text,
                        selection: palette.primary,
                    }
                })
                .padding(Padding {
                    top: 20.0,
                    right: 30.0,
                    bottom: 20.0,
                    left: 30.0,
                }),
            // thin line under search
            Rule::horizontal(divider_size).style(move |_theme: &Theme| Style {
                color: _theme.palette().success,
                width: divider_size,
                radius: iced::border::Radius::new(iced::Pixels(0.0)),
                fill_mode: FillMode::Full,
            }),
            // list scroll area (but no scrollbar)
            Scrollable::new(list_column).style(|_theme: &Theme, _| {
                iced::widget::scrollable::Style {
                    vertical_rail: Rail {
                        background: Some(iced::Background::Color(Color::TRANSPARENT)),
                        border: iced::Border::default(),
                        scroller: scrollable::Scroller {
                            color: Color::TRANSPARENT,
                            border: iced::Border::default(),
                        },
                    },
                    container: container::Style::default(),
                    gap: Default::default(),
                    horizontal_rail: Rail {
                        background: Some(iced::Background::Color(Color::TRANSPARENT)),
                        border: iced::Border::default(),
                        scroller: scrollable::Scroller {
                            color: Color::TRANSPARENT,
                            border: iced::Border::default(),
                        },
                    },
                }
            }).id(Id::new("scrollable"))
        ])
        .style(move |_theme: &Theme| container::Style {
            // container background = full window bg
            background: Some(iced::Background::Color(palette.background)),
            text_color: Some(palette.text),
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
        })
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
}