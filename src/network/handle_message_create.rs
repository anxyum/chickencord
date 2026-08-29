use crate::{
    app_event::{AppEvent, NetworkEvent},
    components::Message,
};
use discord_client_gateway::events::structs::message::MessageCreateEvent;
use tokio::sync::broadcast::Sender;

pub async fn handle_message_create(event: MessageCreateEvent, sender: &Sender<AppEvent>) {
    let _ = sender.send(AppEvent::Network(NetworkEvent::MessageCreate {
        channel_id: event.message.channel_id,
        message: Message::try_from(event.message).unwrap(),
    }));
}
