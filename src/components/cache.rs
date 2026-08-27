use super::{Guild, Member, Messages, User, channel::GuildChannel};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Cache {
    pub guilds: HashMap<u64, Guild>,
    pub channels: HashMap<u64, GuildChannel>,
    pub messages: HashMap<u64, Messages>,
    pub users: HashMap<u64, User>,
    pub members: HashMap<u64, HashMap<u64, Member>>,
}
