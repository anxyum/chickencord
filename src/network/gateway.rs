use super::handle_message_create::handle_message_create;
use super::handle_ready::handle_ready;
use super::{EventSender, Request};
use discord_client_gateway::{events::Event, gateway::GatewayClient};
use tokio::sync::broadcast::{Receiver, error::RecvError};

pub fn start(mut request_rx: Receiver<Request>, event_sender: EventSender) {
    let token = std::env::var("DISCORD_TOKEN").expect("missing token in .env");

    tokio::spawn(async move {
        let capabilities = 53607934;

        let mut client = match GatewayClient::connect(token, true, capabilities, None).await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("failed to connect gateway client: {e:?}");
                return;
            }
        };

        loop {
            let action = tokio::select! {
                event = client.next_event() => match event {
                    Ok(event) => Action::Event(event),
                    Err(e) => {
                        eprintln!("gateway error: {e:?}");
                        return;
                    }
                },
                request = request_rx.recv() => match request {
                    Ok(Request::SubscribeGuild { guild_id }) => Action::SubscribeGuild { guild_id },
                    Ok(Request::FetchMessages { .. }) => continue,
                    Ok(Request::LoadUser(_)) => continue,
                    Ok(Request::LoadMember(_, _)) => continue,
                    Err(RecvError::Lagged(_)) => continue,
                    Err(RecvError::Closed) => return,
                },
            };

            match action {
                Action::Event(event) => match event {
                    Event::Ready(event) => handle_ready(event, &event_sender).await,
                    Event::MessageCreate(event) => {
                        handle_message_create(event, &event_sender).await
                    }
                    _ => {}
                },
                Action::SubscribeGuild { guild_id } => {
                    if let Err(e) = client.bulk_guild_subscribe(vec![guild_id]).await {
                        eprintln!("failed to subscribe to guild {guild_id}: {e:?}");
                    }
                }
            }
        }
    });
}

enum Action {
    Event(discord_client_gateway::events::Event),
    SubscribeGuild { guild_id: u64 },
}
