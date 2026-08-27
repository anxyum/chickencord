use super::{Message, channel::GuildChannel};
use crate::{Context, app_event::AppEvent};
use discord_client_structs::structs::message::query::MessageQuery;
use iced::{
    Color, Element, Padding,
    border::Radius,
    widget::{column, container, scrollable},
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Channels {
    channels: HashMap<u64, GuildChannel>,
    channels_order: Vec<u64>,
    categories_order: Vec<u64>,
    selected_channel: u64,
}

impl Channels {
    pub fn new(channels: HashMap<u64, GuildChannel>) -> Self {
        let mut channels = Self {
            channels,
            channels_order: Vec::new(),
            categories_order: Vec::new(),
            selected_channel: 0,
        }
        .organize();
        channels.selected_channel = channels
            .channels_order
            .first()
            .copied()
            .or_else(|| {
                channels.categories_order.first().and_then(|id| {
                    channels.channels.get(id).and_then(|c| {
                        if let GuildChannel::Category(c) = c {
                            c.children.first().copied()
                        } else {
                            None
                        }
                    })
                })
            })
            .unwrap_or_default();

        channels
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

    pub fn show_channels<'a>(&'a self, context: &'a Context) -> Element<'a, AppEvent> {
        let channels_theme = &context.theme.channels;

        let mut uncategorized = self
            .channels_order
            .iter()
            .filter_map(|id| Some(self.channels.get(id)?.show(context, self.selected_channel)))
            .peekable();
        let uncategorized_empty = uncategorized.peek().is_none();

        let categorized = self.categories_order.iter().filter_map(|id| {
            let channel = self.channels.get(id)?;
            let GuildChannel::Category(category) = channel else {
                return None;
            };

            let mut col = column([channel.show(context, self.selected_channel)])
                .spacing(channels_theme.spacing);

            if category.is_open {
                let children = category.children.iter().filter_map(|id| {
                    Some(self.channels.get(id)?.show(context, self.selected_channel))
                });

                col = col.extend(children);
            }

            Some(col.into())
        });

        let content = column(uncategorized.chain(categorized))
            .padding(
                Padding::new(channels_theme.padding).top(if uncategorized_empty {
                    0.0
                } else {
                    channels_theme.category_spacing
                }),
            )
            .spacing(channels_theme.spacing);
        scrollable(content)
            .style(|theme, status| {
                let mut style = scrollable::default(theme, status);
                let border_width = (12.0 - channels_theme.scroller_width) / 2.0;

                style.container =
                    container::Style::default().background(context.theme.channels.background);
                style.vertical_rail.background = None;
                style.vertical_rail.scroller.background = channels_theme.scroller_color.into();
                style.vertical_rail.scroller.border.width = border_width;
                style.vertical_rail.scroller.border.color = Color::TRANSPARENT;
                style.vertical_rail.scroller.border.radius =
                    Radius::new(border_width + channels_theme.scroller_width / 2.0);

                style
            })
            .into()
    }

    pub fn toggle_category(&mut self, id: u64) -> Option<()> {
        match self.channels.get_mut(&id)? {
            GuildChannel::Category(category) => category.is_open = !category.is_open,
            _ => return None,
        }

        Some(())
    }

    pub fn channel_hover(&mut self, channel_id: u64, hovered: bool) -> Option<()> {
        self.channels.get_mut(&channel_id)?.set_hovered(hovered);
        Some(())
    }

    pub fn select_channel(&mut self, channel_id: u64) {
        self.selected_channel = channel_id;
    }

    pub fn load_messages(
        &mut self,
        channel_id: u64,
        query: MessageQuery,
        messages: Vec<Message>,
    ) -> Option<()> {
        self.channels
            .get_mut(&channel_id)?
            .load_messages(query, messages);
        Some(())
    }

    pub fn show_body(&self, context: &Context) -> Option<Element<'_, AppEvent>> {
        self.channels
            .get(&self.selected_channel)?
            .show_body(context)
    }
}
