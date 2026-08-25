use super::{GatewayChannel, GuildChannelBase, Unknown, guild_base};

#[derive(Debug)]
pub struct StageChannel {
    pub base: GuildChannelBase,
    pub topic: Option<String>,
    pub last_message_id: Option<u64>,
    pub bitrate: u32,
    pub user_limit: u32,
    pub rtc_region: Option<String>,
    pub rate_limit_per_user: u32,
    pub nsfw: bool,
}

impl TryFrom<GatewayChannel> for StageChannel {
    type Error = Unknown;

    fn try_from(value: GatewayChannel) -> Result<Self, Self::Error> {
        let flags = value.flags.unwrap_or_default() as u32;

        let base = guild_base(
            value.id,
            value.guild_id,
            value.name,
            value.position,
            flags,
            value.parent_id,
        );

        Ok(Self {
            base,
            topic: value.topic,
            last_message_id: value.last_message_id,
            bitrate: value.bitrate.unwrap_or_default(),
            user_limit: value.user_limit.unwrap_or_default() as u32,
            rtc_region: value.rtc_region,
            rate_limit_per_user: value.rate_limit_per_user.unwrap_or_default(),
            nsfw: value.nsfw.unwrap_or_default(),
        })
    }
}
