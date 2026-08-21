mod app_event;
mod discord_gateway;
mod icons;

use app_event::AppEvent;
use tokio::sync::mpsc;

use crate::discord_gateway::spawn_discord_client;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("failed to load .env");
    let token = std::env::var("DISCORD_TOKEN").expect("missing token in .env");

    let (sender, receiver) = mpsc::channel::<AppEvent>(100);

    spawn_discord_client(sender, token);
}
