use crate::network::utils::maybe_fetch_avatar;
use bytes::Bytes;
use iced::widget::image::Handle;

#[derive(Debug, Clone)]
pub struct LazyMember {
    pub id: u64,
    pub nick: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Member {
    pub id: u64,
    pub nick: Option<String>,
    pub avatar: Option<Handle>,
}

impl LazyMember {
    pub async fn load(self) -> Member {
        let Self { id, nick, avatar } = self;
        Member {
            id: id,
            nick: nick,
            avatar: maybe_fetch_avatar(avatar.as_deref())
                .await
                .map(|bytes| Handle::from_bytes(Bytes::from(bytes))),
        }
    }
}
