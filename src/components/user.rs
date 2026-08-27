use discord_client_structs::structs::user::User as GatewayUser;

#[derive(Debug)]
pub struct User {
    pub id: u64,
    pub username: String,
}

impl From<GatewayUser> for User {
    fn from(value: GatewayUser) -> Self {
        Self {
            id: value.id,
            username: value.username,
        }
    }
}
