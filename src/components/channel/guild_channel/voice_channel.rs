use super::{GatewayChannel, GuildChannelBase, Unknown, guild_base};

#[derive(Debug, Clone)]
pub struct VoiceChannel {
    pub base: GuildChannelBase,
    pub last_message_id: Option<u64>,
    pub bitrate: u32,
    pub user_limit: u32,
    pub rtc_region: Option<String>,
    pub rate_limit_per_user: u32,
    pub nsfw: bool,
    pub video_quality_mode: u8,
}

impl TryFrom<GatewayChannel> for VoiceChannel {
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
        let last_message_id = value.last_message_id;
        let bitrate = value.bitrate.unwrap_or_default();
        let user_limit = value.user_limit.unwrap_or_default() as u32;
        let rtc_region = value.rtc_region;
        let rate_limit_per_user = value.rate_limit_per_user.unwrap_or_default();
        let nsfw = value.nsfw.unwrap_or_default();
        let video_quality_mode = value.video_quality_mode.unwrap_or_default();

        Ok(Self {
            base,
            last_message_id,
            bitrate,
            user_limit,
            rtc_region,
            rate_limit_per_user,
            nsfw,
            video_quality_mode,
        })
    }
}
