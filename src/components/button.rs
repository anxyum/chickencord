use crate::app_event::AppEvent;
use iced::{
    Border, Color, Element, Shadow,
    widget::{Button, button as iced_button},
};

pub fn button<'a>(content: impl Into<Element<'a, AppEvent>>) -> Button<'a, AppEvent> {
    iced_button(content)
        .padding(0)
        .style(|_, _| iced_button::Style {
            background: None,
            border: Border::default(),
            shadow: Shadow::default(),
            text_color: Color::WHITE,
            snap: false,
        })
}
