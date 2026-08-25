use discord_client_structs::structs::channel::Channel as GatewayChannel;

use crate::components::channel::{Unknown, recipients};

#[derive(Debug)]
pub struct GroupDm {
    pub id: u64,
    pub name: Option<String>,
    pub flags: u32,
    pub last_message_id: Option<u64>,
    pub recipients: Vec<u64>,
    pub icon: Option<String>,
    pub nicks: Vec<(u64, String)>,
    pub managed: bool,
    pub owner_id: Option<u64>,
    pub application_id: Option<u64>,
    pub blocked_user_warning_dismissed: bool,
    pub recipient_flags: u32,
}

impl TryFrom<GatewayChannel> for GroupDm {
    type Error = Unknown;

    fn try_from(value: GatewayChannel) -> Result<Self, Self::Error> {
        let flags = value.flags.unwrap_or_default() as u32;

        Ok(Self {
            id: value.id,
            name: value.name,
            flags,
            last_message_id: value.last_message_id,
            recipients: recipients(value.recipients),
            icon: value.icon,
            nicks: Vec::new(),
            managed: value.managed.unwrap_or_default(),
            owner_id: value.owner_id,
            application_id: value.application_id,
            blocked_user_warning_dismissed: false,
            recipient_flags: 0,
        })
    }
}
