use crate::discord_gateway::PreloadedUserSettings;
use discord_client_structs::structs::channel::Channel;

#[derive(Clone)]
pub enum AppEvent {
    Network(NetworkEvent),
    Message(AppMessage),
}

#[derive(Clone)]
pub enum NetworkEvent {
    CreateGuild {
        id: u64,
        name: String,
        avatar: Option<Vec<u8>>,
        channels: Vec<Channel>,
    },
    UserSettings(PreloadedUserSettings),
}

#[derive(Clone)]
pub enum AppMessage {
    ToggleFolder(u64),
    OpenGuild(u64),
    ChannelPanelResized(f32),
    ToggleCategory(u64),
    Tick,
}
