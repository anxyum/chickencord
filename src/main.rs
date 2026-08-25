mod app_event;
mod components;
mod discord_gateway;
mod icons;
mod themes;

use app_event::AppEvent;
use bytes::Bytes;
use components::Guilds;
use iced::{
    Color, Element, Length, Subscription, Task,
    time::{self, Instant},
    widget::{container, image::Handle, row},
};
use icons::Icons;
use std::time::Duration;
use themes::AppTheme;

use crate::{
    app_event::{AppMessage, NetworkEvent},
    components::Channel,
    discord_gateway::PreloadedUserSettings,
};

fn main() -> iced::Result {
    dotenvy::dotenv().expect("failed to load .env");

    iced::application(|| App::new(), update, view)
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
pub struct Context {
    pub theme: AppTheme,
    pub icons: Icons,
}

impl Context {
    pub fn new(theme: AppTheme, icons: Icons) -> Self {
        Self { theme, icons }
    }
}

#[derive(Default, Debug)]
struct App {
    guilds: Guilds,
    context: Context,
}

impl App {
    fn new() -> Self {
        let context = Context::default();

        Self {
            guilds: Guilds::new(&context),
            context,
        }
    }
}

fn update(app: &mut App, message: AppEvent) -> Task<AppEvent> {
    match message {
        AppEvent::Network(event) => match event {
            NetworkEvent::CreateGuild {
                id,
                name,
                avatar,
                channels,
            } => {
                let avatar = avatar.map(|bytes| Handle::from_bytes(Bytes::from(bytes)));
                app.guilds.create_guild(
                    id,
                    name,
                    avatar,
                    channels
                        .into_iter()
                        .filter_map(|c| {
                            let channel = c.try_into().ok()?;
                            match channel {
                                Channel::Guild(channel) => Some((channel.base().id, channel)),
                                _ => None,
                            }
                        })
                        .collect(),
                );
            }

            NetworkEvent::UserSettings(user_settings) => {
                let PreloadedUserSettings { guild_folders, .. } = user_settings;
                if let Some(guild_folders) = guild_folders {
                    app.guilds.reorganize(guild_folders, &app.context)
                }
            }
        },

        AppEvent::Message(message) => match message {
            AppMessage::ToggleFolder(id) => {
                app.guilds.toggle_folder(id, Instant::now());
            }

            AppMessage::OpenGuild(id) => {
                app.guilds.open_guild(id);
            }

            AppMessage::ChannelPanelResized(width) => {
                app.guilds.set_channel_panel_width(width);
            }

            AppMessage::ToggleCategory(channel_id) => {
                app.guilds.toggle_category(channel_id);
            }

            AppMessage::Tick => {}
        },
    }

    Task::none()
}

fn view(app: &App) -> Element<'_, AppEvent> {
    let mut content = row![app.guilds.show(&app.context)];

    if let Some(channels) = app.guilds.show_opened_guild_channels(&app.context) {
        content = content.push(channels);
        content = content.push(app.guilds.channel_resize_divider());
    }

    container(content)
        .width(Length::Fill)
        .style(|_| container::Style::default().background(Color::BLACK))
        .into()
}
