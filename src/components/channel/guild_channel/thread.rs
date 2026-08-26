use super::{GatewayChannel, GuildChannelBase, Unknown};
use crate::components::channel::nz;

#[derive(Debug)]
pub struct Thread {
    pub base: GuildChannelBase,
    pub kind: ThreadKind,
    pub owner_id: u64,
    pub last_message_id: Option<u64>,
    pub rate_limit_per_user: u32,
    pub message_count: u32,
    pub member_count: u32,
    pub member_ids_preview: Vec<u64>,
    pub metadata: ThreadMetadata,
}

#[repr(u8)]
#[derive(Debug)]
pub enum ThreadKind {
    News = 10,
    Public = 11,
    Private = 12,
}

#[derive(Debug)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: u32,
    pub archive_timestamp: String,
    pub locked: bool,
    pub invitable: Option<bool>,
    pub create_timestamp: Option<String>,
}

impl TryFrom<GatewayChannel> for Thread {
    type Error = Unknown;

    fn try_from(value: GatewayChannel) -> Result<Self, Self::Error> {
        let flags = value.flags.unwrap_or_default() as u32;

        let Some(parent_id) = nz(value.parent_id) else {
            return Err(Unknown);
        };

        let Some(metadata) = value.thread_metadata.map(|metadata| ThreadMetadata {
            archived: metadata.archived,
            auto_archive_duration: metadata.auto_archive_duration,
            archive_timestamp: metadata.archive_timestamp.to_rfc3339(),
            locked: metadata.locked,
            invitable: metadata.invitable,
            create_timestamp: metadata.create_timestamp.map(|t| t.to_rfc3339()),
        }) else {
            return Err(Unknown);
        };

        let kind = match value.r#type {
            10 => ThreadKind::News,
            11 => ThreadKind::Public,
            12 => ThreadKind::Private,
            _ => return Err(Unknown),
        };

        let base = GuildChannelBase {
            id: value.id,
            guild_id: value.guild_id.unwrap_or_default(),
            name: value.name.unwrap_or_default(),
            position: value.position.unwrap_or_default(),
            flags,
            parent_id: Some(parent_id),
            hovered: false,
        };

        Ok(Self {
            base,
            kind,
            owner_id: value.owner_id.unwrap_or_default(),
            last_message_id: value.last_message_id,
            rate_limit_per_user: value.rate_limit_per_user.unwrap_or_default(),
            message_count: value.message_count.unwrap_or_default(),
            member_count: value.member_count.unwrap_or_default(),
            member_ids_preview: Vec::new(),
            metadata,
        })
    }
}
