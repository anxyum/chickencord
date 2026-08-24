use crate::discord_gateway::PreloadedUserSettings;

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
    },
    UserSettings(PreloadedUserSettings),
}

#[derive(Clone)]
pub enum AppMessage {
    ToggleFolder(u64),
    Tick,
}
