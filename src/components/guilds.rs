use super::Guild;
use crate::app_event::AppEvent;
use iced::{
    Background, Color,
    border::Radius,
    widget::{Row, column, image::Handle, row, scrollable},
};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Guilds {
    guilds: HashMap<u64, Guild>,
}

impl Guilds {
    pub fn create_guild(&mut self, id: u64, name: String, avatar: Option<Handle>) {
        self.guilds.insert(id, Guild::new(id, name, avatar));
    }

    pub fn show(&self) -> Row<'_, AppEvent> {
        row![
            scrollable(self.guilds.values().fold(column![], |column, guild| {
                column.push(guild.show_avatar(Radius::new(12), 40))
            }))
            .style(|theme, status| {
                let mut style = scrollable::default(theme, status);

                style.vertical_rail.background = None;
                style.vertical_rail.scroller.background = Background::Color(Color::TRANSPARENT);

                style
            })
        ]
    }
}
