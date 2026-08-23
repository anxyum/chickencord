use super::{Guild, GuildFolder};
use crate::{
    app_event::AppEvent,
    discord_gateway::user_settings::GuildFolders,
};
use iced::{
    Background, Color,
    border::Radius,
    widget::{Row, Scrollable, column, image::Handle, row, scrollable},
};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Guilds {
    guilds: HashMap<u64, Guild>,
    folders: Vec<GuildFolder>,
    orphans: Vec<u64>,
}

impl Guilds {
    pub fn create_guild(&mut self, id: u64, name: String, avatar: Option<Handle>) {
        self.guilds.insert(id, Guild::new(id, name, avatar));
    }

    fn guilds_preview(&self) -> Scrollable<'_, AppEvent> {
        let folders_preview = self
            .folders
            .iter()
            .map(|f| f.show_miniature(&self.guilds));

        let guilds_preview = self.orphans.iter().map(|id| {
            self.guilds
                .get(id)
                .unwrap()
                .show_avatar(Radius::new(12), 40)
        });

        scrollable(column([
            column(folders_preview).into(),
            column(guilds_preview).into(),
        ]))
        .style(|theme, status| {
            let mut style = scrollable::default(theme, status);

            style.vertical_rail.background = None;
            style.vertical_rail.scroller.background = Background::Color(Color::TRANSPARENT);

            style
        })
    }

    pub fn show(&self) -> Row<'_, AppEvent> {
        row![self.guilds_preview()]
    }

    pub fn reorganize(&mut self, folders: &GuildFolders) {
        self.orphans = folders.guild_positions.clone();
        self.folders = folders
            .folders
            .iter()
            .map(|f| GuildFolder::new(f.guild_ids.clone()))
            .collect();
    }
}
