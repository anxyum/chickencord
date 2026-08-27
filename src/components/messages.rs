use super::Message;
use crate::{Context, app_event::AppEvent};
use discord_client_structs::structs::message::query::MessageQuery;
use iced::{Element, widget::column};
use std::collections::{HashMap, VecDeque};

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
}

impl Messages {
    pub fn new() -> Self {
        Self {
            messages: HashMap::new(),
            message_order: VecDeque::new(),
            current_chunk: Vec::new(),
        }
    }

    fn first(&self) -> Option<u64> {
        Some(self.message_order.get(0)?.first)
    }

    fn last(&self) -> Option<u64> {
        Some(self.message_order.get(self.message_order.len() - 1)?.last)
    }

    pub fn extend(&mut self, messages: Vec<Message>, _query: Option<MessageQuery>) {
        if messages.is_empty() {
            return;
        }

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

    pub fn show(&self, context: &Context) -> Element<'_, AppEvent> {
        column(
            self.current_chunk
                .iter()
                .filter_map(|id| self.messages.get(id).map(|m| m.show(context)))
                .chain(self.message_order.iter().flat_map(|mc| {
                    mc.messages
                        .iter()
                        .filter_map(|id| self.messages.get(id).map(|m| m.show(context)))
                })),
        )
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
