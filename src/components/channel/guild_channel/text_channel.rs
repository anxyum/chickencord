use super::{GatewayChannel, GuildChannelBase, Unknown, guild_base};

#[derive(Debug)]
pub struct TextChannel {
    pub base: GuildChannelBase,
    pub kind: TextKind,
    pub topic: Option<String>,
    pub nsfw: bool,
    pub last_message_id: Option<u64>,
    pub rate_limit_per_user: u32,
    pub last_pin_timestamp: Option<String>,
    pub default_auto_archive_duration: u32,
    pub default_thread_rate_limit_per_user: u32,
}

#[repr(u8)]
#[derive(Debug)]
pub enum TextKind {
    Text = 0,
    News = 5,
}

impl TryFrom<GatewayChannel> for TextChannel {
    type Error = Unknown;

    fn try_from(value: GatewayChannel) -> Result<Self, Self::Error> {
        let flags = value.flags.unwrap_or_default() as u32;

        let kind = match value.r#type {
            0 => TextKind::Text,
            5 => TextKind::News,
            _ => {
                return Err(Unknown);
            }
        };

        let base = guild_base(
            value.id,
            value.guild_id,
            value.name,
            value.position,
            flags,
            value.parent_id,
        );
        let topic = value.topic;
        let nsfw = value.nsfw.unwrap_or(false);
        let last_message_id = value.last_message_id;
        let rate_limit_per_user = value.rate_limit_per_user.unwrap_or_default();
        let last_pin_timestamp = value.last_pin_timestamp.map(|t| t.to_rfc3339());
        let default_auto_archive_duration = value.default_auto_archive_duration.unwrap_or_default();
        let default_thread_rate_limit_per_user =
            value.default_thread_rate_limit_per_user.unwrap_or_default();

        Ok(Self {
            base,
            kind,
            topic,
            nsfw,
            last_message_id,
            rate_limit_per_user,
            last_pin_timestamp,
            default_auto_archive_duration,
            default_thread_rate_limit_per_user,
        })
    }
}
