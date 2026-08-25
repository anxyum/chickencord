#![allow(dead_code)]

use discord_client_structs::structs::channel::Channel as GatewayChannel;
use std::num::NonZeroU64;

#[derive(Debug)]
pub enum Channel {
    Guild(GuildChannel),
    Direct(DirectChannel),
    Lobby(Lobby),
}

#[repr(u8)]
#[derive(Debug)]
pub enum GuildChannel {
    Text(TextChannel) = 0,
    Voice(VoiceChannel) = 2,
    Category(Category) = 4,
    Store(GuildStore) = 6,
    Thread(Thread) = 10,
    Stage(StageChannel) = 13,
    Directory(GuildDirectory) = 14,
    Forum(ForumChannel) = 15,
}

#[repr(u8)]
#[derive(Debug)]
pub enum DirectChannel {
    Dm(DmChannel) = 1,
    Group(GroupDm) = 3,
    Ephemeral(EphemeralDm) = 18,
}

#[repr(u8)]
#[derive(Debug)]
pub enum TextKind {
    Text = 0,
    News = 5,
}

#[repr(u8)]
#[derive(Debug)]
pub enum ThreadKind {
    News = 10,
    Public = 11,
    Private = 12,
}

#[repr(u8)]
#[derive(Debug)]
pub enum ForumKind {
    Forum = 15,
    Media = 16,
}

#[derive(Debug)]
pub struct GuildChannelBase {
    pub id: u64,
    pub guild_id: u64,
    pub name: String,
    pub position: i64,
    pub flags: u32,
    pub parent_id: Option<NonZeroU64>,
}

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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Category {
    pub base: GuildChannelBase,
    pub children: Vec<u64>,
    pub is_open: bool,
}

#[derive(Debug)]
pub struct GuildStore {
    pub base: GuildChannelBase,
}

#[derive(Debug)]
pub struct GuildDirectory {
    pub base: GuildChannelBase,
    pub last_message_id: Option<u64>,
}

#[derive(Debug)]
pub struct Thread {
    pub base: GuildChannelBase,
    pub kind: ThreadKind,
    pub owner_id: u64,
    pub last_message_id: Option<u64>,
    pub rate_limit_per_user: u32,
    pub message_count: u32,
    pub member_count: u32,
    pub member_ids_preview: Vec<u64>,
    pub metadata: ThreadMetadata,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct DmChannel {
    pub id: u64,
    pub flags: u32,
    pub last_message_id: Option<u64>,
    pub recipients: Vec<u64>,
    pub recipient_flags: u32,
    pub is_message_request: bool,
    pub is_message_request_timestamp: Option<String>,
    pub is_spam: bool,
}

#[derive(Debug)]
pub struct EphemeralDm {
    pub id: u64,
    pub flags: u32,
    pub last_message_id: Option<u64>,
    pub recipients: Vec<u64>,
    pub is_message_request: bool,
    pub is_message_request_timestamp: Option<String>,
    pub is_spam: bool,
}

#[derive(Debug)]
pub struct GroupDm {
    pub id: u64,
    pub name: Option<String>,
    pub flags: u32,
    pub last_message_id: Option<u64>,
    pub recipients: Vec<u64>,
    pub icon: Option<String>,
    pub nicks: Vec<(u64, String)>,
    pub managed: bool,
    pub owner_id: Option<u64>,
    pub application_id: Option<u64>,
    pub blocked_user_warning_dismissed: bool,
    pub recipient_flags: u32,
}

#[derive(Debug)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: u32,
    pub archive_timestamp: String,
    pub locked: bool,
    pub invitable: Option<bool>,
    pub create_timestamp: Option<String>,
}

#[derive(Debug)]
pub struct ForumTag {
    pub id: u64,
    pub name: String,
    pub emoji_id: Option<u64>,
    pub emoji_name: Option<String>,
    pub moderated: bool,
}

#[derive(Debug)]
pub struct DefaultReaction {
    pub emoji_id: Option<u64>,
    pub emoji_name: Option<String>,
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

impl GuildChannel {
    pub fn base(&self) -> &GuildChannelBase {
        match self {
            Self::Text(channel) => &channel.base,
            Self::Voice(channel) => &channel.base,
            Self::Category(category) => &category.base,
            Self::Store(store) => &store.base,
            Self::Thread(thread) => &thread.base,
            Self::Stage(channel) => &channel.base,
            Self::Directory(directory) => &directory.base,
            Self::Forum(channel) => &channel.base,
        }
    }
}

fn nz(id: Option<u64>) -> Option<NonZeroU64> {
    id.and_then(NonZeroU64::new)
}

fn guild_base(
    id: u64,
    guild_id: Option<u64>,
    name: Option<String>,
    position: Option<i64>,
    flags: u32,
    parent_id: Option<u64>,
) -> GuildChannelBase {
    GuildChannelBase {
        id,
        guild_id: guild_id.unwrap_or_default(),
        name: name.unwrap_or_default(),
        position: position.unwrap_or_default(),
        flags,
        parent_id: nz(parent_id),
    }
}

pub struct Unknown;

impl TryFrom<GatewayChannel> for Channel {
    type Error = Unknown;
    fn try_from(value: GatewayChannel) -> Result<Self, Unknown> {
        let flags = value.flags.unwrap_or_default() as u32;

        match value.r#type {
            0 | 5 => {
                let kind = if value.r#type == 0 {
                    TextKind::Text
                } else {
                    TextKind::News
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
                let default_auto_archive_duration =
                    value.default_auto_archive_duration.unwrap_or_default();
                let default_thread_rate_limit_per_user =
                    value.default_thread_rate_limit_per_user.unwrap_or_default();

                Ok(Self::Guild(GuildChannel::Text(TextChannel {
                    base,
                    kind,
                    topic,
                    nsfw,
                    last_message_id,
                    rate_limit_per_user,
                    last_pin_timestamp,
                    default_auto_archive_duration,
                    default_thread_rate_limit_per_user,
                })))
            }

            2 => {
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

                Ok(Self::Guild(GuildChannel::Voice(VoiceChannel {
                    base,
                    last_message_id,
                    bitrate,
                    user_limit,
                    rtc_region,
                    rate_limit_per_user,
                    nsfw,
                    video_quality_mode,
                })))
            }

            4 => Ok(Self::Guild(GuildChannel::Category(Category {
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
            }))),

            6 => Ok(Self::Guild(GuildChannel::Store(GuildStore {
                base: guild_base(
                    value.id,
                    value.guild_id,
                    value.name,
                    value.position,
                    flags,
                    value.parent_id,
                ),
            }))),

            10 | 11 | 12 => {
                let Some(parent_id) = nz(value.parent_id) else {
                    return Err(Unknown);
                };

                let Some(metadata) = value.thread_metadata.map(|metadata| ThreadMetadata {
                    archived: metadata.archived,
                    auto_archive_duration: metadata.auto_archive_duration,
                    archive_timestamp: metadata.archive_timestamp.to_rfc3339(),
                    locked: metadata.locked,
                    invitable: metadata.invitable,
                    create_timestamp: metadata.create_timestamp.map(|t| t.to_rfc3339()),
                }) else {
                    return Err(Unknown);
                };

                let kind = match value.r#type {
                    10 => ThreadKind::News,
                    11 => ThreadKind::Public,
                    _ => ThreadKind::Private,
                };

                let owner_id = value.owner_id.unwrap_or_default();
                let last_message_id = value.last_message_id;
                let rate_limit_per_user = value.rate_limit_per_user.unwrap_or_default();
                let message_count = value.message_count.unwrap_or_default();
                let member_count = value.member_count.unwrap_or_default();
                let base = GuildChannelBase {
                    id: value.id,
                    guild_id: value.guild_id.unwrap_or_default(),
                    name: value.name.unwrap_or_default(),
                    position: value.position.unwrap_or_default(),
                    flags,
                    parent_id: Some(parent_id),
                };

                Ok(Self::Guild(GuildChannel::Thread(Thread {
                    base,
                    kind,
                    owner_id,
                    last_message_id,
                    rate_limit_per_user,
                    message_count,
                    member_count,
                    member_ids_preview: Vec::new(),
                    metadata,
                })))
            }

            13 => {
                let base = guild_base(
                    value.id,
                    value.guild_id,
                    value.name,
                    value.position,
                    flags,
                    value.parent_id,
                );
                let topic = value.topic;
                let last_message_id = value.last_message_id;
                let bitrate = value.bitrate.unwrap_or_default();
                let user_limit = value.user_limit.unwrap_or_default() as u32;
                let rtc_region = value.rtc_region;
                let rate_limit_per_user = value.rate_limit_per_user.unwrap_or_default();
                let nsfw = value.nsfw.unwrap_or_default();

                Ok(Self::Guild(GuildChannel::Stage(StageChannel {
                    base,
                    topic,
                    last_message_id,
                    bitrate,
                    user_limit,
                    rtc_region,
                    rate_limit_per_user,
                    nsfw,
                })))
            }

            14 => {
                let last_message_id = value.last_message_id;

                Ok(Self::Guild(GuildChannel::Directory(GuildDirectory {
                    base: guild_base(
                        value.id,
                        value.guild_id,
                        value.name,
                        value.position,
                        flags,
                        value.parent_id,
                    ),
                    last_message_id,
                })))
            }

            15 | 16 => {
                let kind = if value.r#type == 15 {
                    ForumKind::Forum
                } else {
                    ForumKind::Media
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
                let nsfw = value.nsfw.unwrap_or_default();
                let last_message_id = value.last_message_id;
                let rate_limit_per_user = value.rate_limit_per_user.unwrap_or_default();
                let default_auto_archive_duration =
                    value.default_auto_archive_duration.unwrap_or_default();
                let default_thread_rate_limit_per_user =
                    value.default_thread_rate_limit_per_user.unwrap_or_default();
                let default_sort_order = value.default_sort_order;
                let default_forum_layout = value.default_forum_layout.unwrap_or_default();
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
                let default_reaction_emoji =
                    value
                        .default_reaction_emoji
                        .map(|reaction| DefaultReaction {
                            emoji_id: reaction.emoji_id,
                            emoji_name: reaction.emoji_name,
                        });

                Ok(Self::Guild(GuildChannel::Forum(ForumChannel {
                    base,
                    kind,
                    topic,
                    nsfw,
                    last_message_id,
                    rate_limit_per_user,
                    default_auto_archive_duration,
                    default_thread_rate_limit_per_user,
                    default_sort_order,
                    default_forum_layout,
                    default_tag_setting: String::new(),
                    available_tags,
                    default_reaction_emoji,
                    template: None,
                })))
            }

            1 => Ok(Self::Direct(DirectChannel::Dm(DmChannel {
                id: value.id,
                flags,
                last_message_id: value.last_message_id,
                recipients: recipients(value.recipients),
                recipient_flags: 0,
                is_message_request: false,
                is_message_request_timestamp: None,
                is_spam: false,
            }))),

            3 => Ok(Self::Direct(DirectChannel::Group(GroupDm {
                id: value.id,
                name: value.name,
                flags,
                last_message_id: value.last_message_id,
                recipients: recipients(value.recipients),
                icon: value.icon,
                nicks: Vec::new(),
                managed: value.managed.unwrap_or_default(),
                owner_id: value.owner_id,
                application_id: value.application_id,
                blocked_user_warning_dismissed: false,
                recipient_flags: 0,
            }))),

            18 => Ok(Self::Direct(DirectChannel::Ephemeral(EphemeralDm {
                id: value.id,
                flags,
                last_message_id: value.last_message_id,
                recipients: recipients(value.recipients),
                is_message_request: false,
                is_message_request_timestamp: None,
                is_spam: false,
            }))),

            17 => Ok(Self::Lobby(Lobby {
                id: value.id,
                guild_id: value.guild_id,
                name: value.name,
                position: value.position,
                flags,
                parent_id: nz(value.parent_id),
                last_message_id: value.last_message_id,
            })),

            _ => Err(Unknown),
        }
    }
}

fn recipients(recipients: Option<Vec<discord_client_structs::structs::user::User>>) -> Vec<u64> {
    recipients
        .unwrap_or_default()
        .into_iter()
        .map(|user| user.id)
        .collect()
}
