mod category;
mod channel_base;
mod forum_channel;
mod guild_directory;
mod guild_store;
mod stage_channel;
mod text_channel;
mod thread;
mod voice_channel;

use category::Category;
pub(crate) use channel_base::GuildChannelBase;
use forum_channel::ForumChannel;
use guild_directory::GuildDirectory;
use guild_store::GuildStore;
use stage_channel::StageChannel;
use text_channel::TextChannel;
use thread::Thread;
use voice_channel::VoiceChannel;

use super::{Unknown, nz};
use discord_client_structs::structs::channel::Channel as GatewayChannel;

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

impl TryFrom<GatewayChannel> for GuildChannel {
    type Error = Unknown;
    fn try_from(value: GatewayChannel) -> Result<Self, Unknown> {
        match value.r#type {
            0 | 5 => Ok(Self::Text(TextChannel::try_from(value)?)),
            2 => Ok(Self::Voice(VoiceChannel::try_from(value)?)),
            4 => Ok(Self::Category(Category::try_from(value)?)),
            6 => Ok(Self::Store(GuildStore::try_from(value)?)),
            10 | 11 | 12 => Ok(Self::Thread(Thread::try_from(value)?)),
            13 => Ok(Self::Stage(StageChannel::try_from(value)?)),
            14 => Ok(Self::Directory(GuildDirectory::try_from(value)?)),
            15 | 16 => Ok(Self::Forum(ForumChannel::try_from(value)?)),
            _ => Err(Unknown),
        }
    }
}

pub(crate) fn guild_base(
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
