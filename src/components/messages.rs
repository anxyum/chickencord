use super::Message;
use crate::{
    Context,
    app_event::{AppEvent, AppMessage},
    components::Cache,
};
use discord_client_structs::structs::message::query::MessageQuery;
use iced::{
    Border, Color, Element, Length, Padding, Theme,
    border::Radius,
    widget::{Id, Space, column, container, mouse_area, row, scrollable, text},
};
use rand::{SeedableRng, rngs::StdRng, seq::SliceRandom};
use std::collections::{HashMap, VecDeque};

const PLACEHOLDERS: &[&[f32]] = &[
    &[120.0, 250.0, 200.0, 230.0],
    &[130.0, 230.0, 280.0],
    &[140.0, 240.0, 280.0],
    &[150.0, 220.0, 240.0, 280.0],
    &[160.0, 220.0, 250.0, 300.0],
    &[170.0, 240.0, 280.0],
    &[180.0, 300.0, 220.0, 180.0],
    &[190.0, 260.0, 180.0],
    &[200.0, 270.0, 180.0],
];

const PLACEHOLDER_MESSAGE_COUNT: f32 = PLACEHOLDERS.len() as f32;

const PLACEHOLDER_TEXT_LINE_COUNT: f32 = {
    let mut count = 0;
    let mut i = 0;

    while i < PLACEHOLDERS.len() {
        count += PLACEHOLDERS[i].len() - 1;
        i += 1;
    }

    count as f32
};

fn place_holder_total_size(context: &Context) -> f32 {
    let msgs = &context.theme.messages;
    let msg = &msgs.message;

    let message_heights = (msg.padding_y * 2.0 + msg.username_size) * PLACEHOLDER_MESSAGE_COUNT
        + (msg.text_size + msg.padding_y * 2.0) * PLACEHOLDER_TEXT_LINE_COUNT;

    message_heights + msgs.message_spacing * (PLACEHOLDER_MESSAGE_COUNT - 1.0)
}

#[derive(Debug)]
pub struct MessagesChunk {
    messages: Vec<u64>,
    first: u64,
    last: u64,
}

impl FromIterator<u64> for MessagesChunk {
    fn from_iter<T: IntoIterator<Item = u64>>(iter: T) -> Self {
        let mut messages: Vec<u64> = iter.into_iter().collect();
        messages.sort_unstable();
        let first = *messages.first().unwrap();
        let last = *messages.last().unwrap();

        Self {
            messages,
            first,
            last,
        }
    }
}

impl MessagesChunk {
    pub fn merge(&mut self, other: Self) {
        if self.last < other.first {
            self.messages.extend(other.messages);
            self.last = other.last;
        } else if other.last < self.first {
            let messages = std::mem::replace(&mut self.messages, other.messages);
            self.messages.extend(messages);
            self.first = other.first;
        } else {
            let self_messages = std::mem::replace(&mut self.messages, Vec::new());
            let other_messages = other.messages;

            sort_merge_into(&mut self.messages, self_messages, other_messages);
            self.first = *self.messages.first().unwrap();
            self.last = *self.messages.last().unwrap();
        }
    }
}

#[derive(Debug)]
pub struct Messages {
    messages: HashMap<u64, Message>,
    message_order: VecDeque<MessagesChunk>,
    current_chunk: Vec<u64>,
    loaded: bool,
    exhausted: bool,
    anchor_bottom: bool,
    restoring: bool,
    restore_offset: f32,
    restore_height: f32,
}

impl Messages {
    pub fn new() -> Self {
        Self {
            messages: HashMap::new(),
            message_order: VecDeque::new(),
            current_chunk: Vec::new(),
            loaded: false,
            exhausted: false,
            anchor_bottom: true,
            restoring: false,
            restore_offset: 0.0,
            restore_height: 0.0,
        }
    }

    pub fn set_anchor_bottom(&mut self, anchor_bottom: bool) -> bool {
        if self.anchor_bottom == anchor_bottom {
            return false;
        }
        self.anchor_bottom = anchor_bottom;
        true
    }

    pub fn start_restore(&mut self, anchor_bottom: bool, offset: f32, height: f32) {
        self.restoring = !anchor_bottom;
        if !anchor_bottom {
            self.restore_offset = offset;
            self.restore_height = height;
        }
    }

    pub fn clear_restore(&mut self) {
        self.restoring = false;
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    fn first(&self) -> Option<u64> {
        Some(self.message_order.get(0)?.first)
    }

    fn last(&self) -> Option<u64> {
        Some(self.message_order.get(self.message_order.len() - 1)?.last)
    }

    pub fn extend(&mut self, messages: Vec<Message>, query: Option<MessageQuery>) {
        if let Some(query) = &query
            && query.before.is_some()
            && (messages.len() as u8) < query.limit
        {
            self.exhausted = true;
            self.restoring = false;
        }

        if messages.is_empty() {
            return;
        }

        self.loaded = true;

        let messages_chunk = messages.iter().map(|m| m.id).collect();
        self.messages
            .extend(messages.into_iter().map(|m| (m.id, m)));

        self.integrate(messages_chunk);
    }

    fn integrate(&mut self, chunk: MessagesChunk) {
        if self.message_order.is_empty() {
            self.message_order.push_back(chunk);
            return;
        }

        let first = self.first().unwrap();
        let last = self.last().unwrap();

        if chunk.last < first {
            self.message_order.get_mut(0).unwrap().merge(chunk);
        } else if last < chunk.first {
            let i = self.message_order.len() - 1;
            self.message_order.get_mut(i).unwrap().merge(chunk);
        } else {
            self.merge_overlapping(chunk);
        }
    }

    fn merge_overlapping(&mut self, chunk: MessagesChunk) {
        let mut ids: Vec<u64> = Vec::new();

        for order in &self.message_order {
            ids.extend_from_slice(&order.messages);
        }

        ids.extend(chunk.messages);
        ids.sort_unstable();
        ids.dedup();

        self.message_order.clear();

        for batch in ids.chunks(64) {
            self.message_order
                .push_back(batch.iter().copied().collect());
        }
    }

    pub fn new_message(&mut self, message: Message) {
        let id = message.id;

        if let Some(&last) = self.current_chunk.last() {
            if id == last {
                self.messages.insert(id, message);
                return;
            }
            if id < last {
                self.flush_current_chunk();
            }
        }

        self.current_chunk.push(id);
        self.messages.insert(id, message);
    }

    fn flush_current_chunk(&mut self) {
        if self.current_chunk.is_empty() {
            return;
        }

        let chunk: MessagesChunk = std::mem::take(&mut self.current_chunk)
            .into_iter()
            .collect();

        self.integrate(chunk);
    }

    pub fn load_messages(&mut self, query: MessageQuery, messages: Vec<Message>) {
        self.extend(messages, Some(query));
    }

    pub fn show<'a>(
        &'a self,
        context: &'a Context,
        cache: &'a Cache,
        guild_id: u64,
        channel_id: u64,
        hovered_message: Option<u64>,
    ) -> Element<'a, AppEvent> {
        let mut col = Vec::new();
        col.push(Space::new().height(Length::Fill).into());

        if !self.exhausted {
            col.push(Self::loading_placeholder(channel_id, context));
        }

        let mut group: Vec<Element<'a, AppEvent>> = Vec::new();

        let flush_group = |group: &mut Vec<Element<'a, AppEvent>>,
                           messages_el: &mut Vec<Element<'a, AppEvent>>| {
            if !group.is_empty() {
                messages_el.push(column(group.drain(..)).into());
            }
        };

        let ids = self
            .message_order
            .iter()
            .flat_map(|chunk| chunk.messages.iter())
            .chain(self.current_chunk.iter());

        let mut previous_author = 0;
        for id in ids {
            let Some(m) = self.messages.get(&id) else {
                continue;
            };
            let new_group = previous_author != m.author_id;
            previous_author = m.author_id;

            if new_group && !group.is_empty() {
                flush_group(&mut group, &mut col);
            }
            let hovered = hovered_message == Some(m.id);
            let message = if new_group {
                m.show(context, cache, guild_id, hovered)
            } else {
                m.show_reduced(context, hovered)
            };
            group.push(
                mouse_area(message)
                    .on_enter(AppEvent::Message(AppMessage::MessageHover(m.id, true)))
                    .on_exit(AppEvent::Message(AppMessage::MessageHover(m.id, false)))
                    .into(),
            );
        }
        flush_group(&mut group, &mut col);

        let anchor_bottom = self.anchor_bottom;

        scrollable(column(col).spacing(context.theme.messages.message_spacing))
            .style(|theme, status| {
                let mut style = scrollable::default(theme, status);
                let border_width = (12.0 - context.theme.messages.scroller_width) / 2.0;

                style.vertical_rail.background = None;
                style.vertical_rail.scroller.background =
                    context.theme.messages.scroller_color.into();
                style.vertical_rail.scroller.border.width = border_width;
                style.vertical_rail.scroller.border.radius =
                    Radius::new(border_width + context.theme.messages.scroller_width / 2.0);

                style
            })
            .id(Id::new("chickencord-messages-scroll"))
            .anchor_y(if anchor_bottom {
                scrollable::Anchor::End
            } else {
                scrollable::Anchor::Start
            })
            .on_scroll(move |viewport| {
                let abs = viewport.absolute_offset().y;
                let rev = viewport.absolute_offset_reversed().y;
                let distance_to_top = if anchor_bottom { rev } else { abs };
                let distance_to_bottom = if anchor_bottom { abs } else { rev };
                let height = viewport.content_bounds().height;

                if self.restoring && height - self.restore_height > 1.0 {
                    AppEvent::Message(AppMessage::RestoreScroll {
                        channel_id,
                        offset: self.restore_offset + (height - self.restore_height),
                    })
                } else if distance_to_top < place_holder_total_size(context)
                    && !self.exhausted
                    && !self.restoring
                {
                    AppEvent::Message(AppMessage::LoadBefore {
                        guild_id,
                        channel_id,
                        before: self.first().unwrap_or(u64::MAX),
                        anchor_bottom,
                        offset: abs,
                        height,
                    })
                } else {
                    AppEvent::Message(AppMessage::Scroll {
                        channel_id,
                        anchor_bottom: distance_to_bottom < 2.0,
                        offset: rev,
                    })
                }
            })
            .into()
    }

    pub fn loading_placeholder<'a>(channel_id: u64, context: &'a Context) -> Element<'a, AppEvent> {
        let messages_theme = &context.theme.messages;
        let message_theme = &messages_theme.message;

        let wireline = |width: f32, height: f32, fill: Color| -> Element<'a, AppEvent> {
            container(text(""))
                .width(width)
                .height(height)
                .style(move |_theme: &Theme| container::Style {
                    background: Some(fill.into()),
                    border: Border {
                        radius: Radius::new(height / 2.0),
                        ..Border::default()
                    },
                    ..container::Style::default()
                })
                .into()
        };

        let wireavatar = || -> Element<'a, AppEvent> {
            container(text(""))
                .width(message_theme.avatar_size)
                .height(message_theme.avatar_size)
                .style(move |_theme: &Theme| container::Style {
                    background: Some(message_theme.placeholder_avatar_color.into()),
                    border: Border {
                        radius: Radius::new(message_theme.avatar_size / 2.0),
                        ..Border::default()
                    },
                    ..container::Style::default()
                })
                .into()
        };

        let name_size = message_theme.username_size;
        let text_size = message_theme.text_size;
        let wiremessage = |widths: &[f32]| -> Element<'a, AppEvent> {
            let mut widths = widths.into_iter();
            let mut children = vec![wireline(
                *widths.next().unwrap(),
                name_size,
                message_theme.placeholder_name_color,
            )];

            for &w in widths {
                children.push(wireline(w, text_size, message_theme.placeholder_text_color));
            }
            row![
                wireavatar(),
                column(children).spacing(message_theme.padding_y * 2.0),
            ]
            .spacing(message_theme.avatar_spacing)
            .padding(Padding::new(message_theme.padding_y).left(message_theme.avatar_padding_left))
            .into()
        };

        let mut children: Vec<_> = PLACEHOLDERS.iter().map(|w| wiremessage(w)).collect();
        children.shuffle(&mut StdRng::seed_from_u64(channel_id));
        column(children)
            .spacing(messages_theme.message_spacing)
            .into()
    }
}

fn sort_merge_into<T: Ord>(into: &mut Vec<T>, a: Vec<T>, b: Vec<T>) {
    into.clear();
    into.reserve(a.len() + b.len());

    let mut a = a.into_iter().peekable();
    let mut b = b.into_iter().peekable();

    while let (Some(x), Some(y)) = (a.peek(), b.peek()) {
        let next = if x <= y {
            a.next().unwrap()
        } else {
            b.next().unwrap()
        };
        if into.last() != Some(&next) {
            into.push(next);
        }
    }

    for next in a.chain(b) {
        if into.last() != Some(&next) {
            into.push(next);
        }
    }
}
