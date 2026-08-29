mod app_event;
mod components;
mod icons;
mod network;
mod themes;

use app_event::{AppEvent, AppMessage, NetworkEvent};
use components::{Cache, Guilds};
use discord_client_structs::structs::message::query::MessageQuery;
use iced::{
    Color, Element, Font, Length, Subscription, Task,
    time::{self, Instant},
    widget::{container, row},
};
use icons::Icons;
use network::{NetworkChannels, PreloadedUserSettings, Request};
use std::{collections::HashSet, time::Duration};
use themes::AppTheme;

pub(crate) const GG_SANS_REGULAR: Font = Font::with_name("gg sans Regular");

fn main() -> iced::Result {
    dotenvy::dotenv().expect("failed to load .env");

    iced::application(|| App::new(), update, view)
        .font(include_bytes!("../fonts/gg sans Regular.ttf"))
        .subscription(subscription)
        .run()
}

fn subscription(app: &App) -> Subscription<AppEvent> {
    let mut subscriptions = vec![Subscription::run_with(
        app.context.network.event_receiver.clone(),
        |handle| network::worker(handle.clone()),
    )];

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
    pub network: NetworkChannels,
}

impl Default for Context {
    fn default() -> Self {
        Self::new(AppTheme::default(), Icons::default())
    }
}

impl Context {
    pub fn new(theme: AppTheme, icons: Icons) -> Self {
        Self {
            theme,
            icons,
            network: network::start(),
        }
    }
}

#[derive(Default, Debug)]
struct App {
    guilds: Guilds,
    cache: Cache,
    context: Context,
    hovered_message: Option<u64>,
    loading_messages: HashSet<u64>,
}

impl App {
    fn new() -> Self {
        let context = Context::default();

        Self {
            guilds: Guilds::new(&context),
            cache: Cache::default(),
            context,
            hovered_message: None,
            loading_messages: HashSet::new(),
        }
    }

    fn apply_user_settings(&mut self, settings: PreloadedUserSettings) {
        let PreloadedUserSettings { guild_folders, .. } = settings;
        if let Some(guild_folders) = guild_folders {
            self.guilds.reorganize(guild_folders, &self.context)
        }
    }

    fn load_channel(&mut self, guild_id: u64, channel_id: u64) {
        if !self.loading_messages.insert(channel_id) {
            return;
        }
        let loaded = self
            .cache
            .messages
            .get(&channel_id)
            .is_some_and(|messages| messages.is_loaded());

        if self.cache.channels.contains_key(&channel_id) && !loaded {
            let _ = self
                .context
                .network
                .request_sender
                .send(Request::FetchMessages {
                    channel_id,
                    guild_id: Some(guild_id),
                    query: MessageQuery {
                        around: None,
                        before: None,
                        after: None,
                        limit: 20,
                    },
                });
        }
    }

    fn subscribe_guild(&mut self, guild_id: u64) {
        let _ = self
            .context
            .network
            .request_sender
            .send(Request::SubscribeGuild { guild_id });
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

            NetworkEvent::Messages {
                channel_id,
                query,
                messages,
                ..
            } => {
                app.loading_messages.remove(&channel_id);
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

                app.subscribe_guild(id);

                app.load_channel(id, app.guilds.selected_channel(id).unwrap());
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

            AppMessage::MessageHover(id, hovered) => match app.hovered_message {
                Some(previous_id) => {
                    if previous_id == id && !hovered {
                        app.hovered_message = None;
                    } else if hovered {
                        app.hovered_message = Some(id);
                    }
                }
                None => {
                    if hovered {
                        app.hovered_message = Some(id);
                    }
                }
            },

            AppMessage::SelectChannel {
                guild_id,
                channel_id,
            } => {
                app.guilds.select_channel(guild_id, channel_id);

                app.load_channel(guild_id, channel_id);
            }

            AppMessage::LoadBefore {
                guild_id,
                channel_id,
                before,
                anchor_bottom,
                offset,
                height,
            } => {
                if app.loading_messages.insert(channel_id) {
                    if let Some(messages) = app.cache.messages.get_mut(&channel_id) {
                        messages.start_restore(anchor_bottom, offset, height);
                    }

                    app.context
                        .network
                        .request_sender
                        .send(Request::FetchMessages {
                            channel_id,
                            guild_id: Some(guild_id),
                            query: MessageQuery {
                                around: None,
                                before: Some(before),
                                after: None,
                                limit: 20,
                            },
                        })
                        .unwrap();
                }
            }

            AppMessage::RestoreScroll { channel_id, offset } => {
                if let Some(messages) = app.cache.messages.get_mut(&channel_id) {
                    messages.clear_restore();
                }
                return iced::widget::operation::scroll_to(
                    iced::widget::Id::new("chickencord-messages-scroll"),
                    iced::widget::operation::AbsoluteOffset {
                        x: None,
                        y: Some(offset),
                    },
                );
            }

            AppMessage::Scroll {
                channel_id,
                anchor_bottom,
                offset,
            } => {
                if let Some(messages) = app.cache.messages.get_mut(&channel_id)
                    && messages.set_anchor_bottom(anchor_bottom)
                {
                    return iced::widget::operation::scroll_to(
                        iced::widget::Id::new("chickencord-messages-scroll"),
                        iced::widget::operation::AbsoluteOffset {
                            x: None,
                            y: Some(offset),
                        },
                    );
                }
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

    if let Some(body) = app
        .guilds
        .show_body(&app.cache, &app.context, app.hovered_message)
    {
        content = content.push(body);
    }

    container(content)
        .width(Length::Fill)
        .style(|_| container::Style::default().background(Color::BLACK))
        .into()
}
