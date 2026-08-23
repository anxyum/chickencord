use super::Guild;
use crate::{
    app_event::{AppEvent, AppMessage},
    themes::GuildsTheme,
};
use iced::{
    Background, Border, Color, Shadow,
    border::Radius,
    widget::{Container, button, column, container, row},
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct GuildFolder {
    guilds: Vec<u64>,
    id: u64,
    pub is_open: bool,
}

impl GuildFolder {
    pub fn new(id: u64, guilds: Vec<u64>) -> Self {
        Self {
            id,
            guilds,
            is_open: false,
        }
    }

    pub fn show_miniature<'a>(
        &'a self,
        theme: &'a GuildsTheme,
        guilds: &'a HashMap<u64, Guild>,
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

        container(
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
                .spacing(spacing)
                .padding(theme.folder.padding),
            )
            .padding(0)
            .style(|_, _| button::Style {
                background: None,
                border: Border::default(),
                shadow: Shadow::default(),
                text_color: Color::WHITE,
                snap: false,
            })
            .on_press(AppEvent::Message(if self.is_open {
                AppMessage::CloseFolder(self.id)
            } else {
                AppMessage::OpenFolder(self.id)
            })),
        )
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
    ) -> Container<'a, AppEvent> {
        container(column(self.guilds.iter().filter_map(|id| {
            Some(
                guilds
                    .get(id)?
                    .show_avatar(theme, Radius::new(theme.radius), theme.size),
            )
        })))
    }
}
