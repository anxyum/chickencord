use super::{Guild, GuildFolder, resizer::ChannelResizeHandle};
use crate::{
    Context, app_event::AppEvent, components::channel::GuildChannel,
    discord_gateway::user_settings::GuildFolders,
};
use iced::{
    Background, Color, Element, Length, Padding,
    alignment::Horizontal,
    animation::{Animation, Easing},
    border::Radius,
    time::Instant,
    widget::{Canvas, Container, Scrollable, column, container, image::Handle, row, scrollable},
};
use std::{collections::HashMap, time::Duration};

#[derive(Debug)]
pub struct Guilds {
    guilds: HashMap<u64, Guild>,
    folders: HashMap<u64, GuildFolder>,
    guild_order: Vec<u64>,
    folder_order: Vec<u64>,
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
            panel: Animation::new(false)
                .easing(Easing::EaseInOut)
                .duration(Duration::from_millis(2000)),
            opened_guild: None,
            channel_panel_width: 192.0,
        }
    }
}

impl Guilds {
    pub fn new(context: &Context) -> Self {
        Self {
            panel: Animation::new(false)
                .easing(Easing::EaseInOut)
                .duration(context.theme.guilds.animation_duration),
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

    fn guilds_preview<'a>(&'a self, context: &'a Context) -> Scrollable<'a, AppEvent> {
        let folders_preview = self.folder_order.iter().filter_map(|id| {
            Some(
                self.folders
                    .get(&id)?
                    .show_miniature(context, &self.guilds)
                    .into(),
            )
        });

        let guilds_preview = self.guild_order.iter().map(|id| {
            self.guilds.get(id).unwrap().show_clickable_avatar(
                context,
                Radius::new(context.theme.guilds.radius),
                context.theme.guilds.size,
            )
        });

        let spacing = context.theme.guilds.spacing;
        scrollable(
            column(folders_preview.chain(guilds_preview))
                .spacing(spacing)
                .padding(Padding::new(0.0).vertical(context.theme.guilds.padding)),
        )
        .height(Length::Fill)
        .style(|context, status| {
            let mut style = scrollable::default(context, status);

            style.vertical_rail.background = None;
            style.vertical_rail.scroller.background = Background::Color(Color::TRANSPARENT);

            style
        })
    }

    fn opened_folders<'a>(
        &'a self,
        context: &'a Context,
        now: Instant,
    ) -> Scrollable<'a, AppEvent> {
        let spacing = context.theme.guilds.spacing;
        scrollable(
            column(self.folder_order.iter().filter_map(|id| {
                let folder = self.folders.get(&id)?;

                if folder.is_visible(now) {
                    Some(folder.show_opened(context, &self.guilds, now).into())
                } else {
                    None
                }
            }))
            .spacing(spacing)
            .padding(Padding::new(0.0).vertical(context.theme.guilds.padding)),
        )
        .width(context.theme.guilds.size + context.theme.guilds.folder.padding * 2.0)
        .height(Length::Fill)
        .style(|context, status| {
            let mut style = scrollable::default(context, status);

            style.vertical_rail.background = None;
            style.vertical_rail.scroller.background = Background::Color(Color::TRANSPARENT);

            style
        })
    }

    pub fn show<'a>(&'a self, context: &'a Context) -> Container<'a, AppEvent> {
        let now = Instant::now();

        let mut content = row![self.guilds_preview(context)]
            .padding(Padding::new(0.0).horizontal(context.theme.guilds.padding))
            .spacing(context.theme.guilds.padding);

        if self.folders.values().any(|f| f.is_visible(now)) {
            let panel_width = self.panel.interpolate(
                0.0,
                context.theme.guilds.size + context.theme.guilds.folder.padding * 2.0,
                now,
            );

            content = content.push(
                container(self.opened_folders(context, now))
                    .width(panel_width)
                    .clip(true)
                    .align_x(Horizontal::Right),
            );
        }

        container(row![
            content,
            container("")
                .width(context.theme.border_size)
                .height(Length::Fill)
                .style(|_| container::Style::default().background(context.theme.border_color))
        ])
        .style(|_| container::Style::default().background(context.theme.guilds.background))
    }

    pub fn reorganize(&mut self, folders: GuildFolders, context: &Context) {
        self.guild_order = folders.guild_positions.clone();
        self.folders = HashMap::new();
        self.folder_order = Vec::new();

        for (i, f) in folders.folders.into_iter().enumerate() {
            let id = f.id.as_ref().map(|v| v.value).unwrap_or(i as i64) as u64;

            self.folders
                .insert(id, GuildFolder::new(id, f.guild_ids, context));

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
        context: &'a Context,
    ) -> Option<Element<'a, AppEvent>> {
        Some(
            self.guilds
                .get(&self.opened_guild?)?
                .show_channels(context, self.channel_panel_width),
        )
    }

    pub fn toggle_category(&mut self, channel_id: u64) -> Option<()> {
        self.guilds
            .get_mut(&self.opened_guild?)?
            .toggle_category(channel_id)
    }

    pub fn channel_hover(&mut self, channel_id: u64, hovered: bool) -> Option<()> {
        self.guilds
            .get_mut(&self.opened_guild?)?
            .channel_hover(channel_id, hovered)
    }

    pub fn select_channel(&mut self, channel_id: u64) -> Option<()> {
        self.guilds
            .get_mut(&self.opened_guild?)?
            .select_channel(channel_id);
        Some(())
    }
}
