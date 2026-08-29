mod gateway;
mod handle_message_create;
mod handle_ready;
mod rest;
pub mod user_settings;
pub mod utils;

pub use user_settings::PreloadedUserSettings;

use crate::{
    app_event::AppEvent,
    components::{LazyMember, LazyUser},
};
use discord_client_structs::structs::message::query::MessageQuery;
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast, broadcast::error::RecvError};

pub type EventSender = broadcast::Sender<AppEvent>;
pub type RequestSender = broadcast::Sender<Request>;

#[derive(Debug, Clone)]
pub enum Request {
    SubscribeGuild {
        guild_id: u64,
    },
    FetchMessages {
        channel_id: u64,
        guild_id: Option<u64>,
        query: MessageQuery,
    },
    LoadUser(LazyUser),
    LoadMember(u64, LazyMember),
}

#[derive(Clone)]
pub struct Receiver<T>(pub Arc<Mutex<broadcast::Receiver<T>>>);

impl<T> std::fmt::Debug for Receiver<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Receiver").finish()
    }
}

impl<T> std::hash::Hash for Receiver<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(Arc::as_ptr(&self.0), state);
    }
}

impl<T> PartialEq for Receiver<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Eq for Receiver<T> {}

#[derive(Debug)]
pub struct NetworkChannels {
    pub request_sender: RequestSender,
    pub event_receiver: Receiver<AppEvent>,
}

pub fn start() -> NetworkChannels {
    let (event_sender, event_receiver) = broadcast::channel(100);
    let (request_sender, gateway_rx) = broadcast::channel(100);
    let rest_rx = request_sender.subscribe();

    gateway::start(gateway_rx, event_sender.clone());
    rest::start(rest_rx, event_sender);

    NetworkChannels {
        request_sender,
        event_receiver: Receiver(Arc::new(Mutex::new(event_receiver))),
    }
}

pub fn worker(receiver: Receiver<AppEvent>) -> impl iced::futures::Stream<Item = AppEvent> {
    iced::stream::channel(100, async move |mut output| {
        use iced::futures::SinkExt;

        loop {
            let mut rx = receiver.0.lock().await;
            match rx.recv().await {
                Ok(event) => {
                    drop(rx);
                    let _ = output.send(event).await;
                }
                Err(RecvError::Lagged(n)) => {
                    eprintln!("network worker lagged, skipped {n} events");
                    drop(rx);
                }
                Err(RecvError::Closed) => break,
            }
        }
    })
}
