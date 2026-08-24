use std::collections::HashMap;

use super::channel::{Channel, GuildChannel};
use crate::{
    app_event::{AppEvent, AppMessage},
    themes::{ChannelsTheme, GuildsTheme},
};
use iced::{
    Background, Border, Color, Element, Length, Shadow, alignment,
    border::Radius,
    widget::{
        button, column, container,
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
    channels: HashMap<u64, GuildChannel>,
    channels_order: Vec<u64>,
    categories_order: Vec<u64>,
}

impl Guild {
    pub fn new(
        id: u64,
        name: String,
        avatar: Option<Handle>,
        channels: HashMap<u64, GuildChannel>,
    ) -> Self {
        let initials = get_initials(&name);

        Self {
            id,
            name,
            initials,
            avatar,
            channels,
            channels_order: Vec::new(),
            categories_order: Vec::new(),
        }
        .organize_channels()
    }

    fn organize_channels(mut self) -> Self {
        self.channels_order.clear();
        self.categories_order.clear();

        for channel in self.channels.values_mut() {
            if let GuildChannel::Category(category) = channel {
                category.children.clear();
            }
        }

        let mut child_channels = Vec::new();

        for (id, channel) in &self.channels {
            match channel.base().parent_id {
                Some(parent_id) => child_channels.push((*id, parent_id.get())),
                None => match channel {
                    GuildChannel::Category(_) => self.categories_order.push(*id),
                    _ => self.channels_order.push(*id),
                },
            }
        }

        for (id, parent_id) in child_channels {
            match self.channels.get_mut(&parent_id) {
                Some(GuildChannel::Category(category)) => category.children.push(id),

                _ => self.channels_order.push(id),
            }
        }

        let positions: HashMap<u64, i64> = self
            .channels
            .iter()
            .map(|(id, channel)| (*id, channel.base().position))
            .collect();

        let by_position = |id: &u64| positions.get(id).copied();

        self.channels_order.sort_by_key(by_position);
        self.categories_order.sort_by_key(by_position);

        for channel in self.channels.values_mut() {
            if let GuildChannel::Category(category) = channel {
                category.children.sort_by_key(by_position);
            }
        }

        self
    }

    pub fn show_avatar(
        &self,
        theme: &GuildsTheme,
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

    pub fn show_clickable_avatar(
        &self,
        theme: &GuildsTheme,
        radius: Radius,
        size: impl Into<Length> + Copy,
    ) -> Element<'_, AppEvent> {
        button(self.show_avatar(theme, radius, size))
            .padding(0)
            .style(|_, _| button::Style {
                background: None,
                border: Border::default(),
                shadow: Shadow::default(),
                text_color: Color::WHITE,
                snap: false,
            })
            .on_press(AppEvent::Message(AppMessage::OpenGuild(self.id)))
            .into()
    }

    pub fn show_channels(&self, theme: &ChannelsTheme) -> Element<'_, AppEvent> {
        let uncategorized = self
            .channels_order
            .iter()
            .filter_map(|id| Some(self.show_channel(self.channels.get(id)?, theme)));

        let categorized = self.categories_order.iter().filter_map(|id| {
            let channel = self.channels.get(id)?;
            let GuildChannel::Category(category) = channel else {
                return None;
            };

            let children = category
                .children
                .iter()
                .filter_map(|id| Some(self.show_channel(self.channels.get(id)?, theme)));

            Some(column(std::iter::once(self.show_channel(channel, theme)).chain(children)).into())
        });

        column(uncategorized.chain(categorized)).into()
    }

    fn show_channel<'a>(
        &self,
        channel: &'a GuildChannel,
        _theme: &ChannelsTheme,
    ) -> Element<'a, AppEvent> {
        match channel {
            GuildChannel::Text(channel) => text(&channel.base.name).into(),
            GuildChannel::Category(category) => text(&category.base.name).into(),

            _ => text("not implemented yet")
                .color(Color::from_rgb8(255, 0, 0))
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
