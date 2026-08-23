use super::Guild;
use crate::app_event::AppEvent;
use iced::{
    Background, Color, Element,
    border::Radius,
    widget::{column, container, row},
};
use std::collections::HashMap;

const PADDING: f32 = 4.0;
const MINIATURE_GUILD_SPACING: f32 = 2.0;
const MINIATURE_GUILD_SIZE: u32 = 19;

const OUTER_RADIUS: f32 = 16.0;
const SMALL_RADIUS: f32 = 4.0;

const BACKGROUND_COLOR: Color = Color::from_rgb8(19, 19, 19);

#[derive(Debug)]
pub struct GuildFolder {
    guilds: Vec<u64>,
}

impl GuildFolder {
    pub fn new(guilds: Vec<u64>) -> Self {
        Self { guilds }
    }

    pub fn show_miniature<'a>(&'a self, guilds: &'a HashMap<u64, Guild>) -> Element<'a, AppEvent> {
        let guild = |index: usize, radius: Radius| {
            self.guilds
                .get(index)
                .and_then(|id| guilds.get(id))
                .map(|guild| guild.show_avatar(radius, MINIATURE_GUILD_SIZE))
                .unwrap_or_else(|| {
                    container("")
                        .width(MINIATURE_GUILD_SIZE)
                        .height(MINIATURE_GUILD_SIZE)
                        .into()
                })
        };

        let radius = Radius::new(SMALL_RADIUS);
        container(
            column![
                row![
                    guild(0, radius.clone().bottom_right(OUTER_RADIUS)),
                    guild(1, radius.clone().bottom_left(OUTER_RADIUS))
                ]
                .spacing(MINIATURE_GUILD_SPACING),
                row![
                    guild(2, radius.clone().top_right(OUTER_RADIUS)),
                    guild(3, radius.clone().top_left(OUTER_RADIUS))
                ]
                .spacing(MINIATURE_GUILD_SPACING),
            ]
            .spacing(MINIATURE_GUILD_SPACING)
            .padding(PADDING),
        )
        .style(|_| {
            let mut style = container::Style::default();

            style.background = Some(Background::from(BACKGROUND_COLOR));
            style.border.radius = Radius::new(OUTER_RADIUS);

            style
        })
        .into()
    }
}
