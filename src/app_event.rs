use crate::discord_gateway::PreloadedUserSettings;

pub enum AppEvent {
    CreateGuild {
        id: u64,
        name: String,
        avatar: Option<Vec<u8>>,
    },
    UserSettings(PreloadedUserSettings),
}
