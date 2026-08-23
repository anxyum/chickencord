mod app_event;
mod components;
mod discord_gateway;
mod icons;
mod themes;

use app_event::AppEvent;
use bytes::Bytes;
use components::Guilds;
use iced::{
    Element, Subscription, Task,
    widget::{container, image::Handle, row},
};
use themes::AppTheme;

use crate::app_event::{AppMessage, NetworkEvent};

fn main() -> iced::Result {
    dotenvy::dotenv().expect("failed to load .env");

    iced::application(App::default, update, view)
        .subscription(|_| Subscription::run(discord_gateway::worker))
        .run()
}

#[derive(Default, Debug)]
struct App {
    theme: AppTheme,
    guilds: Guilds,
}

fn update(app: &mut App, message: AppEvent) -> Task<AppEvent> {
    match message {
        AppEvent::Network(event) => match event {
            NetworkEvent::CreateGuild { id, name, avatar } => {
                let avatar = avatar.map(|bytes| Handle::from_bytes(Bytes::from(bytes)));
                app.guilds.create_guild(id, name, avatar);
            }

            NetworkEvent::UserSettings(user_settings) => {
                if let Some(guild_folders) = &user_settings.guild_folders {
                    app.guilds.reorganize(guild_folders)
                }
            }
        },

        AppEvent::Message(message) => match message {
            AppMessage::OpenFolder(id) => {
                app.guilds.open_folder(id);
            }

            AppMessage::CloseFolder(id) => {
                app.guilds.close_folder(id);
            }
        },
    }

    Task::none()
}

fn view(app: &App) -> Element<'_, AppEvent> {
    dbg!(app);

    container(row![app.guilds.show(&app.theme)]).into()
}
