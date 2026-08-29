use crate::components::{Guild, Member, Message, User, channel::GuildChannel};
use crate::network::PreloadedUserSettings;
use discord_client_structs::structs::message::{Message as RawMessage, query::MessageQuery};
use std::collections::HashMap;

#[derive(Clone)]
pub enum AppEvent {
    Network(NetworkEvent),
    Message(AppMessage),
}

#[derive(Clone)]
pub enum NetworkEvent {
    Ready {
        guilds: Vec<Guild>,
        members: HashMap<u64, Vec<Member>>,
        channels: HashMap<u64, Vec<GuildChannel>>,
        users: Vec<User>,
        user_settings: PreloadedUserSettings,
    },
    CreateGuild {
        guild: Guild,
        members: Vec<Member>,
        channels: Vec<GuildChannel>,
    },
    UserSettings(PreloadedUserSettings),
    MessageCreate {
        message: Message,
        channel_id: u64,
    },
    Messages {
        channel_id: u64,
        guild_id: Option<u64>,
        query: MessageQuery,
        messages: Vec<RawMessage>,
    },
}

#[derive(Clone)]
pub enum AppMessage {
    ToggleFolder(u64),
    OpenGuild(u64),
    ChannelPanelResized(f32),
    ToggleCategory(u64),
    ChannelHover(u64, bool),
    SelectChannel { guild_id: u64, channel_id: u64 },
    Tick,
}
