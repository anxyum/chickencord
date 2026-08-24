use super::{Guild, GuildFolder};
use crate::{
    app_event::AppEvent,
    discord_gateway::user_settings::GuildFolders,
    icons::FOLDER_SVG,
    themes::{AppTheme, GuildsTheme},
};
use iced::{
    Background, Color, Length,
    alignment::Horizontal,
    animation::{Animation, Easing},
    border::Radius,
    time::Instant,
    widget::{Container, Scrollable, column, container, image::Handle, row, scrollable, svg},
};
use std::{collections::HashMap, time::Duration};

#[derive(Debug)]
pub struct Guilds {
    guilds: HashMap<u64, Guild>,
    folders: HashMap<u64, GuildFolder>,
    guild_order: Vec<u64>,
    folder_order: Vec<u64>,
    folder_icon: svg::Handle,
    panel: Animation<bool>,
}

impl Default for Guilds {
    fn default() -> Self {
        Self {
            guilds: Default::default(),
            folders: Default::default(),
            guild_order: Default::default(),
            folder_order: Default::default(),
            folder_icon: svg::Handle::from_memory(FOLDER_SVG),
            panel: Animation::new(false)
                .easing(Easing::EaseInOut)
                .duration(Duration::from_millis(2000)),
        }
    }
}

impl Guilds {
    pub fn new(theme: &GuildsTheme) -> Self {
        Self {
            panel: Animation::new(false)
                .easing(Easing::EaseInOut)
                .duration(theme.animation_duration),
            ..Default::default()
        }
    }

    pub fn create_guild(&mut self, id: u64, name: String, avatar: Option<Handle>) {
        self.guilds.insert(id, Guild::new(id, name, avatar));
    }

    fn guilds_preview<'a>(&'a self, theme: &'a GuildsTheme) -> Scrollable<'a, AppEvent> {
        let folders_preview = self.folder_order.iter().filter_map(|id| {
            Some(
                self.folders
                    .get(&id)?
                    .show_miniature(theme, &self.guilds, &self.folder_icon)
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

    fn opened_folders<'a>(
        &'a self,
        theme: &'a GuildsTheme,
        now: Instant,
    ) -> Scrollable<'a, AppEvent> {
        let spacing = theme.spacing;
        scrollable(
            column(self.folder_order.iter().filter_map(|id| {
                let folder = self.folders.get(&id)?;

                if folder.is_visible(now) {
                    Some(
                        folder
                            .show_opened(theme, &self.guilds, &self.folder_icon, now)
                            .into(),
                    )
                } else {
                    None
                }
            }))
            .spacing(spacing),
        )
        .width(theme.size + theme.folder.padding * 2.0)
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
        let now = Instant::now();

        let mut content = row![self.guilds_preview(theme)]
            .padding(theme.padding)
            .spacing(theme.padding);

        if self.folders.values().any(|f| f.is_visible(now)) {
            let panel_width =
                self.panel
                    .interpolate(0.0, theme.size + theme.folder.padding * 2.0, now);

            content = content.push(
                container(self.opened_folders(theme, now))
                    .width(panel_width)
                    .clip(true)
                    .align_x(Horizontal::Right),
            );
        }

        container(row![
            content,
            container("")
                .width(theme.border_size)
                .height(Length::Fill)
                .style(|_| container::Style::default().background(theme.border_color))
        ])
        .style(|_| container::Style::default().background(theme.background))
    }

    pub fn reorganize(&mut self, folders: &GuildFolders, theme: &GuildsTheme) {
        self.guild_order = folders.guild_positions.clone();
        self.folders = folders
            .folders
            .iter()
            .map(|f| {
                (
                    f.id.as_ref().unwrap().value as u64,
                    GuildFolder::new(
                        f.id.as_ref().unwrap().value as u64,
                        f.guild_ids.clone(),
                        theme,
                    ),
                )
            })
            .collect();
        self.folder_order = folders
            .folders
            .iter()
            .map(|f| f.id.as_ref().unwrap().value as u64)
            .collect();
    }

    pub fn toggle_folder(&mut self, id: u64, now: Instant) {
        if let Some(folder) = self.folders.get_mut(&id) {
            folder.toggle(now);
        }

        let any_open = self.folders.values().any(|f| f.is_open);
        self.panel.go_mut(any_open, now);
    }

    pub fn is_animating(&self) -> bool {
        let now = Instant::now();
        self.panel.is_animating(now) || self.folders.values().any(|f| f.is_animating(now))
    }
}
