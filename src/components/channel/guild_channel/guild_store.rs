use super::{GatewayChannel, GuildChannelBase, Unknown, guild_base};

#[derive(Debug, Clone)]
pub struct GuildStore {
    pub base: GuildChannelBase,
}

impl TryFrom<GatewayChannel> for GuildStore {
    type Error = Unknown;

    fn try_from(value: GatewayChannel) -> Result<Self, Self::Error> {
        let flags = value.flags.unwrap_or_default() as u32;

        Ok(Self {
            base: guild_base(
                value.id,
                value.guild_id,
                value.name,
                value.position,
                flags,
                value.parent_id,
            ),
        })
    }
}
