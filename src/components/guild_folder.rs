use super::Guild;
use crate::{
    app_event::{AppEvent, AppMessage},
    themes::GuildsTheme,
};
use iced::{
    Background, Border, Color, Length, Shadow,
    animation::{Animation, Easing},
    border::Radius,
    time::Instant,
    widget::{Button, Container, Svg, button, column, container, row, svg},
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct GuildFolder {
    guilds: Vec<u64>,
    id: u64,
    pub is_open: bool,
    animation: Animation<bool>,
}

impl GuildFolder {
    pub fn new(id: u64, guilds: Vec<u64>, theme: &GuildsTheme) -> Self {
        Self {
            id,
            guilds,
            is_open: false,
            animation: Animation::new(false)
                .easing(Easing::EaseInOut)
                .duration(theme.animation_duration),
        }
    }

    pub fn toggle(&mut self, now: Instant) {
        self.is_open = !self.is_open;
        self.animation.go_mut(self.is_open, now);
    }

    pub fn is_animating(&self, now: Instant) -> bool {
        self.animation.is_animating(now)
    }

    pub fn is_visible(&self, now: Instant) -> bool {
        self.is_open || self.is_animating(now)
    }

    fn content_height(&self, theme: &GuildsTheme, guilds: &HashMap<u64, Guild>) -> f32 {
        let count = self
            .guilds
            .iter()
            .filter(|id| guilds.contains_key(*id))
            .count() as f32;

        theme.size * (count + 1.0) + theme.spacing * count + theme.folder.padding * 2.0
    }

    fn height(&self, theme: &GuildsTheme, guilds: &HashMap<u64, Guild>, now: Instant) -> f32 {
        let target = self.content_height(theme, guilds);

        self.animation.interpolate(0.0, target, now)
    }

    fn show_icon<'a>(
        &'a self,
        theme: &'a GuildsTheme,
        folder_icon: &svg::Handle,
    ) -> Button<'a, AppEvent> {
        let folder_icon_size = theme.folder.folder_icon_size;
        button(
            Svg::new(folder_icon.clone())
                .width(folder_icon_size)
                .height(folder_icon_size),
        )
        .padding(theme.folder.folder_icon_padding)
        .width(Length::Shrink)
        .style(|_, status| button::Style {
            background: if status == button::Status::Hovered {
                Some(Background::Color(theme.folder.active_background))
            } else {
                None
            },
            border: Border::default().rounded(theme.radius),
            shadow: Shadow::default(),
            text_color: Color::WHITE,
            snap: false,
        })
        .on_press(AppEvent::Message(AppMessage::ToggleFolder(self.id)))
    }

    pub fn show_miniature<'a>(
        &'a self,
        theme: &'a GuildsTheme,
        guilds: &'a HashMap<u64, Guild>,
        folder_icon: &svg::Handle,
    ) -> Container<'a, AppEvent> {
        let guild = |index: usize, radius: Radius| {
            self.guilds
                .get(index)
                .and_then(|id| guilds.get(id))
                .map(|guild| guild.show_avatar(theme, radius, theme.folder.miniature_guild_size))
                .unwrap_or_else(|| {
                    container("")
                        .width(theme.folder.miniature_guild_size)
                        .height(theme.folder.miniature_guild_size)
                        .into()
                })
        };

        let radius = Radius::new(theme.folder.small_radius);
        let spacing = theme.folder.miniature_guild_spacing;

        container(if self.is_open {
            self.show_icon(theme, folder_icon)
        } else {
            button(
                column![
                    row![
                        guild(0, radius.clone().top_left(theme.radius)),
                        guild(1, radius.clone().top_right(theme.radius))
                    ]
                    .spacing(spacing),
                    row![
                        guild(2, radius.clone().bottom_left(theme.radius)),
                        guild(3, radius.clone().bottom_right(theme.radius))
                    ]
                    .spacing(spacing),
                ]
                .spacing(spacing),
            )
            .padding(0)
            .style(|_, _| button::Style {
                background: None,
                border: Border::default(),
                shadow: Shadow::default(),
                text_color: Color::WHITE,
                snap: false,
            })
            .on_press(AppEvent::Message(AppMessage::ToggleFolder(self.id)))
        })
        .padding(theme.folder.padding)
        .style(|_| {
            let mut style = container::Style::default();

            style.background = Some(Background::from(theme.folder.background));
            style.border.radius = Radius::new(theme.folder.outer_radius);

            style
        })
    }

    pub fn show_opened<'a>(
        &'a self,
        theme: &'a GuildsTheme,
        guilds: &'a HashMap<u64, Guild>,
        folder_icon: &svg::Handle,
        now: Instant,
    ) -> Container<'a, AppEvent> {
        let col = column([self.show_icon(theme, folder_icon).into()]).extend(
            self.guilds.iter().filter_map(|id| {
                Some(guilds.get(id)?.show_clickable_avatar(
                    theme,
                    Radius::new(theme.radius),
                    theme.size,
                ))
            }),
        );

        container(
            container(col.spacing(theme.spacing))
                .height(self.content_height(theme, guilds))
                .padding(theme.folder.padding)
                .style(|_| {
                    container::Style::default()
                        .background(theme.folder.background)
                        .border(Border {
                            radius: Radius::new(theme.folder.outer_radius),
                            ..Default::default()
                        })
                }),
        )
        .height(self.height(theme, guilds, now))
        .clip(true)
    }
}
