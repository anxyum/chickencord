mod app_event;
mod components;
mod discord_gateway;
mod icons;

use app_event::AppEvent;
use bytes::Bytes;
use components::Guilds;
use iced::{
    Element, Subscription, Task,
    widget::{image::Handle, row},
};

fn main() -> iced::Result {
    dotenvy::dotenv().expect("failed to load .env");

    iced::application(App::default, update, view)
        .subscription(|_| Subscription::run(discord_gateway::worker))
        .run()
}

#[derive(Default, Debug)]
struct App {
    guilds: Guilds,
}

fn update(app: &mut App, message: AppEvent) -> Task<AppEvent> {
    match message {
        AppEvent::CreateGuild { id, name, avatar } => {
            let avatar = avatar.map(|bytes| Handle::from_bytes(Bytes::from(bytes)));
            app.guilds.create_guild(id, name, avatar);
        }
        _ => {}
    }

    Task::none()
}

fn view(app: &App) -> Element<'_, AppEvent> {
    dbg!(app);
    row![app.guilds.show()].into()
}
