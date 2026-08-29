use crate::{
    app_event::{AppEvent, NetworkEvent},
    components::{LazyMember, LazyUser, Message},
};
use discord_client_gateway::events::structs::message::MessageCreateEvent;
use discord_client_structs::structs::message::Message as GatewayMessage;
use tokio::sync::broadcast::Sender;

pub async fn handle_message_create(event: MessageCreateEvent, sender: &Sender<AppEvent>) {
    let MessageCreateEvent {
        message,
        member,
        guild_id,
        ..
    } = event;

    let GatewayMessage {
        author,
        content,
        id,
        channel_id,
        ..
    } = message;

    let author_member = match (member, guild_id) {
        (Some(m), Some(guild_id)) => Some((
            guild_id,
            LazyMember {
                id: author.id,
                nick: m.nick,
                avatar: m.avatar,
            },
        )),
        _ => None,
    };

    let message = Message {
        id,
        content,
        author_id: author.id,
    };

    let author_user = LazyUser {
        id: author.id,
        display_name: author.global_name,
        username: author.username,
        avatar: author.avatar.unwrap_or_default(),
    };

    let _ = sender.send(AppEvent::Network(NetworkEvent::MessageCreate {
        channel_id,
        message,
        author_member,
        author_user,
    }));
}
