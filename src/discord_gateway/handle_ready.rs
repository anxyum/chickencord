use super::PreloadedUserSettings;
use crate::app_event::{AppEvent, NetworkEvent};
use base64::{Engine, engine::general_purpose::STANDARD};
use discord_client_gateway::events::structs::ready::ReadyEvent;
use discord_client_structs::structs::guild::GatewayGuild;
use iced::futures::{SinkExt, channel::mpsc::Sender};
use prost::Message;

pub async fn handle_ready(event: ReadyEvent, sender: &mut Sender<AppEvent>) {
    let client = reqwest::Client::new();

    for guild in event.guilds {
        let client = client.clone();
        let mut sender = sender.clone();

        tokio::spawn(async move {
            let name = guild
                .name
                .clone()
                .or_else(|| guild.properties.as_ref()?.name.clone())
                .unwrap_or_default();

            let avatar_link = guild_avatar_url(&guild);

            let avatar = match avatar_link {
                Some(link) => Some(fetch_avatar(client, &link).await),
                None => None,
            };

            sender
                .send(AppEvent::Network(NetworkEvent::CreateGuild {
                    id: guild.id,
                    name,
                    avatar,
                    channels: guild.channels.unwrap_or_default(),
                }))
                .await
                .unwrap();
        });
    }

    if let Some(user_settings_proto) = &event.user_settings_proto {
        let user_settings = decode_settings(user_settings_proto).unwrap_or_default();
        sender
            .send(AppEvent::Network(NetworkEvent::UserSettings(user_settings)))
            .await
            .unwrap();
    }
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

async fn fetch_avatar(client: reqwest::Client, link: &str) -> Vec<u8> {
    client
        .get(link)
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap()
        .to_vec()
}

fn decode_settings(
    user_settings_proto: &str,
) -> Result<PreloadedUserSettings, Box<dyn std::error::Error>> {
    let bytes = STANDARD.decode(user_settings_proto)?;

    let settings = PreloadedUserSettings::decode(bytes.as_slice())?;

    Ok(settings)
}
