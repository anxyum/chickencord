mod app_event;
mod components;
mod discord_gateway;
mod discord_rest;
mod icons;
mod themes;

use app_event::{AppEvent, AppMessage, NetworkEvent};
use bytes::Bytes;
use components::{Cache, Channel, Guilds};
use discord_client_structs::structs::message::query::MessageQuery;
use discord_gateway::PreloadedUserSettings;
use discord_rest::{RestChannels, RestResponse};
use iced::{
    Color, Element, Font, Length, Subscription, Task,
    time::{self, Instant},
    widget::{container, image::Handle, row},
};
use icons::Icons;
use std::time::Duration;
use themes::AppTheme;

use crate::discord_rest::RestRequest;

pub(crate) const GG_SANS_REGULAR: Font = Font::with_name("gg sans Regular");

fn main() -> iced::Result {
    dotenvy::dotenv().expect("failed to load .env");

    iced::application(|| App::new(), update, view)
        .font(include_bytes!("../fonts/gg sans Regular.ttf"))
        .subscription(subscription)
        .run()
}

fn subscription(app: &App) -> Subscription<AppEvent> {
    let mut subscriptions = vec![
        Subscription::run(discord_gateway::worker),
        Subscription::run_with(app.context.rest.response_receiver.clone(), |handle| {
            discord_rest::worker(handle.0.clone())
        }),
    ];

    if app.guilds.is_animating() {
        subscriptions.push(
            time::every(Duration::from_millis(16)).map(|_| AppEvent::Message(AppMessage::Tick)),
        );
    }

    Subscription::batch(subscriptions)
}

#[derive(Debug)]
pub struct Context {
    pub theme: AppTheme,
    pub icons: Icons,
    pub rest: RestChannels,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            theme: AppTheme::default(),
            icons: Icons::default(),
            rest: discord_rest::start(),
        }
    }
}

impl Context {
    pub fn new(theme: AppTheme, icons: Icons) -> Self {
        Self {
            theme,
            icons,
            rest: discord_rest::start(),
        }
    }
}

#[derive(Default, Debug)]
struct App {
    guilds: Guilds,
    cache: Cache,
    context: Context,
}

impl App {
    fn new() -> Self {
        let context = Context::default();

        Self {
            guilds: Guilds::new(&context),
            cache: Cache::default(),
            context,
        }
    }

    fn apply_user_settings(&mut self, settings: PreloadedUserSettings) {
        let PreloadedUserSettings { guild_folders, .. } = settings;
        if let Some(guild_folders) = guild_folders {
            self.guilds.reorganize(guild_folders, &self.context)
        }
    }
}

fn load_channel(app: &mut App, guild_id: u64, channel_id: u64) {
    let loaded = app
        .cache
        .messages
        .get(&channel_id)
        .is_some_and(|messages| messages.is_loaded());

    if app.cache.channels.contains_key(&channel_id) && !loaded {
        let request_sender = app.context.rest.request_sender.clone();

        tokio::spawn(async move {
            _ = request_sender
                .send(RestRequest::FetchMessages {
                    channel_id,
                    guild_id: Some(guild_id),
                    query: MessageQuery {
                        around: None,
                        before: None,
                        after: None,
                        limit: 20,
                    },
                })
                .await;
        });
    }
}

fn update(app: &mut App, message: AppEvent) -> Task<AppEvent> {
    match message {
        AppEvent::Network(event) => match event {
            NetworkEvent::Ready {
                guilds,
                mut channels,
                mut members,
                user_settings,
                users,
            } => {
                for guild in guilds {
                    let guild_id = guild.id;
                    app.guilds.add_guild(
                        guild,
                        channels.remove(&guild_id).unwrap(),
                        members.remove(&guild_id).unwrap(),
                        &mut app.cache,
                    );
                }

                for user in users {
                    app.cache.users.insert(user.id, user);
                }

                app.apply_user_settings(user_settings);
            }

            NetworkEvent::CreateGuild {
                guild,
                members,
                channels,
            } => {
                app.guilds
                    .add_guild(guild, channels, members, &mut app.cache);
            }

            NetworkEvent::UserSettings(user_settings) => {
                app.apply_user_settings(user_settings);
            }

            NetworkEvent::MessageCreate {
                message,
                channel_id,
            } => {
                if let Some(messages) = app.cache.messages.get_mut(&channel_id)
                    && messages.is_loaded()
                {
                    messages.new_message(message);
                }
            }
        },

        AppEvent::Rest(response) => match response {
            RestResponse::Messages {
                channel_id,
                query,
                messages,
                ..
            } => {
                let messages = messages
                    .into_iter()
                    .filter_map(|m| m.try_into().ok())
                    .collect();
                app.guilds
                    .load_messages(&mut app.cache, channel_id, query, messages);
            }
        },

        AppEvent::Message(message) => match message {
            AppMessage::ToggleFolder(id) => {
                app.guilds.toggle_folder(id, Instant::now());
            }

            AppMessage::OpenGuild(id) => {
                app.guilds.open_guild(id);

                load_channel(app, id, app.guilds.selected_channel(id).unwrap());
            }

            AppMessage::ChannelPanelResized(width) => {
                app.guilds.set_channel_panel_width(width);
            }

            AppMessage::ToggleCategory(channel_id) => {
                app.guilds.toggle_category(channel_id);
            }

            AppMessage::ChannelHover(id, hovered) => {
                app.guilds.channel_hover(&mut app.cache, id, hovered);
            }

            AppMessage::SelectChannel {
                guild_id,
                channel_id,
            } => {
                app.guilds.select_channel(guild_id, channel_id);

                load_channel(app, guild_id, channel_id);
            }

            AppMessage::Tick => {}
        },
    }

    Task::none()
}

fn view(app: &App) -> Element<'_, AppEvent> {
    let mut content = row![app.guilds.show(&app.cache, &app.context)];

    if let Some(channels) = app
        .guilds
        .show_opened_guild_channels(&app.cache, &app.context)
    {
        content = content.push(channels);
        content = content.push(app.guilds.channel_resize_divider());
    }

    if let Some(body) = app.guilds.show_body(&app.cache, &app.context) {
        content = content.push(body);
    }

    container(content)
        .width(Length::Fill)
        .style(|_| container::Style::default().background(Color::BLACK))
        .into()
}
