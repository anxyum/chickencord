use discord_client_structs::structs::user::Member as GatewayMember;

#[derive(Debug)]
pub struct Member {
    pub id: u64,
}

impl TryFrom<GatewayMember> for Member {
    type Error = Option<()>;

    fn try_from(value: GatewayMember) -> Result<Self, Self::Error> {
        let Some(id) = value.user.map(|v| v.id) else {
            return Err(None);
        };
        Ok(Self { id })
    }
}
