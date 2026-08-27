use crate::discord_gateway::PreloadedUserSettings;
use crate::discord_rest::RestResponse;
use discord_client_structs::structs::{channel::Channel, user::Member};

#[derive(Clone)]
pub enum AppEvent {
    Network(NetworkEvent),
    Rest(RestResponse),
    Message(AppMessage),
}

#[derive(Clone)]
pub enum NetworkEvent {
    CreateGuild {
        id: u64,
        name: String,
        avatar: Option<Vec<u8>>,
        channels: Vec<Channel>,
        members: Vec<Member>,
    },
    UserSettings(PreloadedUserSettings),
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
