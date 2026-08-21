mod handle_ready;
mod user_settings;

pub use user_settings::PreloadedUserSettings;

use handle_ready::handle_ready;

use crate::app_event::AppEvent;
use discord_client_gateway::{events::Event, gateway::GatewayClient};
use tokio::sync::mpsc::Sender;

pub fn spawn_discord_client(mut sender: Sender<AppEvent>, token: String) {
    tokio::spawn(async move {
        let capabilities = 53607934;

        let mut client = GatewayClient::connect(token, true, capabilities, None)
            .await
            .unwrap();

        loop {
            let event = client.next_event().await.unwrap();
            match event {
                Event::Ready(event) => handle_ready(event, &mut sender).await,
                _ => {}
            }
        }
    });
}
