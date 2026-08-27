mod app_event;
mod components;
mod discord_gateway;
mod discord_rest;
mod icons;
mod themes;

use app_event::{AppEvent, AppMessage, NetworkEvent};
use bytes::Bytes;
use components::{Channel, Guilds, Member};
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
                id: guild_id,
                name,
                avatar,
                channels,
                members,
            } => {
                let avatar = avatar.map(|bytes| Handle::from_bytes(Bytes::from(bytes)));
                app.guilds.create_guild(
                    guild_id,
                    name,
                    avatar,
                    channels
                        .into_iter()
                        .filter_map(|c| {
                            let channel = c.try_into().ok()?;
                            match channel {
                                Channel::Guild(mut channel) => {
                                    channel.base_mut().guild_id = guild_id;
                                    Some((channel.base().id, channel))
                                }
                                _ => None,
                            }
                        })
                        .collect(),
                    members
                        .into_iter()
                        .filter_map(|m| {
                            let member: Member = m.try_into().ok()?;
                            Some((member.id, member))
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

        AppEvent::Rest(response) => match response {
            RestResponse::Messages {
                channel_id,
                guild_id,
                query,
                messages,
            } => {
                if let Some(guild_id) = guild_id {
                    let messages = messages
                        .into_iter()
                        .filter_map(|m| m.try_into().ok())
                        .collect();
                    app.guilds
                        .load_messages(guild_id, channel_id, query, messages);
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

            AppMessage::ChannelHover(id, hovered) => {
                app.guilds.channel_hover(id, hovered);
            }

            AppMessage::SelectChannel {
                guild_id,
                channel_id,
            } => {
                app.guilds.select_channel(guild_id, channel_id);
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

    if let Some(body) = app.guilds.show_body(&app.context) {
        content = content.push(body);
    }

    container(content)
        .width(Length::Fill)
        .style(|_| container::Style::default().background(Color::BLACK))
        .into()
}
