use super::{Guild, GuildFolder};
use crate::{
    app_event::AppEvent,
    discord_gateway::user_settings::GuildFolders,
    themes::{AppTheme, GuildsTheme},
};
use iced::{
    Background, Color, Length,
    border::Radius,
    widget::{Container, Scrollable, column, container, image::Handle, row, scrollable},
};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Guilds {
    guilds: HashMap<u64, Guild>,
    folders: HashMap<u64, GuildFolder>,
    folder_order: Vec<u64>,
    guild_order: Vec<u64>,
}

impl Guilds {
    pub fn create_guild(&mut self, id: u64, name: String, avatar: Option<Handle>) {
        self.guilds.insert(id, Guild::new(id, name, avatar));
    }

    fn guilds_preview<'a>(&'a self, theme: &'a GuildsTheme) -> Scrollable<'a, AppEvent> {
        let folders_preview = self.folder_order.iter().filter_map(|id| {
            Some(
                self.folders
                    .get(&id)?
                    .show_miniature(theme, &self.guilds)
                    .into(),
            )
        });

        let guilds_preview = self.guild_order.iter().map(|id| {
            self.guilds
                .get(id)
                .unwrap()
                .show_avatar(theme, Radius::new(theme.radius), theme.size)
        });

        let spacing = theme.spacing;
        scrollable(
            column([
                column(folders_preview).spacing(spacing).into(),
                column(guilds_preview).spacing(spacing).into(),
            ])
            .spacing(spacing),
        )
        .height(Length::Fill)
        .style(|theme, status| {
            let mut style = scrollable::default(theme, status);

            style.vertical_rail.background = None;
            style.vertical_rail.scroller.background = Background::Color(Color::TRANSPARENT);

            style
        })
    }

    pub fn show<'a>(&'a self, theme: &'a AppTheme) -> Container<'a, AppEvent> {
        let theme = &theme.guilds;

        container(row![
            row![self.guilds_preview(theme),]
                .padding(theme.padding)
                .spacing(theme.padding),
            container("")
                .width(theme.border_size)
                .height(Length::Fill)
                .style(|_| container::Style::default().background(theme.border_color))
        ])
        .style(|_| container::Style::default().background(theme.background))
    }

    pub fn reorganize(&mut self, folders: &GuildFolders) {
        self.guild_order = folders.guild_positions.clone();
        self.folders = folders
            .folders
            .iter()
            .map(|f| {
                (
                    f.id.as_ref().unwrap().value as u64,
                    GuildFolder::new(f.id.as_ref().unwrap().value as u64, f.guild_ids.clone()),
                )
            })
            .collect();
        self.folder_order = folders
            .folders
            .iter()
            .map(|f| f.id.as_ref().unwrap().value as u64)
            .collect();
    }

    pub fn open_folder(&mut self, id: u64) -> Option<bool> {
        let folder = self.folders.get_mut(&id)?;
        let was_open = folder.is_open;
        folder.is_open = true;
        Some(was_open)
    }

    pub fn close_folder(&mut self, id: u64) -> Option<bool> {
        let folder = self.folders.get_mut(&id)?;
        let was_open = folder.is_open;
        folder.is_open = false;
        Some(was_open)
    }
}
