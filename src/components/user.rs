use discord_client_structs::structs::user::User as GatewayUser;
use iced::widget::image::Handle;

#[derive(Debug, Clone)]
pub struct User {
    pub id: u64,
    display_name: Option<String>,
    pub username: String,
    pub avatar: Handle,
}

impl User {
    pub fn display_name(&self) -> &str {
        self.display_name.as_ref().unwrap_or(&self.username)
    }

    pub fn new(id: u64, display_name: Option<String>, username: String, avatar: Handle) -> Self {
        Self {
            id,
            display_name,
            username,
            avatar,
        }
    }
}

impl From<GatewayUser> for User {
    fn from(value: GatewayUser) -> Self {
        Self {
            id: value.id,
            display_name: value.global_name,
            username: value.username,
            avatar: Handle::from_rgba(1, 1, vec![0, 0, 0, 0]),
        }
    }
}
