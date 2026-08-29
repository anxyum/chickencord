use discord_client_structs::structs::channel::Channel as GatewayChannel;

use crate::components::channel::{Unknown, recipients};

#[derive(Debug, Clone)]
pub struct DmChannel {
    pub id: u64,
    pub flags: u32,
    pub last_message_id: Option<u64>,
    pub recipients: Vec<u64>,
    pub recipient_flags: u32,
    pub is_message_request: bool,
    pub is_message_request_timestamp: Option<String>,
    pub is_spam: bool,
}

impl TryFrom<GatewayChannel> for DmChannel {
    type Error = Unknown;

    fn try_from(value: GatewayChannel) -> Result<Self, Self::Error> {
        let flags = value.flags.unwrap_or_default() as u32;

        Ok(Self {
            id: value.id,
            flags,
            last_message_id: value.last_message_id,
            recipients: recipients(value.recipients),
            recipient_flags: 0,
            is_message_request: false,
            is_message_request_timestamp: None,
            is_spam: false,
        })
    }
}
