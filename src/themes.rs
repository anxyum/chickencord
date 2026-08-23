use iced::Color;

#[derive(Debug, Default)]
pub struct AppTheme {
    pub guilds: GuildsTheme,
}

#[derive(Debug)]
pub struct GuildsTheme {
    pub folder: GuildFolderTheme,

    pub padding: f32,
    pub size: f32,
    pub radius: f32,
    pub spacing: f32,

    pub border_size: f32,

    pub background: Color,
    pub border_color: Color,
    pub placeholder_background: Color,
}

impl Default for GuildsTheme {
    fn default() -> Self {
        let size = 40.0;
        let radius = 12.0;

        Self {
            folder: GuildFolderTheme::new(size, radius),

            padding: 16.0,
            size,
            radius,
            spacing: 8.0,

            border_size: 1.0,

            background: Color::BLACK,
            border_color: Color::from_rgb8(35, 35, 35),
            placeholder_background: Color::BLACK,
        }
    }
}

#[derive(Debug)]
pub struct GuildFolderTheme {
    pub padding: f32,

    pub miniature_guild_spacing: f32,
    pub miniature_guild_size: f32,

    pub folder_icon_size: f32,
    pub folder_icon_padding: f32,

    pub outer_radius: f32,
    pub small_radius: f32,

    pub background: Color,
    pub active_background: Color,
}

impl GuildFolderTheme {
    fn new(size: f32, radius: f32) -> Self {
        let padding = 4.0;
        let miniature_guild_spacing = 2.0;

        let folder_icon_size = 18.0;

        Self {
            padding,

            miniature_guild_spacing,
            miniature_guild_size: (size - miniature_guild_spacing) / 2.0,

            folder_icon_size,
            folder_icon_padding: (size - folder_icon_size) / 2.0,

            outer_radius: radius + padding,
            small_radius: 4.0,

            background: Color::from_rgb8(19, 19, 19),
            active_background: Color::from_rgb8(30, 30, 30),
        }
    }
}
