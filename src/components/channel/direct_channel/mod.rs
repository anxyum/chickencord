mod dm_channel;
mod ephemeral_dm;
mod group_dm;

use dm_channel::DmChannel;
use ephemeral_dm::EphemeralDm;
use group_dm::GroupDm;

use discord_client_structs::structs::channel::Channel as GatewayChannel;

use crate::components::channel::Unknown;

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum DirectChannel {
    Dm(DmChannel) = 1,
    Group(GroupDm) = 3,
    Ephemeral(EphemeralDm) = 18,
}

impl TryFrom<GatewayChannel> for DirectChannel {
    type Error = Unknown;

    fn try_from(value: GatewayChannel) -> Result<Self, Self::Error> {
        match value.r#type {
            1 => Ok(Self::Dm(DmChannel::try_from(value)?)),
            3 => Ok(Self::Group(GroupDm::try_from(value)?)),
            18 => Ok(Self::Ephemeral(EphemeralDm::try_from(value)?)),
            _ => Err(Unknown),
        }
    }
}
