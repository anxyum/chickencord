mod app_event;
mod components;
mod discord_gateway;
mod icons;
mod themes;

use app_event::AppEvent;
use bytes::Bytes;
use components::Guilds;
use iced::{
    Element, Subscription, Task, time,
    widget::{container, image::Handle, row},
};
use std::time::Duration;
use themes::AppTheme;

use crate::app_event::{AppMessage, NetworkEvent};

fn main() -> iced::Result {
    dotenvy::dotenv().expect("failed to load .env");

    iced::application(|| App::new(themes::AppTheme::default()), update, view)
        .subscription(subscription)
        .run()
}

fn subscription(app: &App) -> Subscription<AppEvent> {
    let mut subscriptions = vec![Subscription::run(discord_gateway::worker)];

    if app.guilds.is_animating() {
        subscriptions.push(
            time::every(Duration::from_millis(16)).map(|_| AppEvent::Message(AppMessage::Tick)),
        );
    }

    Subscription::batch(subscriptions)
}

#[derive(Default, Debug)]
struct App {
    theme: AppTheme,
    guilds: Guilds,
}

impl App {
    fn new(theme: AppTheme) -> Self {
        let guilds = Guilds::new(&theme.guilds);

        Self { theme, guilds }
    }
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
                    app.guilds.reorganize(guild_folders, &app.theme.guilds)
                }
            }
        },

        AppEvent::Message(message) => match message {
            AppMessage::ToggleFolder(id) => {
                app.guilds.toggle_folder(id, iced::time::Instant::now());
            }

            AppMessage::Tick => {}
        },
    }

    Task::none()
}

fn view(app: &App) -> Element<'_, AppEvent> {
    container(row![app.guilds.show(&app.theme)]).into()
}
