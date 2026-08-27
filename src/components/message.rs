use discord_client_structs::structs::message::Message as GatewayMessage;
use iced::{Element, widget::text};

use crate::{Context, app_event::AppEvent};

#[derive(Debug)]
pub struct Message {
    pub id: u64,
    pub content: Option<String>,
    pub author_name: String,
}

impl Message {
    pub fn show(&self, context: &Context) -> Element<'_, AppEvent> {
        text(
            self.content
                .as_ref()
                .map(|v| v.as_str())
                .unwrap_or_default(),
        )
        .into()
    }
}

pub struct Invalid;

impl TryFrom<GatewayMessage> for Message {
    type Error = Invalid;

    fn try_from(value: GatewayMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            content: value.content,
            author_name: value.author.username,
        })
    }
}
