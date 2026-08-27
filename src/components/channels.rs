use super::{Cache, Messages, channel::GuildChannel};
use crate::{Context, app_event::AppEvent};
use iced::{
    Color, Element, Padding,
    border::Radius,
    widget::{column, container, scrollable},
};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Channels {
    channels_order: Vec<u64>,
    categories_order: Vec<u64>,
    children: HashMap<u64, Vec<u64>>,
    open_categories: HashSet<u64>,
    selected_channel: u64,
}

impl Channels {
    pub fn new(
        guild_id: u64,
        channels: impl IntoIterator<Item = GuildChannel>,
        cache: &mut Cache,
    ) -> Self {
        let mut channels_order = Vec::new();
        let mut categories_order = Vec::new();
        let mut children: HashMap<u64, Vec<u64>> = HashMap::new();
        let mut positions: HashMap<u64, i64> = HashMap::new();

        for mut channel in channels {
            channel.base_mut().guild_id = guild_id;

            let parent = channel.base().parent_id;
            let is_category = matches!(&channel, GuildChannel::Category(_));
            let is_text = matches!(&channel, GuildChannel::Text(_));
            let id = channel.base().id;
            let position = channel.base().position;

            if is_text {
                cache.messages.insert(id, Messages::new());
            }

            positions.insert(id, position);
            cache.channels.insert(id, channel);

            match parent {
                Some(parent) => children.entry(parent.get()).or_default().push(id),
                None if is_category => categories_order.push(id),
                None => channels_order.push(id),
            }
        }

        let by_position = |id: &u64| positions.get(id).copied();

        channels_order.sort_by_key(by_position);
        categories_order.sort_by_key(by_position);

        for siblings in children.values_mut() {
            siblings.sort_by_key(by_position);
        }

        let selected_channel = channels_order
            .first()
            .copied()
            .or_else(|| {
                categories_order.first().and_then(|id| {
                    children
                        .get(id)
                        .and_then(|siblings| siblings.first())
                        .copied()
                })
            })
            .unwrap_or_default();

        let open_categories = categories_order.iter().copied().collect();

        Self {
            channels_order,
            categories_order,
            children,
            open_categories,
            selected_channel,
        }
    }

    pub fn selected_channel(&self) -> u64 {
        self.selected_channel
    }

    pub fn show_channels<'a>(
        &'a self,
        cache: &'a Cache,
        context: &'a Context,
    ) -> Element<'a, AppEvent> {
        let channels_theme = &context.theme.channels;

        let mut uncategorized = self
            .channels_order
            .iter()
            .filter_map(|id| {
                Some(
                    cache
                        .channels
                        .get(id)?
                        .show(context, self.selected_channel, false),
                )
            })
            .peekable();
        let uncategorized_empty = uncategorized.peek().is_none();

        let categorized = self.categories_order.iter().filter_map(|id| {
            let channel = cache.channels.get(id)?;
            let GuildChannel::Category(_) = channel else {
                return None;
            };

            let is_open = self.open_categories.contains(id);

            let mut col = column([channel.show(context, self.selected_channel, is_open)])
                .spacing(channels_theme.spacing);

            if is_open {
                let children =
                    self.children
                        .get(id)
                        .into_iter()
                        .flatten()
                        .filter_map(|id| {
                            Some(cache.channels.get(id)?.show(
                                context,
                                self.selected_channel,
                                false,
                            ))
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
        if self.open_categories.contains(&id) {
            self.open_categories.remove(&id);
        } else {
            self.open_categories.insert(id);
        }

        Some(())
    }

    pub fn select_channel(&mut self, channel_id: u64) {
        self.selected_channel = channel_id;
    }
}
