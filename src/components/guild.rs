use super::{Cache, Channels};
use crate::{
    Context,
    app_event::{AppEvent, AppMessage},
    components::button,
};
use iced::{
    Background, Border, Color, Element, Length, alignment,
    border::Radius,
    widget::{
        column, container,
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
        context: &Context,
        radius: Radius,
        size: impl Into<Length> + Copy,
    ) -> Element<'_, AppEvent> {
        match &self.avatar {
            Some(avatar) => {
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
                let placeholder = context.theme.guilds.placeholder_background;
                let text = text(&self.initials)
                    .font(crate::GG_SANS_REGULAR)
                    .color(Color::WHITE);
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

    pub fn show_clickable_avatar(
        &self,
        context: &Context,
        radius: Radius,
        size: impl Into<Length> + Copy,
    ) -> Element<'_, AppEvent> {
        button(self.show_avatar(context, radius, size))
            .on_press(AppEvent::Message(AppMessage::OpenGuild(self.id)))
            .into()
    }

    pub fn show_pannel<'a>(
        &'a self,
        cache: &'a Cache,
        channels: &'a Channels,
        context: &'a Context,
        panel_width: f32,
    ) -> Element<'a, AppEvent> {
        column([
            container(text(&self.name).size(16.0)).padding(8.0).into(),
            container("")
                .width(Length::Fill)
                .height(context.theme.border_size)
                .style(|_| container::Style::default().background(context.theme.border_color))
                .into(),
            channels.show_channels(cache, context),
        ])
        .width(panel_width)
        .into()
    }
}

fn get_initials(string: &str) -> String {
    string
        .split_whitespace()
        .filter_map(|word| word.chars().next())
        .collect::<String>()
}
