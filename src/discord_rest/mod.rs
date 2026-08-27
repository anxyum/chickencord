use crate::app_event::AppEvent;
use discord_client_rest::rest::RestClient;
use discord_client_structs::structs::message::{Message, query::MessageQuery};
use iced::futures::SinkExt;
use iced::{futures::Stream, stream};
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast, broadcast::error::RecvError};

#[derive(Debug, Clone)]
pub enum RestRequest {
    FetchMessages {
        channel_id: u64,
        guild_id: Option<u64>,
        query: MessageQuery,
    },
}

#[derive(Debug, Clone)]
pub enum RestResponse {
    Messages {
        channel_id: u64,
        guild_id: Option<u64>,
        query: MessageQuery,
        messages: Vec<Message>,
    },
}

#[derive(Clone)]
pub struct ResponseReceiverHandle(pub Arc<Mutex<broadcast::Receiver<RestResponse>>>);

impl std::fmt::Debug for ResponseReceiverHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResponseReceiverHandle").finish()
    }
}

impl std::hash::Hash for ResponseReceiverHandle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(Arc::as_ptr(&self.0), state);
    }
}

impl PartialEq for ResponseReceiverHandle {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for ResponseReceiverHandle {}

#[derive(Debug)]
pub struct RestChannels {
    pub request_sender: tokio::sync::mpsc::Sender<RestRequest>,
    pub response_receiver: ResponseReceiverHandle,
}

pub fn start() -> RestChannels {
    let (request_tx, mut request_rx) = tokio::sync::mpsc::channel(100);
    let (response_tx, response_rx) = broadcast::channel(100);

    let response_receiver = ResponseReceiverHandle(Arc::new(Mutex::new(response_rx)));

    let token = std::env::var("DISCORD_TOKEN").expect("missing token in .env");

    tokio::spawn(async move {
        let client = RestClient::connect(token, None, None, None, None)
            .await
            .expect("failed to connect rest client");

        let client = Arc::new(client);

        while let Some(request) = request_rx.recv().await {
            match request {
                RestRequest::FetchMessages {
                    channel_id,
                    guild_id,
                    query,
                } => {
                    let client = client.clone();
                    let response_tx = response_tx.clone();

                    tokio::spawn(async move {
                        match client
                            .message(channel_id)
                            .get_channel_messages(guild_id, query.clone())
                            .await
                        {
                            Ok(messages) => {
                                let _ = response_tx.send(RestResponse::Messages {
                                    channel_id,
                                    guild_id,
                                    query,
                                    messages,
                                });
                            }
                            Err(e) => {
                                eprintln!("failed to fetch messages: {e:?}");
                            }
                        }
                    });
                }
            }
        }
    });

    RestChannels {
        request_sender: request_tx,
        response_receiver,
    }
}

pub fn worker(
    receiver: Arc<Mutex<broadcast::Receiver<RestResponse>>>,
) -> impl Stream<Item = AppEvent> {
    stream::channel(100, async move |mut output| {
        loop {
            let mut rx = receiver.lock().await;
            match rx.recv().await {
                Ok(response) => {
                    drop(rx);
                    let _ = output.send(AppEvent::Rest(response)).await;
                }
                Err(RecvError::Lagged(n)) => {
                    eprintln!("rest worker lagged, skipped {n} messages");
                    drop(rx);
                }
                Err(RecvError::Closed) => {
                    break;
                }
            }
        }
    })
}
