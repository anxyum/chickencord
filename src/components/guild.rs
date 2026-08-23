use crate::{app_event::AppEvent, themes::GuildsTheme};
use iced::{
    Background, Border, Color, Element, Length, alignment,
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

    pub fn show_avatar(
        &self,
        theme: &GuildsTheme,
        radius: Radius,
        size: impl Into<Length> + Copy,
    ) -> Element<'_, AppEvent> {
        match &self.avatar {
            Some(avatar) => {
                // iced_wgpu applique les rayons d'image avec un décalage diagonal
                // (négation manquante dans shader/image.wgsl contrairement aux quads)
                let radius = Radius {
                    top_left: radius.bottom_right,
                    top_right: radius.bottom_left,
                    bottom_right: radius.top_left,
                    bottom_left: radius.top_right,
                };

                Image::new(avatar)
                    .width(size)
                    .height(size)
                    .border_radius(radius)
                    .into()
            }
            None => {
                let placeholder = theme.placeholder_background;
                let text = text(&self.initials).color(Color::WHITE);
                container(text)
                    .width(size)
                    .height(size)
                    .align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Center)
                    .style(move |_| container::Style {
                        background: Some(Background::Color(placeholder)),
                        border: Border {
                            radius,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .into()
            }
        }
    }
}

fn get_initials(string: &str) -> String {
    string
        .split_whitespace()
        .filter_map(|word| word.chars().next())
        .collect::<String>()
}
