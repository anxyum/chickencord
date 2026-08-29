use super::{GatewayChannel, GuildChannelBase, Unknown, guild_base};
use crate::{
    Context,
    app_event::{AppEvent, AppMessage},
};
use iced::{
    Length, Padding, alignment,
    mouse::Interaction,
    widget::{Container, Svg, container, mouse_area, row, svg, text},
};
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct Category {
    pub base: GuildChannelBase,
}

impl Category {
    pub fn show(&self, context: &Context, is_open: bool) -> Container<'_, AppEvent> {
        let channel_id = self.base.id;

        let channel_theme = &context.theme.channels.channel;

        let color = if self.base.hovered {
            channel_theme.text.hover
        } else {
            channel_theme.text.inactive
        };

        container(
            mouse_area(
                row([
                    text(&self.base.name)
                        .font(crate::GG_SANS_REGULAR)
                        .size(channel_theme.category_text_size)
                        .color(color)
                        .into(),
                    Svg::new(context.icons.unfold_category.clone())
                        .width(channel_theme.unfold_category_icon_size)
                        .height(channel_theme.unfold_category_icon_size)
                        .rotation(if is_open { 0.0 } else { -PI * 0.5 })
                        .style(move |_, _| svg::Style { color: Some(color) })
                        .into(),
                ])
                .height(channel_theme.category_size)
                .width(Length::Fill)
                .align_y(alignment::Vertical::Center)
                .spacing(channel_theme.category_unfold_icon_spacing),
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
            .on_press(AppEvent::Message(AppMessage::ToggleCategory(channel_id))),
        )
        .padding(Padding::new(0.0).top(context.theme.channels.category_spacing))
    }
}

impl TryFrom<GatewayChannel> for Category {
    type Error = Unknown;

    fn try_from(value: GatewayChannel) -> Result<Self, Self::Error> {
        let flags = value.flags.unwrap_or_default() as u32;

        Ok(Self {
            base: guild_base(
                value.id,
                value.guild_id,
                value.name,
                value.position,
                flags,
                value.parent_id,
            ),
        })
    }
}
