use super::channel::GuildChannel;
use crate::{
    app_event::{AppEvent, AppMessage},
    components::button,
    themes::{AppTheme, ChannelsTheme},
};
use iced::{
    Color, Element,
    widget::{column, container, text},
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Channels {
    channels: HashMap<u64, GuildChannel>,
    channels_order: Vec<u64>,
    categories_order: Vec<u64>,
}

impl Channels {
    pub fn new(channels: HashMap<u64, GuildChannel>) -> Self {
        Self {
            channels,
            channels_order: Vec::new(),
            categories_order: Vec::new(),
        }
        .organize()
    }

    fn organize(mut self) -> Self {
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

    fn show_channel<'a>(
        &self,
        channel: &'a GuildChannel,
        _theme: &ChannelsTheme,
    ) -> Element<'a, AppEvent> {
        match channel {
            GuildChannel::Text(channel) => text(&channel.base.name).into(),
            GuildChannel::Category(category) => {
                let channel_id = category.base.id;

                button(text(&category.base.name))
                    .on_press(AppEvent::Message(AppMessage::ToggleCategory(channel_id)))
                    .into()
            }

            _ => text("not implemented yet")
                .color(Color::from_rgb8(255, 0, 0))
                .into(),
        }
    }

    pub fn show_channels<'a>(
        &'a self,
        theme: &'a AppTheme,
        panel_width: f32,
    ) -> Element<'a, AppEvent> {
        let uncategorized = self
            .channels_order
            .iter()
            .filter_map(|id| Some(self.show_channel(self.channels.get(id)?, &theme.channels)));

        let categorized = self.categories_order.iter().filter_map(|id| {
            let channel = self.channels.get(id)?;
            let GuildChannel::Category(category) = channel else {
                return None;
            };

            let mut col = column(std::iter::once(self.show_channel(channel, &theme.channels)));

            if category.is_open {
                let children = category.children.iter().filter_map(|id| {
                    Some(self.show_channel(self.channels.get(id)?, &theme.channels))
                });

                col = col.extend(children);
            }

            Some(col.into())
        });

        let content = container(column(uncategorized.chain(categorized))).width(panel_width);

        container(content)
            .style(|_| container::Style::default().background(theme.channels.background))
            .into()
    }

    pub fn toggle_category(&mut self, id: u64) -> Option<()> {
        match self.channels.get_mut(&id)? {
            GuildChannel::Category(category) => category.is_open = !category.is_open,
            _ => return None,
        }

        Some(())
    }
}
