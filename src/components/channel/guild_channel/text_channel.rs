use super::{GatewayChannel, GuildChannelBase, Unknown, guild_base};
use crate::{
    Context,
    app_event::{AppEvent, AppMessage},
};
use iced::{
    Background, Border, Length, Padding, Shadow, alignment,
    mouse::Interaction,
    widget::{MouseArea, Svg, container, mouse_area, row, svg, text},
};

#[derive(Debug)]
pub struct TextChannel {
    pub base: GuildChannelBase,
    pub kind: TextKind,
    pub topic: Option<String>,
    pub nsfw: bool,
    pub last_message_id: Option<u64>,
    pub rate_limit_per_user: u32,
    pub last_pin_timestamp: Option<String>,
    pub default_auto_archive_duration: u32,
    pub default_thread_rate_limit_per_user: u32,
}

#[repr(u8)]
#[derive(Debug)]
pub enum TextKind {
    Text = 0,
    News = 5,
}

impl TextChannel {
    pub fn show<'a>(
        &'a self,
        context: &'a Context,
        selected_channel: u64,
    ) -> MouseArea<'a, AppEvent> {
        let channel_theme = &context.theme.channels.channel;
        let selected = selected_channel == self.base.id;

        let channel_icon_color = if selected {
            channel_theme.icons.active
        } else if self.base.hovered {
            channel_theme.icons.hover
        } else {
            channel_theme.icons.inactive
        };

        let text_color = if selected {
            channel_theme.text.active
        } else if self.base.hovered {
            channel_theme.text.hover
        } else {
            channel_theme.text.inactive
        };

        let background_color = if selected {
            channel_theme.background.active
        } else if self.base.hovered {
            channel_theme.background.hover
        } else {
            channel_theme.background.inactive
        };

        mouse_area(
            container(
                row([
                    Svg::new(context.icons.text_channel.clone())
                        .height(channel_theme.channel_icon_size)
                        .width(channel_theme.channel_icon_size)
                        .style(move |_, _| svg::Style {
                            color: Some(channel_icon_color),
                        })
                        .into(),
                    text(&self.base.name)
                        .font(crate::GG_SANS_REGULAR)
                        .size(channel_theme.default_text_size)
                        .color(text_color)
                        .into(),
                ])
                .padding(Padding::new(0.0).horizontal(8.0))
                .spacing(8.0)
                .align_y(alignment::Vertical::Center)
                .height(channel_theme.default_size)
                .width(Length::Fill),
            )
            .style(move |_| container::Style {
                text_color: None,
                background: Some(Background::Color(background_color)),
                border: Border::default().rounded(channel_theme.corner_radius),
                shadow: Shadow::default(),
                snap: false,
            }),
        )
        .interaction(Interaction::Pointer)
        .on_enter(AppEvent::Message(AppMessage::ChannelHover(
            self.base.id,
            true,
        )))
        .on_exit(AppEvent::Message(AppMessage::ChannelHover(
            self.base.id,
            false,
        )))
        .on_press(AppEvent::Message(AppMessage::SelectChannel(self.base.id)))
    }
}

impl TryFrom<GatewayChannel> for TextChannel {
    type Error = Unknown;

    fn try_from(value: GatewayChannel) -> Result<Self, Self::Error> {
        let flags = value.flags.unwrap_or_default() as u32;

        let kind = match value.r#type {
            0 => TextKind::Text,
            5 => TextKind::News,
            _ => {
                return Err(Unknown);
            }
        };

        let base = guild_base(
            value.id,
            value.guild_id,
            value.name,
            value.position,
            flags,
            value.parent_id,
        );
        let topic = value.topic;
        let nsfw = value.nsfw.unwrap_or(false);
        let last_message_id = value.last_message_id;
        let rate_limit_per_user = value.rate_limit_per_user.unwrap_or_default();
        let last_pin_timestamp = value.last_pin_timestamp.map(|t| t.to_rfc3339());
        let default_auto_archive_duration = value.default_auto_archive_duration.unwrap_or_default();
        let default_thread_rate_limit_per_user =
            value.default_thread_rate_limit_per_user.unwrap_or_default();

        Ok(Self {
            base,
            kind,
            topic,
            nsfw,
            last_message_id,
            rate_limit_per_user,
            last_pin_timestamp,
            default_auto_archive_duration,
            default_thread_rate_limit_per_user,
        })
    }
}
