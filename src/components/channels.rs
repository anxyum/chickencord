use super::channel::GuildChannel;
use crate::{
    Context,
    app_event::{AppEvent, AppMessage},
    components::button,
};
use iced::{
    Color, Element, alignment,
    widget::{Svg, column, container, row, text},
};
use std::{collections::HashMap, f32::consts::PI};

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
        context: &Context,
        channel: &'a GuildChannel,
    ) -> Element<'a, AppEvent> {
        match channel {
            GuildChannel::Text(channel) => button(
                text(&channel.base.name)
                    .size(16)
                    .color(Color::from_rgb8(120, 120, 120)),
            )
            .into(),
            GuildChannel::Category(category) => {
                let channel_id = category.base.id;

                button(
                    row([
                        text(&category.base.name)
                            .size(14)
                            .color(Color::from_rgb8(120, 120, 120))
                            .into(),
                        Svg::new(context.icons.unfold_category.clone())
                            .width(12)
                            .height(12)
                            .rotation(if category.is_open { 0.0 } else { -PI * 0.5 })
                            .into(),
                    ])
                    .align_y(alignment::Vertical::Center)
                    .spacing(4),
                )
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
        context: &'a Context,
        panel_width: f32,
    ) -> Element<'a, AppEvent> {
        let uncategorized = self
            .channels_order
            .iter()
            .filter_map(|id| Some(self.show_channel(context, self.channels.get(id)?)));

        let categorized = self.categories_order.iter().filter_map(|id| {
            let channel = self.channels.get(id)?;
            let GuildChannel::Category(category) = channel else {
                return None;
            };

            let mut col = column(std::iter::once(self.show_channel(context, channel)));

            if category.is_open {
                let children = category
                    .children
                    .iter()
                    .filter_map(|id| Some(self.show_channel(context, self.channels.get(id)?)));

                col = col.extend(children);
            }

            Some(col.into())
        });

        let content = container(column(uncategorized.chain(categorized))).width(panel_width);

        container(content)
            .style(|_| container::Style::default().background(context.theme.channels.background))
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
