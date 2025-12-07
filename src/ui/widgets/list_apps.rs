use iced::{Alignment, Background, Border, Color, Length, Pixels, Shadow, Theme, widget::{Button, Row, button, image, row, svg, text}};

use crate::{core::apps::{model::Handler}, ui::app::Message};

pub fn list_apps(
    name: String,
    _exec: String,
    theme: Theme,
    selected: bool,
    highlight_text: bool,
    handlers: Handler,
    icon_size: u16,
) -> iced::widget::Button<'static, Message> {
    let mut _content: Row<'_, Message> = Row::new();

    if !handlers.image_handler.is_none() || !handlers.svg_handler.is_none() {
        // If icon exists, i show it
        if !handlers.svg_handler.is_none() {
            if let Some(svg_handle) = handlers.svg_handler.as_ref() {
                _content = row![
                    svg(svg_handle.clone())
                        .width(icon_size)
                        .height(icon_size),
                    text(name)
                ]
                .spacing(10)
                .align_y(iced::Alignment::Center);
            }
            else {
                _content = row![text(name)];
            }
            // If icon is svg, i show with svg widget
        } else {
            // If icon is not svg, use image widget
            if let Some(img_handle) = handlers.image_handler.as_ref() {      
                _content = row![image(img_handle)
                    .width(icon_size)
                    .height(icon_size),
                text(name)
            ].spacing(10);
            }else {
                _content = row![text(name)];
            }
        };

    } else {
        _content = row![text(name)];
    }
    let palette = theme.palette();
    let bg_color = if selected && !highlight_text {
        palette.danger
    }else {
        palette.background
    };
    let text_color = if selected && highlight_text {
        palette.primary
    }else {
        palette.text
    };
    Button::new(_content.align_y(Alignment::Center))
            .padding(iced::Padding {
                top: 5.0,
                left: 25.0,
                right: 0.0,
                bottom: 0.0,
            })
            .width(Length::Fill)
            .height(50)
            .style(
                move |_theme: &Theme, _status: button::Status| button::Style {
                    // button bg from theme
                    background: Some(Background::Color(bg_color)),
                    // text from theme
                    text_color: text_color,
                    // border no color and small round
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::new(Pixels(0.0)),
                    },
                    // no shadow change
                    shadow: Shadow::default(),
                },
            )
}