use super::Cache;
use crate::{
    Context,
    app_event::{AppEvent, AppMessage},
    components::button,
};
use iced::{
    Background, Border, Color, Length, Shadow,
    animation::{Animation, Easing},
    border::Radius,
    time::Instant,
    widget::{Button, Container, Svg, button as iced_button, column, container, row},
};
#[derive(Debug)]
pub struct GuildFolder {
    guilds: Vec<u64>,
    id: u64,
    pub is_open: bool,
    animation: Animation<bool>,
}

impl GuildFolder {
    pub fn new(id: u64, guilds: Vec<u64>, context: &Context) -> Self {
        Self {
            id,
            guilds,
            is_open: false,
            animation: Animation::new(false)
                .easing(Easing::EaseInOut)
                .duration(context.theme.guilds.animation_duration),
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

    fn content_height(&self, context: &Context, cache: &Cache) -> f32 {
        let count = self
            .guilds
            .iter()
            .filter(|id| cache.guilds.contains_key(*id))
            .count() as f32;

        context.theme.guilds.size * (count + 1.0)
            + context.theme.guilds.spacing * count
            + context.theme.guilds.folder.padding * 2.0
    }

    fn height(&self, context: &Context, cache: &Cache, now: Instant) -> f32 {
        let target = self.content_height(context, cache);

        self.animation.interpolate(0.0, target, now)
    }

    fn show_icon<'a>(&'a self, context: &'a Context) -> Button<'a, AppEvent> {
        let folder_icon_size = context.theme.guilds.folder.folder_icon_size;
        button(
            Svg::new(context.icons.folder.clone())
                .width(folder_icon_size)
                .height(folder_icon_size),
        )
        .padding(context.theme.guilds.folder.folder_icon_padding)
        .width(Length::Shrink)
        .style(|_, status| iced_button::Style {
            background: if status == iced_button::Status::Hovered {
                Some(Background::Color(
                    context.theme.guilds.folder.active_background,
                ))
            } else {
                None
            },
            border: Border::default().rounded(context.theme.guilds.radius),
            shadow: Shadow::default(),
            text_color: Color::WHITE,
            snap: false,
        })
        .on_press(AppEvent::Message(AppMessage::ToggleFolder(self.id)))
    }

    pub fn show_miniature<'a>(
        &'a self,
        cache: &'a Cache,
        context: &'a Context,
    ) -> Container<'a, AppEvent> {
        let guild = |index: usize, radius: Radius| {
            self.guilds
                .get(index)
                .and_then(|id| cache.guilds.get(id))
                .map(|guild| {
                    guild.show_avatar(
                        context,
                        radius,
                        context.theme.guilds.folder.miniature_guild_size,
                    )
                })
                .unwrap_or_else(|| {
                    container("")
                        .width(context.theme.guilds.folder.miniature_guild_size)
                        .height(context.theme.guilds.folder.miniature_guild_size)
                        .into()
                })
        };

        let radius = Radius::new(context.theme.guilds.folder.small_radius);
        let spacing = context.theme.guilds.folder.miniature_guild_spacing;

        container(if self.is_open {
            self.show_icon(context)
        } else {
            button(
                column![
                    row![
                        guild(0, radius.clone().top_left(context.theme.guilds.radius)),
                        guild(1, radius.clone().top_right(context.theme.guilds.radius))
                    ]
                    .spacing(spacing),
                    row![
                        guild(2, radius.clone().bottom_left(context.theme.guilds.radius)),
                        guild(3, radius.clone().bottom_right(context.theme.guilds.radius))
                    ]
                    .spacing(spacing),
                ]
                .spacing(spacing),
            )
            .padding(0)
            .style(|_, _| iced_button::Style {
                background: None,
                border: Border::default(),
                shadow: Shadow::default(),
                text_color: Color::WHITE,
                snap: false,
            })
            .on_press(AppEvent::Message(AppMessage::ToggleFolder(self.id)))
        })
        .padding(context.theme.guilds.folder.padding)
        .style(|_| {
            let mut style = container::Style::default();

            style.background = Some(Background::from(context.theme.guilds.folder.background));
            style.border.radius = Radius::new(context.theme.guilds.folder.outer_radius);

            style
        })
    }

    pub fn show_opened<'a>(
        &'a self,
        cache: &'a Cache,
        context: &'a Context,
        now: Instant,
    ) -> Container<'a, AppEvent> {
        let col =
            column([self.show_icon(context).into()]).extend(self.guilds.iter().filter_map(|id| {
                Some(cache.guilds.get(id)?.show_clickable_avatar(
                    context,
                    Radius::new(context.theme.guilds.radius),
                    context.theme.guilds.size,
                ))
            }));

        container(
            container(col.spacing(context.theme.guilds.spacing))
                .height(self.content_height(context, cache))
                .padding(context.theme.guilds.folder.padding)
                .style(|_| {
                    container::Style::default()
                        .background(context.theme.guilds.folder.background)
                        .border(Border {
                            radius: Radius::new(context.theme.guilds.folder.outer_radius),
                            ..Default::default()
                        })
                }),
        )
        .height(self.height(context, cache, now))
        .clip(true)
    }
}
