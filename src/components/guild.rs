use super::{Channels, Member, channel::GuildChannel};
use crate::{
    Context,
    app_event::{AppEvent, AppMessage},
    components::{Message, button},
};
use discord_client_structs::structs::message::query::MessageQuery;
use iced::{
    Background, Border, Color, Element, Length, alignment,
    border::Radius,
    widget::{
        column, container,
        image::{Handle, Image},
        text,
    },
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Guild {
    id: u64,
    name: String,
    initials: String,
    avatar: Option<Handle>,
    channels: Channels,
    members: HashMap<u64, Member>,
}

impl Guild {
    pub fn new(
        id: u64,
        name: String,
        avatar: Option<Handle>,
        channels: HashMap<u64, GuildChannel>,
        members: HashMap<u64, Member>,
    ) -> Self {
        let initials = get_initials(&name);

        Self {
            id,
            name,
            initials,
            avatar,
            channels: Channels::new(channels),
            members,
        }
    }

    pub fn show_avatar(
        &self,
        context: &Context,
        radius: Radius,
        size: impl Into<Length> + Copy,
    ) -> Element<'_, AppEvent> {
        match &self.avatar {
            Some(avatar) => {
                let radius = Radius {
                    top_left: radius.bottom_right,
                    top_right: radius.bottom_left,
                    bottom_right: radius.top_left,
                    bottom_left: radius.top_right,
                };

                Image::new(avatar)
                    .width(size)
                    .height(size)
                    .border_radius(radius)
                    .into()
            }
            None => {
                let placeholder = context.theme.guilds.placeholder_background;
                let text = text(&self.initials)
                    .font(crate::GG_SANS_REGULAR)
                    .color(Color::WHITE);
                container(text)
                    .width(size)
                    .height(size)
                    .align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Center)
                    .style(move |_| container::Style {
                        background: Some(Background::Color(placeholder)),
                        border: Border {
                            radius,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .into()
            }
        }
    }

    pub fn show_clickable_avatar(
        &self,
        context: &Context,
        radius: Radius,
        size: impl Into<Length> + Copy,
    ) -> Element<'_, AppEvent> {
        button(self.show_avatar(context, radius, size))
            .on_press(AppEvent::Message(AppMessage::OpenGuild(self.id)))
            .into()
    }

    pub fn show_pannel<'a>(
        &'a self,
        context: &'a Context,
        panel_width: f32,
    ) -> Element<'a, AppEvent> {
        column([
            container(text(&self.name).size(16.0)).padding(8.0).into(),
            container("")
                .width(Length::Fill)
                .height(context.theme.border_size)
                .style(|_| container::Style::default().background(context.theme.border_color))
                .into(),
            self.channels.show_channels(context),
        ])
        .width(panel_width)
        .into()
    }

    pub fn toggle_category(&mut self, id: u64) -> Option<()> {
        self.channels.toggle_category(id)
    }

    pub fn channel_hover(&mut self, channel_id: u64, hovered: bool) -> Option<()> {
        self.channels.channel_hover(channel_id, hovered)
    }

    pub fn select_channel(&mut self, channel_id: u64) {
        self.channels.select_channel(channel_id);
    }

    pub fn load_messages(
        &mut self,
        channel_id: u64,
        query: MessageQuery,
        messages: Vec<Message>,
    ) -> Option<()> {
        self.channels.load_messages(channel_id, query, messages)
    }

    pub fn show_body(&self, context: &Context) -> Option<Element<'_, AppEvent>> {
        self.channels.show_body(context)
    }
}

fn get_initials(string: &str) -> String {
    string
        .split_whitespace()
        .filter_map(|word| word.chars().next())
        .collect::<String>()
}
