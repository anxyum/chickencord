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
pub use channel_base::GuildChannelBase;
use forum_channel::ForumChannel;
use guild_directory::GuildDirectory;
use guild_store::GuildStore;
use stage_channel::StageChannel;
use text_channel::TextChannel;
use thread::Thread;
use voice_channel::VoiceChannel;

use super::{Unknown, nz};
use crate::{Context, app_event::AppEvent, components::Messages};
use discord_client_structs::structs::channel::Channel as GatewayChannel;
use iced::{Color, Element, widget::text};

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

    pub fn base_mut(&mut self) -> &mut GuildChannelBase {
        match self {
            Self::Text(channel) => &mut channel.base,
            Self::Voice(channel) => &mut channel.base,
            Self::Category(category) => &mut category.base,
            Self::Store(store) => &mut store.base,
            Self::Thread(thread) => &mut thread.base,
            Self::Stage(channel) => &mut channel.base,
            Self::Directory(directory) => &mut directory.base,
            Self::Forum(channel) => &mut channel.base,
        }
    }

    pub fn show<'a>(
        &'a self,
        context: &'a Context,
        selected_channel: u64,
        is_open: bool,
    ) -> Element<'a, AppEvent> {
        match self {
            GuildChannel::Text(channel) => channel.show(context, selected_channel).into(),
            GuildChannel::Category(category) => category.show(context, is_open).into(),

            _ => text("not implemented yet")
                .font(crate::GG_SANS_REGULAR)
                .color(Color::from_rgb8(255, 0, 0))
                .into(),
        }
    }

    pub fn set_hovered(&mut self, hovered: bool) {
        self.base_mut().hovered = hovered
    }

    pub fn show_body<'a>(
        &'a self,
        messages: Option<&'a Messages>,
        context: &'a Context,
    ) -> Option<Element<'a, AppEvent>> {
        match self {
            Self::Text(channel) => Some(channel.show_body(messages, context)),
            _ => None,
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
        hovered: false,
    }
}
