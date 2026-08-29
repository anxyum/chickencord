use super::PreloadedUserSettings;
use crate::{
    app_event::{AppEvent, NetworkEvent},
    components::{Channel, Guild, Member, User},
};
use anyhow::Result as AnyResult;
use base64::{Engine, engine::general_purpose::STANDARD};
use bytes::Bytes;
use discord_client_gateway::events::structs::ready::ReadyEvent;
use discord_client_structs::structs::{guild::GatewayGuild, user::User as GatewayUser};
use futures::{StreamExt, stream};
use iced::widget::image::Handle;
use prost::Message;
use std::collections::HashMap;
use tokio::sync::broadcast::Sender;

pub async fn handle_ready(event: ReadyEvent, sender: &Sender<AppEvent>) {
    let client = reqwest::Client::new();

    let mut guilds = Vec::new();
    let mut members = HashMap::new();
    let mut channels = HashMap::new();

    for guild in event.guilds {
        let guild_id = guild.id;

        let name = guild
            .name
            .clone()
            .or_else(|| guild.properties.as_ref()?.name.clone())
            .unwrap_or_default();

        let avatar = fetch_guild_avatar(&client, &guild).await;

        guilds.push(Guild::new(guild_id, name, avatar));

        let guild_members = guild
            .members
            .unwrap_or_default()
            .into_iter()
            .map(|member| Member {
                id: member.user.unwrap().id,
            })
            .collect();

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
        .map(|user| async {
            let avatar = fetch_user_avatar(&client, &user)
                .await
                .unwrap_or(Handle::from_rgba(1, 1, vec![0, 0, 0, 0]));

            User::new(user.id, user.global_name, user.username, avatar)
        })
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

async fn fetch_guild_avatar(client: &reqwest::Client, guild: &GatewayGuild) -> Option<Handle> {
    match guild_avatar_url(guild) {
        Some(link) => fetch_avatar(client, &link).await.ok(),
        None => None,
    }
    .map(|bytes| Handle::from_bytes(Bytes::from(bytes)))
}

async fn fetch_user_avatar(client: &reqwest::Client, user: &GatewayUser) -> Option<Handle> {
    match user_avatar_url(user) {
        Some(link) => fetch_avatar(client, &link).await.ok(),
        None => None,
    }
    .map(|bytes| Handle::from_bytes(Bytes::from(bytes)))
}

fn user_avatar_url(user: &GatewayUser) -> Option<String> {
    let hash = user.avatar.as_ref()?;
    Some(format!(
        "https://cdn.discordapp.com/avatars/{}/{hash}.{}?size=64",
        user.id,
        if hash.starts_with("a_") {
            "gif"
        } else {
            "webp"
        }
    ))
}

fn guild_avatar_url(guild: &GatewayGuild) -> Option<String> {
    let properties = guild.properties.as_ref()?;
    let icon_hash = properties.icon.as_ref()?;
    Some(format!(
        "https://cdn.discordapp.com/icons/{}/{}.{}",
        guild.id,
        icon_hash,
        if icon_hash.starts_with("a_") {
            "gif"
        } else {
            "png"
        }
    ))
}

async fn fetch_avatar(client: &reqwest::Client, link: &str) -> AnyResult<Vec<u8>> {
    Ok(client.get(link).send().await?.bytes().await?.to_vec())
}

fn decode_settings(
    user_settings_proto: &str,
) -> Result<PreloadedUserSettings, Box<dyn std::error::Error>> {
    let bytes = STANDARD.decode(user_settings_proto)?;

    let settings = PreloadedUserSettings::decode(bytes.as_slice())?;

    Ok(settings)
}
