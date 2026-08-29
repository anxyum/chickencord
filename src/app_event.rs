use crate::components::{Guild, Member, Message, User, channel::GuildChannel};
use crate::discord_gateway::PreloadedUserSettings;
use crate::discord_rest::RestResponse;
use discord_client_structs::structs::{
    channel::Channel as GatewayChannel, user::Member as GatewayMember,
};
use std::collections::HashMap;

#[derive(Clone)]
pub enum AppEvent {
    Network(NetworkEvent),
    Rest(RestResponse),
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
