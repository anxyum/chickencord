use super::{
    PreloadedUserSettings,
    utils::{fetch_guild_avatar, load_member, load_user},
};
use crate::{
    app_event::{AppEvent, NetworkEvent},
    components::{Channel, Guild},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use discord_client_gateway::events::structs::ready::ReadyEvent;
use discord_client_structs::structs::guild::GatewayGuild;
use futures::{StreamExt, stream};
use prost::Message;
use std::collections::HashMap;
use tokio::sync::broadcast::Sender;

pub async fn handle_ready(event: ReadyEvent, sender: &Sender<AppEvent>) {
    let mut guilds = Vec::new();
    let mut members = HashMap::new();
    let mut channels = HashMap::new();

    for guild in event.guilds.into_iter() {
        let avatar = fetch_guild_avatar(&guild).await;

        let GatewayGuild { id: guild_id, .. } = guild;
        let name = guild
            .name
            .clone()
            .or_else(|| guild.properties.as_ref()?.name.clone())
            .unwrap_or_default();

        guilds.push(Guild::new(guild_id, name, avatar));

        let guild_members = stream::iter(guild.members.unwrap_or_default())
            .map(|member| load_member(member, guild_id))
            .buffer_unordered(10)
            .collect()
            .await;

        members.insert(guild_id, guild_members);

        let guild_channels = guild
            .channels
            .unwrap_or_default()
            .into_iter()
            .filter_map(|channel| {
                Channel::try_from(channel)
                    .ok()
                    .and_then(|channel| match channel {
                        Channel::Guild(channel) => Some(channel),
                        _ => None,
                    })
            })
            .collect();

        channels.insert(guild_id, guild_channels);
    }

    let users = stream::iter(event.users.unwrap_or_default())
        .map(|user| load_user(user))
        .buffer_unordered(10)
        .collect()
        .await;

    let user_settings = event
        .user_settings_proto
        .and_then(|proto| decode_settings(&proto).ok())
        .unwrap_or_default();

    let _ = sender.send(AppEvent::Network(NetworkEvent::Ready {
        guilds,
        users,
        members,
        channels,
        user_settings,
    }));
}

fn decode_settings(
    user_settings_proto: &str,
) -> Result<PreloadedUserSettings, Box<dyn std::error::Error>> {
    let bytes = STANDARD.decode(user_settings_proto)?;

    let settings = PreloadedUserSettings::decode(bytes.as_slice())?;

    Ok(settings)
}
