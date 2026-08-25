use std::num::NonZeroU64;

use discord_client_structs::structs::channel::Channel as GatewayChannel;

use crate::components::channel::{Unknown, nz};

#[derive(Debug)]
pub struct Lobby {
    pub id: u64,
    pub guild_id: Option<u64>,
    pub name: Option<String>,
    pub position: Option<i64>,
    pub flags: u32,
    pub parent_id: Option<NonZeroU64>,
    pub last_message_id: Option<u64>,
}

impl TryFrom<GatewayChannel> for Lobby {
    type Error = Unknown;

    fn try_from(value: GatewayChannel) -> Result<Self, Self::Error> {
        let flags = value.flags.unwrap_or_default() as u32;

        Ok(Self {
            id: value.id,
            guild_id: value.guild_id,
            name: value.name,
            position: value.position,
            flags,
            parent_id: nz(value.parent_id),
            last_message_id: value.last_message_id,
        })
    }
}
