use super::{GatewayChannel, GuildChannelBase, Unknown, guild_base};

#[derive(Debug, Clone)]
pub struct ForumChannel {
    pub base: GuildChannelBase,
    pub kind: ForumKind,
    pub topic: Option<String>,
    pub nsfw: bool,
    pub last_message_id: Option<u64>,
    pub rate_limit_per_user: u32,
    pub default_auto_archive_duration: u32,
    pub default_thread_rate_limit_per_user: u32,
    pub default_sort_order: Option<u8>,
    pub default_forum_layout: u8,
    pub default_tag_setting: String,
    pub available_tags: Vec<ForumTag>,
    pub default_reaction_emoji: Option<DefaultReaction>,
    pub template: Option<String>,
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum ForumKind {
    Forum = 15,
    Media = 16,
}

#[derive(Debug, Clone)]
pub struct ForumTag {
    pub id: u64,
    pub name: String,
    pub emoji_id: Option<u64>,
    pub emoji_name: Option<String>,
    pub moderated: bool,
}

#[derive(Debug, Clone)]
pub struct DefaultReaction {
    pub emoji_id: Option<u64>,
    pub emoji_name: Option<String>,
}

impl TryFrom<GatewayChannel> for ForumChannel {
    type Error = Unknown;

    fn try_from(value: GatewayChannel) -> Result<Self, Self::Error> {
        let flags = value.flags.unwrap_or_default() as u32;

        let kind = match value.r#type {
            15 => ForumKind::Forum,
            16 => ForumKind::Media,
            _ => return Err(Unknown),
        };

        let base = guild_base(
            value.id,
            value.guild_id,
            value.name,
            value.position,
            flags,
            value.parent_id,
        );

        let available_tags = value
            .available_tags
            .unwrap_or_default()
            .into_iter()
            .map(|tag| ForumTag {
                id: tag.id,
                name: tag.name,
                emoji_id: tag.emoji_id,
                emoji_name: tag.emoji_name,
                moderated: tag.moderated,
            })
            .collect();

        let default_reaction_emoji = value
            .default_reaction_emoji
            .map(|reaction| DefaultReaction {
                emoji_id: reaction.emoji_id,
                emoji_name: reaction.emoji_name,
            });

        Ok(Self {
            base,
            kind,
            topic: value.topic,
            nsfw: value.nsfw.unwrap_or_default(),
            last_message_id: value.last_message_id,
            rate_limit_per_user: value.rate_limit_per_user.unwrap_or_default(),
            default_auto_archive_duration: value.default_auto_archive_duration.unwrap_or_default(),
            default_thread_rate_limit_per_user: value
                .default_thread_rate_limit_per_user
                .unwrap_or_default(),
            default_sort_order: value.default_sort_order,
            default_forum_layout: value.default_forum_layout.unwrap_or_default(),
            default_tag_setting: String::new(),
            available_tags,
            default_reaction_emoji,
            template: None,
        })
    }
}
