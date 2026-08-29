#![allow(dead_code)]

mod direct_channel;
mod guild_channel;
mod lobby;

use direct_channel::DirectChannel;
use discord_client_structs::structs::channel::Channel as GatewayChannel;
pub use guild_channel::GuildChannel;
use lobby::Lobby;
use std::num::NonZeroU64;

#[derive(Debug, Clone)]
pub enum Channel {
    Guild(GuildChannel),
    Direct(DirectChannel),
    Lobby(Lobby),
}

impl Channel {
    pub fn id(&self) -> u64 {
        match self {
            Self::Guild(channel) => channel.base().id,
            Self::Direct(DirectChannel::Dm(channel)) => channel.id,
            Self::Direct(DirectChannel::Group(channel)) => channel.id,
            Self::Direct(DirectChannel::Ephemeral(channel)) => channel.id,
            Self::Lobby(channel) => channel.id,
        }
    }

    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Guild(channel) => Some(&channel.base().name),
            Self::Direct(_) => None,
            Self::Lobby(channel) => channel.name.as_deref(),
        }
    }
}

pub(crate) fn nz(id: Option<u64>) -> Option<NonZeroU64> {
    id.and_then(NonZeroU64::new)
}

pub struct Unknown;

impl TryFrom<GatewayChannel> for Channel {
    type Error = Unknown;
    fn try_from(value: GatewayChannel) -> Result<Self, Unknown> {
        match value.r#type {
            0 | 2 | 4 | 5 | 6 | 10 | 11 | 12 | 13 | 14 | 15 | 16 => {
                Ok(Self::Guild(GuildChannel::try_from(value)?))
            }
            1 | 3 | 18 => Ok(Self::Direct(DirectChannel::try_from(value)?)),
            17 => Ok(Self::Lobby(Lobby::try_from(value)?)),
            _ => Err(Unknown),
        }
    }
}

pub(crate) fn recipients(
    recipients: Option<Vec<discord_client_structs::structs::user::User>>,
) -> Vec<u64> {
    recipients
        .unwrap_or_default()
        .into_iter()
        .map(|user| user.id)
        .collect()
}
