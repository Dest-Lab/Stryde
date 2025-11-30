use std::path::PathBuf;

use iced::{Background, Border, Color, Length, Pixels, Shadow, Theme, widget::{Button, Row, button, image, row, svg, text}};

use crate::{core::apps::utils::resize_icon, ui::app::Message};

pub fn list_apps(
    name: String,
    _exec: String,
    icon: Option<PathBuf>,
) -> iced::widget::Button<'static, Message> {
    let mut _content: Row<'_, Message> = Row::new();

    if !icon.is_none() {
        // If icon exists, i show it
        let path = icon.as_ref().expect("");

        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        // Get icon extension like svg or png

        if ext == "svg" {
            _content = row![
                svg(path)
                    .width(40)
                    .height(40),
                text(name)
            ]
            .spacing(10)
            .align_y(iced::Alignment::Center);
        // If icon is svg, i show with svg widget
        } else {
            // If icon is not svg, use image widget
            if let Some(img) = resize_icon(path.as_path().to_str().unwrap_or_default(), 40){      
                _content = row![image(img)
                    .width(40)
                    .height(40),
                text(name)
            ].spacing(10).align_y(iced::Alignment::Center);
            }else {
                _content = row![text(name)].align_y(iced::Alignment::Center);
            }
        };

    } else {
        _content = row![text(name)].align_y(iced::Alignment::Center);
    }
    Button::new(_content)
            .padding(iced::Padding {
                top: 15.0,
                left: 25.0,
                right: 0.0,
                bottom: 0.0,
            })
            .width(Length::Fill)
            .height(50)
            .style(
                move |_theme: &Theme, _status: button::Status| button::Style {
                    // button bg is dark gray
                    background: Some(Background::Color(Color::from_rgb(0.063, 0.063, 0.071))),
                    // text is white
                    text_color: Color::WHITE,
                    // border no color and small round
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::new(Pixels(5.0)),
                    },
                    // no shadow change
                    shadow: Shadow::default(),
                },
            )
}