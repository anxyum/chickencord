use super::{EventSender, Request};
use crate::app_event::{AppEvent, NetworkEvent};
use discord_client_rest::rest::RestClient;
use std::sync::Arc;
use tokio::sync::broadcast::{Receiver, error::RecvError};

pub fn start(mut request_rx: Receiver<Request>, event_sender: EventSender) {
    let token = std::env::var("DISCORD_TOKEN").expect("missing token in .env");

    tokio::spawn(async move {
        let client = RestClient::connect(token, None, None, None, None)
            .await
            .expect("failed to connect rest client");

        let client = Arc::new(client);

        loop {
            let request = match request_rx.recv().await {
                Ok(request) => request,
                Err(RecvError::Lagged(_)) => continue,
                Err(RecvError::Closed) => return,
            };

            match request {
                Request::FetchMessages {
                    channel_id,
                    guild_id,
                    query,
                } => {
                    let client = client.clone();
                    let event_sender = event_sender.clone();

                    tokio::spawn(async move {
                        match client
                            .message(channel_id)
                            .get_channel_messages(guild_id, query.clone())
                            .await
                        {
                            Ok(messages) => {
                                let _ = event_sender.send(AppEvent::Network(
                                    NetworkEvent::Messages {
                                        channel_id,
                                        guild_id,
                                        query,
                                        messages,
                                    },
                                ));
                            }
                            Err(e) => {
                                eprintln!("failed to fetch messages: {e:?}");
                            }
                        }
                    });
                }
                Request::SubscribeGuild { .. } => {}
            }
        }
    });
}
