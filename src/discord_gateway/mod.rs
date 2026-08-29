mod handle_message_create;
mod handle_ready;
pub mod user_settings;

pub use user_settings::PreloadedUserSettings;

use handle_message_create::handle_message_create;
use handle_ready::handle_ready;

use crate::app_event::AppEvent;
use discord_client_gateway::{events::Event, gateway::GatewayClient};
use iced::{futures::Stream, stream};

pub fn worker() -> impl Stream<Item = AppEvent> {
    stream::channel(100, async move |mut output| {
        let token = std::env::var("DISCORD_TOKEN").expect("missing token in .env");

        let capabilities = 53607934;

        let mut client = GatewayClient::connect(token, true, capabilities, None)
            .await
            .expect("failed to connect the client");

        loop {
            let event = client.next_event().await.unwrap();
            match event {
                Event::Ready(event) => handle_ready(event, &mut output).await,
                Event::MessageCreate(event) => handle_message_create(event, &mut output).await,
                _ => {}
            }
        }
    })
}
