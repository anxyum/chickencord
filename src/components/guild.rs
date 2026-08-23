use crate::app_event::AppEvent;
use iced::{
    Background, Border, Color, Element,
    border::Radius,
    widget::{
        container,
        image::{Handle, Image},
        text,
    },
};

#[derive(Debug)]
pub struct Guild {
    id: u64,
    name: String,
    initials: String,
    avatar: Option<Handle>,
}

impl Guild {
    pub fn new(id: u64, name: String, avatar: Option<Handle>) -> Self {
        let initials = get_initials(&name);

        Self {
            id,
            name,
            initials,
            avatar,
        }
    }

    pub fn show_avatar(&self, radius: Radius, size: u32) -> Element<'_, AppEvent> {
        match &self.avatar {
            Some(avatar) => Image::new(avatar)
                .width(size)
                .height(size)
                .border_radius(radius)
                .into(),
            None => container(text(&self.initials))
                .width(size)
                .height(size)
                .style(move |_| container::Style {
                    background: Some(Background::Color(Color::BLACK)),
                    border: Border {
                        radius,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .into(),
        }
    }
}

fn get_initials(string: &str) -> String {
    string
        .split_whitespace()
        .filter_map(|word| word.chars().next())
        .collect::<String>()
}
