use crate::network::utils::fetch_user_avatar;
use iced::widget::image::Handle;

#[derive(Debug, Clone)]
pub struct LazyUser {
    pub id: u64,
    pub display_name: Option<String>,
    pub username: String,
    pub avatar: String,
}

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

impl LazyUser {
    pub async fn load(self) -> User {
        let Self {
            id,
            display_name,
            username,
            avatar,
        } = self;

        let avatar = fetch_user_avatar(&avatar, id)
            .await
            .unwrap_or(Handle::from_rgba(1, 1, vec![0, 0, 0, 0]));

        User {
            id,
            display_name,
            username,
            avatar,
        }
    }
}
