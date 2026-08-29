use crate::{
    app_event::{AppEvent, NetworkEvent},
    components::Message,
};
use discord_client_gateway::events::structs::message::MessageCreateEvent;
use iced::futures::{SinkExt, channel::mpsc::Sender};

pub async fn handle_message_create(event: MessageCreateEvent, sender: &mut Sender<AppEvent>) {
    sender
        .send(AppEvent::Network(NetworkEvent::MessageCreate {
            channel_id: event.message.channel_id,
            message: Message::try_from(event.message).unwrap(),
        }))
        .await
        .unwrap();
}
