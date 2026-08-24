use super::Guild;
use crate::{
    app_event::{AppEvent, AppMessage},
    themes::GuildsTheme,
};
use iced::{
    Background, Border, Color, Length, Shadow,
    border::Radius,
    widget::{Button, Container, Svg, button, column, container, row, svg},
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

    fn show_icon<'a>(
        &'a self,
        theme: &'a GuildsTheme,
        folder_icon: &svg::Handle,
    ) -> Button<'a, AppEvent> {
        button(Svg::new(folder_icon.clone()))
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
            .on_press(AppEvent::Message(AppMessage::CloseFolder(self.id)))
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
            .on_press(AppEvent::Message(AppMessage::OpenFolder(self.id)))
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
    ) -> Container<'a, AppEvent> {
        let col = column([self.show_icon(theme, folder_icon).into()]).extend(
            self.guilds.iter().filter_map(|id| {
                Some(
                    guilds
                        .get(id)?
                        .show_avatar(theme, Radius::new(theme.radius), theme.size),
                )
            }),
        );
        container(col.spacing(theme.spacing))
            .padding(theme.folder.padding)
            .style(|_| {
                container::Style::default()
                    .background(theme.folder.background)
                    .border(Border {
                        radius: Radius::new(theme.folder.outer_radius),
                        ..Default::default()
                    })
            })
    }
}
