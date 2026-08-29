use std::sync::LazyLock;

use crate::components::{Member, User};
use anyhow::Result as AnyResult;
use bytes::Bytes;
use discord_client_structs::structs::{
    guild::GatewayGuild,
    user::{Member as GatewayMember, User as GatewayUser},
};
use iced::widget::image::Handle;
use reqwest::Client;

static CLIENT: LazyLock<Client> = LazyLock::new(|| Client::new());

pub async fn fetch_guild_avatar(guild: &GatewayGuild) -> Option<Handle> {
    maybe_fetch_avatar(guild_avatar_url(guild).as_deref())
        .await
        .map(|bytes| Handle::from_bytes(Bytes::from(bytes)))
}

pub async fn fetch_gateway_user_avatar(user: &GatewayUser) -> Option<Handle> {
    maybe_fetch_avatar(gateway_user_avatar_url(user).as_deref())
        .await
        .map(|bytes| Handle::from_bytes(Bytes::from(bytes)))
}

pub async fn fetch_user_avatar(hash: &str, id: u64) -> Option<Handle> {
    fetch_avatar(&user_avatar_url(hash, id))
        .await
        .ok()
        .map(|bytes| Handle::from_bytes(Bytes::from(bytes)))
}

pub async fn fetch_member_avatar(member: &GatewayMember, guild_id: u64) -> Option<Handle> {
    maybe_fetch_avatar(member_avatar_url(member, guild_id).as_deref())
        .await
        .map(|bytes| Handle::from_bytes(Bytes::from(bytes)))
}

pub fn gateway_user_avatar_url(user: &GatewayUser) -> Option<String> {
    let hash = user.avatar.as_ref()?;
    Some(user_avatar_url(hash, user.id))
}

pub fn user_avatar_url(hash: &str, id: u64) -> String {
    format!(
        "https://cdn.discordapp.com/avatars/{}/{hash}.{}?size=64",
        id,
        if hash.starts_with("a_") {
            "gif"
        } else {
            "webp"
        }
    )
}

pub fn member_avatar_url(member: &GatewayMember, guild_id: u64) -> Option<String> {
    let hash = member.avatar.as_ref()?;
    Some(format!(
        "https://cdn.discordapp.com/guilds/{guild_id}/users/{}/avatars/{hash}.{}?size=240",
        member.user.as_ref()?.id,
        if hash.starts_with("a_") {
            "gif"
        } else {
            "webp"
        }
    ))
}

pub fn guild_avatar_url(guild: &GatewayGuild) -> Option<String> {
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

pub async fn fetch_avatar(link: &str) -> AnyResult<Vec<u8>> {
    Ok(CLIENT.get(link).send().await?.bytes().await?.to_vec())
}

pub async fn maybe_fetch_avatar(link: Option<&str>) -> Option<Vec<u8>> {
    match link {
        Some(l) => fetch_avatar(l).await.ok(),
        None => None,
    }
}

pub async fn load_member(member: GatewayMember, guild_id: u64) -> Member {
    let avatar = fetch_member_avatar(&member, guild_id).await;
    Member {
        id: member.user.unwrap().id,
        nick: member.nick,
        avatar,
    }
}

pub async fn load_user(user: GatewayUser) -> User {
    let avatar = fetch_gateway_user_avatar(&user)
        .await
        .unwrap_or(Handle::from_rgba(1, 1, vec![0, 0, 0, 0]));

    User::new(user.id, user.global_name, user.username, avatar)
}
