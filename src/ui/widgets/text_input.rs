use iced::{
    //font::Family,
    Color, Padding, Theme, widget::{
        Column, Rule, TextInput, column, container, rule::{FillMode, Style}, scrollable::{self, Rail}, text_input::{self}, Scrollable
    },
};

use crate::ui::app::Message;

pub fn input_box<'a>(
    list_column: Column<'a, Message>,
    text: &str,
    theme: &Theme
) -> iced::Element<'a, Message> {

    let palette = theme.palette();

    container(column![
            // input box on top
            TextInput::new("Type a command or search...", text)
                .on_input(Message::SearchChanged)
                .on_submit(Message::Submit)
                .size(18)
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
            Rule::horizontal(1).style(|_theme: &Theme| Style {
                color: Color::from_rgb(35.0 / 255.0, 35.0 / 255.0, 39.0 / 255.0),
                width: 1,
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
            })
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