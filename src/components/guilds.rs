use super::{Guild, GuildFolder, resizer::ChannelResizeHandle};
use crate::{
    app_event::AppEvent,
    components::channel::GuildChannel,
    discord_gateway::user_settings::GuildFolders,
    icons::FOLDER_SVG,
    themes::{AppTheme, GuildsTheme},
};
use iced::{
    Background, Color, Element, Length,
    alignment::Horizontal,
    animation::{Animation, Easing},
    border::Radius,
    time::Instant,
    widget::{
        Canvas, Container, Scrollable, column, container, image::Handle, row, scrollable, svg,
    },
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
    opened_guild: Option<u64>,
    channel_panel_width: f32,
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
            opened_guild: None,
            channel_panel_width: 192.0,
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

    pub fn create_guild(
        &mut self,
        id: u64,
        name: String,
        avatar: Option<Handle>,
        channels: HashMap<u64, GuildChannel>,
    ) {
        self.guilds
            .insert(id, Guild::new(id, name, avatar, channels));
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
            self.guilds.get(id).unwrap().show_clickable_avatar(
                theme,
                Radius::new(theme.radius),
                theme.size,
            )
        });

        let spacing = theme.spacing;
        scrollable(column(folders_preview.chain(guilds_preview)).spacing(spacing))
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
        let now = Instant::now();

        let mut content = row![self.guilds_preview(&theme.guilds)]
            .padding(theme.guilds.padding)
            .spacing(theme.guilds.padding);

        if self.folders.values().any(|f| f.is_visible(now)) {
            let panel_width = self.panel.interpolate(
                0.0,
                theme.guilds.size + theme.guilds.folder.padding * 2.0,
                now,
            );

            content = content.push(
                container(self.opened_folders(&theme.guilds, now))
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
        .style(|_| container::Style::default().background(theme.guilds.background))
    }

    pub fn reorganize(&mut self, folders: GuildFolders, theme: &GuildsTheme) {
        self.guild_order = folders.guild_positions.clone();
        self.folders = HashMap::new();
        self.folder_order = Vec::new();

        for (i, f) in folders.folders.into_iter().enumerate() {
            let id = f.id.as_ref().map(|v| v.value).unwrap_or(i as i64) as u64;

            self.folders
                .insert(id, GuildFolder::new(id, f.guild_ids, theme));

            self.folder_order.push(id);
        }
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

    pub fn open_guild(&mut self, guild_id: u64) {
        self.opened_guild = Some(guild_id)
    }

    pub fn set_channel_panel_width(&mut self, width: f32) {
        self.channel_panel_width = ChannelResizeHandle::clamp(width);
    }

    pub fn channel_resize_divider(&self) -> Canvas<ChannelResizeHandle, AppEvent> {
        Canvas::new(ChannelResizeHandle::new(self.channel_panel_width))
            .width(6.0)
            .height(Length::Fill)
    }

    pub fn show_opened_guild_channels<'a>(
        &'a self,
        theme: &'a AppTheme,
    ) -> Option<Element<'a, AppEvent>> {
        Some(
            self.guilds
                .get(&self.opened_guild?)?
                .show_channels(theme, self.channel_panel_width),
        )
    }

    pub fn toggle_category(&mut self, channel_id: u64) -> Option<()> {
        self.guilds
            .get_mut(&self.opened_guild?)?
            .toggle_category(channel_id)
    }
}
