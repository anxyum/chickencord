use crate::{Context, app_event::AppEvent, components::Cache};
use discord_client_structs::structs::message::Message as GatewayMessage;
use iced::{
    Background, Border, Element, Length, Padding, alignment,
    border::Radius,
    widget::{Image, Text, column, container, image::Handle, row, text},
};
const DISCORD_EPOCH_MS: u64 = 1_424_007_040_000;
static EMPTY_IMAGE: std::sync::LazyLock<Handle> =
    std::sync::LazyLock::new(|| Handle::from_rgba(1, 1, vec![0, 0, 0, 0]));

#[derive(Debug, Clone)]
pub struct Message {
    pub id: u64,
    pub content: Option<String>,
    pub author_id: u64,
}

impl Message {
    pub fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        let timestamp_ms = (self.id >> 22) + DISCORD_EPOCH_MS;

        chrono::DateTime::from_timestamp_millis(timestamp_ms as i64).unwrap_or_default()
    }

    fn author_avatar<'a>(&'a self, context: &'a Context, cache: &'a Cache, guild_id: u64) -> Image {
        let author_member = cache
            .members
            .get(&guild_id)
            .and_then(|members| members.get(&self.author_id));
        let handle = author_member
            .and_then(|member| member.avatar.as_ref())
            .or_else(|| cache.users.get(&self.author_id).map(|user| &user.avatar))
            .unwrap_or(&EMPTY_IMAGE);

        Image::new(handle)
            .width(context.theme.messages.message.avatar_size)
            .height(context.theme.messages.message.avatar_size)
            .border_radius(context.theme.messages.message.avatar_size / 2.0)
    }

    fn text_content(&self, context: &Context) -> Text<'_> {
        text(
            self.content
                .as_ref()
                .map(|v| v.as_str())
                .unwrap_or_default(),
        )
        .size(context.theme.messages.message.text_size)
        .color(context.theme.messages.message.text_color)
    }

    fn full_datetime(&self, context: &Context) -> Text<'_> {
        text(
            self.timestamp()
                .with_timezone(&chrono::Local)
                .format("%d/%m/%Y %H:%M")
                .to_string(),
        )
        .color(context.theme.messages.message.time_color)
        .size(context.theme.messages.message.time_size)
    }

    fn small_datetime(&self, context: &Context) -> Text<'_> {
        text(
            self.timestamp()
                .with_timezone(&chrono::Local)
                .format("%H:%M")
                .to_string(),
        )
        .color(context.theme.messages.message.time_color)
        .size(context.theme.messages.message.time_size)
    }

    pub fn show<'a>(
        &'a self,
        context: &'a Context,
        cache: &'a Cache,
        guild_id: u64,
        hovered: bool,
    ) -> Element<'a, AppEvent> {
        let author_member = cache
            .members
            .get(&guild_id)
            .and_then(|members| members.get(&self.author_id));
        let author_user = cache.users.get(&self.author_id);
        let author_avatar = self.author_avatar(context, cache, guild_id);

        let display_name = author_member
            .and_then(|m| m.nick.as_deref())
            .or_else(|| author_user.map(|u| u.display_name()))
            .unwrap_or_default();

        let messages_theme = &context.theme.messages;
        container(
            row([
                author_avatar.into(),
                column([
                    row([
                        text(display_name)
                            .color(messages_theme.message.default_username_color)
                            .size(messages_theme.message.username_size)
                            .into(),
                        self.full_datetime(context).into(),
                    ])
                    .spacing(messages_theme.message.time_spacing)
                    .align_y(alignment::Vertical::Center)
                    .into(),
                    self.text_content(context).into(),
                ])
                .into(),
            ])
            .padding(
                Padding::new(messages_theme.message.padding_y)
                    .left(messages_theme.message.avatar_padding_left),
            )
            .spacing(messages_theme.message.avatar_spacing),
        )
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: if hovered {
                Some(Background::Color(
                    messages_theme.message.hover_background_color,
                ))
            } else {
                None
            },
            border: Border::default()
                .rounded(Radius::new(0.0).right(messages_theme.message.corner_radius)),
            ..Default::default()
        })
        .into()
    }

    pub fn show_reduced<'a>(
        &'a self,
        context: &'a Context,
        hovered: bool,
    ) -> Element<'a, AppEvent> {
        let messages_theme = &context.theme.messages;

        let row = if hovered {
            row([
                self.small_datetime(context)
                    .width(messages_theme.message.avatar_size)
                    .align_x(alignment::Horizontal::Right)
                    .into(),
                self.text_content(context).into(),
            ])
            .spacing(messages_theme.message.avatar_spacing)
            .padding(
                Padding::new(messages_theme.message.padding_y)
                    .left(messages_theme.message.avatar_padding_left),
            )
        } else {
            row([self.text_content(context).into()]).padding(
                Padding::new(messages_theme.message.padding_y)
                    .left(messages_theme.message.total_padding_left),
            )
        };

        container(row)
            .width(Length::Fill)
            .style(move |_| container::Style {
                background: if hovered {
                    Some(Background::Color(
                        messages_theme.message.hover_background_color,
                    ))
                } else {
                    None
                },
                border: Border::default()
                    .rounded(Radius::new(0.0).right(messages_theme.message.corner_radius)),
                ..Default::default()
            })
            .into()
    }
}

#[derive(Debug)]
pub struct Invalid;

impl TryFrom<GatewayMessage> for Message {
    type Error = Invalid;

    fn try_from(value: GatewayMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            content: value.content,
            author_id: value.author.id,
        })
    }
}
