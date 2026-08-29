use std::num::NonZeroU64;

#[derive(Debug, Clone)]
pub struct GuildChannelBase {
    pub id: u64,
    pub guild_id: u64,
    pub name: String,
    pub position: i64,
    pub flags: u32,
    pub parent_id: Option<NonZeroU64>,
    pub hovered: bool,
}
