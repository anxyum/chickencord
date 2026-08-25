use super::{GatewayChannel, GuildChannelBase, Unknown, guild_base};

#[derive(Debug)]
pub struct Category {
    pub base: GuildChannelBase,
    pub children: Vec<u64>,
    pub is_open: bool,
}

impl TryFrom<GatewayChannel> for Category {
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
            children: Vec::new(),
            is_open: true,
        })
    }
}
