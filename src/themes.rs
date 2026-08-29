use std::time::Duration;

use iced::Color;

#[derive(Debug)]
pub struct AppTheme {
    pub guilds: GuildsTheme,
    pub channels: ChannelsTheme,
    pub messages: MessagesTheme,

    pub border_size: f32,
    pub border_color: Color,
}

impl Default for AppTheme {
    fn default() -> Self {
        Self {
            guilds: Default::default(),
            channels: Default::default(),
            messages: MessagesTheme::default(),

            border_size: 1.0,
            border_color: Color::from_rgb8(35, 35, 35),
        }
    }
}

#[derive(Debug)]
pub struct GuildsTheme {
    pub folder: GuildFolderTheme,

    pub padding: f32,
    pub size: f32,
    pub radius: f32,
    pub spacing: f32,

    pub background: Color,
    pub placeholder_background: Color,

    pub animation_duration: Duration,
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

            background: Color::BLACK,
            placeholder_background: Color::BLACK,

            animation_duration: Duration::from_millis(200),
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

#[derive(Debug)]
pub struct ChannelsTheme {
    pub channel: ChannelTheme,

    pub background: Color,

    pub spacing: f32,
    pub padding: f32,

    pub category_spacing: f32,

    pub scroller_color: Color,
    pub scroller_width: f32,
}

impl Default for ChannelsTheme {
    fn default() -> Self {
        Self {
            channel: ChannelTheme::default(),

            background: Color::BLACK,

            spacing: 2.0,
            padding: 8.0,

            category_spacing: 16.0,

            scroller_color: Color::from_rgb8(128, 128, 128),
            scroller_width: 6.0,
        }
    }
}

#[derive(Debug)]
pub struct ChannelTheme {
    pub background: StateColors,
    pub text: StateColors,
    pub icons: StateColors,

    pub corner_radius: f32,

    pub default_size: f32,
    pub default_text_size: f32,

    pub category_size: f32,
    pub category_text_size: f32,
    pub category_unfold_icon_spacing: f32,

    pub channel_icon_size: f32,
    pub unfold_category_icon_size: f32,
}

impl Default for ChannelTheme {
    fn default() -> Self {
        Self {
            background: StateColors {
                inactive: Color::from_rgba8(150, 150, 150, 0.0),
                hover: Color::from_rgba8(150, 150, 150, 0.125),
                active: Color::from_rgba8(150, 150, 150, 0.25),
            },
            text: StateColors {
                inactive: Color::from_rgba8(127, 127, 127, 1.0),
                hover: Color::from_rgba8(228, 228, 228, 1.0),
                active: Color::from_rgba8(228, 228, 228, 1.0),
            },
            icons: StateColors {
                inactive: Color::from_rgba8(127, 127, 127, 1.0),
                hover: Color::from_rgba8(127, 127, 127, 1.0),
                active: Color::from_rgba8(228, 228, 228, 1.0),
            },

            corner_radius: 8.0,

            default_size: 32.0,
            default_text_size: 16.0,

            category_size: 24.0,
            category_text_size: 14.0,
            category_unfold_icon_spacing: 4.0,

            channel_icon_size: 20.0,
            unfold_category_icon_size: 12.0,
        }
    }
}

#[derive(Debug)]
pub struct StateColors {
    pub inactive: Color,
    pub hover: Color,
    pub active: Color,
}

#[derive(Debug)]
pub struct MessagesTheme {
    pub message: MessageTheme,

    pub background_color: Color,
    pub message_gap: f32, // the gap between groups of messages from different users
}

impl Default for MessagesTheme {
    fn default() -> Self {
        Self {
            message: MessageTheme::default(),
            background_color: Color::BLACK,
            message_gap: 16.0,
        }
    }
}

#[derive(Debug)]
pub struct MessageTheme {
    pub text_color: Color,
    pub time_color: Color,
    pub default_user_name_color: Color,
    pub hover_background_color: Color,

    pub avatar_padding_left: f32,
    pub total_padding_left: f32,
    pub padding_right: f32,
    pub padding_y: f32,

    pub avatar_spacing: f32,
    pub avatar_size: f32,

    pub text_size: f32,
    pub time_size: f32,
    pub time_spacing: f32,

    pub corner_radius: f32,
}

impl Default for MessageTheme {
    fn default() -> Self {
        let avatar_size = 40.0;
        let avatar_padding_left = 16.0;
        let avatar_spacing = 16.0;
        let total_padding_left = avatar_padding_left + avatar_size + avatar_spacing;

        Self {
            text_color: Color::WHITE,
            time_color: Color::from_rgba8(255, 255, 255, 0.5),
            default_user_name_color: Color::WHITE,
            hover_background_color: Color::from_rgba8(128, 128, 128, 0.125),

            avatar_padding_left,
            total_padding_left,
            padding_right: 24.0,
            padding_y: 2.0,

            avatar_spacing,
            avatar_size,

            text_size: 16.0,
            time_size: 12.0,
            time_spacing: 8.0,

            corner_radius: 4.0,
        }
    }
}
