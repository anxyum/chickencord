use prost::Message;

#[derive(Clone, PartialEq, Message)]
pub struct PreloadedUserSettings {
    #[prost(message, optional, tag = "1")]
    pub versions: Option<Versions>,

    #[prost(message, optional, tag = "2")]
    pub inbox: Option<InboxSettings>,

    #[prost(message, optional, tag = "3")]
    pub guilds: Option<AllGuildSettings>,

    #[prost(message, optional, tag = "4")]
    pub user_content: Option<UserContentSettings>,

    #[prost(message, optional, tag = "5")]
    pub voice_and_video: Option<VoiceAndVideoSettings>,

    #[prost(message, optional, tag = "6")]
    pub text_and_images: Option<TextAndImagesSettings>,

    #[prost(message, optional, tag = "7")]
    pub notifications: Option<NotificationSettings>,

    #[prost(message, optional, tag = "8")]
    pub privacy: Option<PrivacySettings>,

    #[prost(message, optional, tag = "9")]
    pub debug: Option<DebugSettings>,

    #[prost(message, optional, tag = "10")]
    pub game_library: Option<GameLibrarySettings>,

    #[prost(message, optional, tag = "11")]
    pub status: Option<StatusSettings>,

    #[prost(message, optional, tag = "12")]
    pub localization: Option<LocalizationSettings>,

    #[prost(message, optional, tag = "13")]
    pub appearance: Option<AppearanceSettings>,

    #[prost(message, optional, tag = "14")]
    pub guild_folders: Option<GuildFolders>,

    #[prost(message, optional, tag = "15")]
    pub favorites: Option<Favorites>,

    #[prost(message, optional, tag = "16")]
    pub audio_context_settings: Option<AudioSettings>,

    #[prost(message, optional, tag = "17")]
    pub communities: Option<CommunitiesSettings>,

    #[prost(message, optional, tag = "18")]
    pub broadcast: Option<BroadcastSettings>,

    #[prost(message, optional, tag = "19")]
    pub clips: Option<ClipsSettings>,

    #[prost(message, optional, tag = "20")]
    pub for_later: Option<ForLaterSettings>,

    #[prost(message, optional, tag = "21")]
    pub safety_settings: Option<SafetySettings>,

    #[prost(message, optional, tag = "22")]
    pub icymi: Option<IcymiSettings>,

    #[prost(message, optional, tag = "23")]
    pub applications: Option<AllApplicationSettings>,

    #[prost(message, optional, tag = "24")]
    pub ads: Option<AdsSettings>,

    #[prost(message, optional, tag = "25")]
    pub in_app_feedback_settings: Option<InAppFeedbackSettings>,

    #[prost(message, optional, tag = "26")]
    pub app_version_settings: Option<AppVersionSettings>,
}

#[derive(Clone, PartialEq, Message)]
pub struct Versions {
    #[prost(uint32, tag = "1")]
    pub client_version: u32,

    #[prost(uint32, tag = "2")]
    pub server_version: u32,

    #[prost(uint32, tag = "3")]
    pub data_version: u32,
}

#[derive(Clone, PartialEq, Message)]
pub struct InboxSettings {
    #[prost(enumeration = "InboxTab", tag = "1")]
    pub current_tab: i32,

    #[prost(bool, tag = "2")]
    pub viewed_tutorial: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
pub enum InboxTab {
    Unspecified = 0,
    Mentions = 1,
    Unreads = 2,
    Todos = 3,
    ForYou = 4,
    GameInvites = 5,
    Bookmarks = 6,
    Scheduled = 7,
    Reminders = 8,
}

#[derive(Clone, PartialEq, Message)]
pub struct ChannelIconEmoji {
    #[prost(message, optional, tag = "1")]
    pub id: Option<UInt64Value>,

    #[prost(message, optional, tag = "2")]
    pub name: Option<StringValue>,

    #[prost(message, optional, tag = "3")]
    pub color: Option<UInt64Value>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CustomNotificationSoundConfig {
    #[prost(message, optional, tag = "1")]
    pub notification_sound_pack_id: Option<StringValue>,
}

#[derive(Clone, PartialEq, Message)]
pub struct ChannelSettings {
    #[prost(bool, tag = "1")]
    pub collapsed_in_inbox: bool,

    #[prost(message, optional, tag = "2")]
    pub icon_emoji: Option<ChannelIconEmoji>,

    #[prost(message, optional, tag = "3")]
    pub custom_notification_sound_config: Option<CustomNotificationSoundConfig>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CustomCallSound {
    #[prost(fixed64, tag = "1")]
    pub sound_id: u64,

    #[prost(fixed64, tag = "2")]
    pub guild_id: u64,
}

#[derive(Clone, PartialEq, Message)]
pub struct ChannelListSettings {
    #[prost(message, optional, tag = "1")]
    pub layout: Option<StringValue>,

    #[prost(message, optional, tag = "2")]
    pub message_previews: Option<StringValue>,
}

#[derive(Clone, PartialEq, Message)]
pub struct GuildDismissibleContentState {
    #[prost(bool, tag = "1")]
    pub dismissed: bool,

    #[prost(uint32, tag = "2")]
    pub last_dismissed_version: u32,

    #[prost(uint64, tag = "3")]
    pub last_dismissed_at_ms: u64,

    #[prost(uint64, tag = "4")]
    pub last_dismissed_object_id: u64,

    #[prost(uint32, tag = "5")]
    pub num_times_dismissed: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
pub enum GuildThemeSourcePreference {
    Unspecified = 0,
    Personal = 1,
    Guild = 2,
}

#[derive(Clone, PartialEq, Message)]
pub struct GuildSettings {
    #[prost(map = "fixed64, message", tag = "1")]
    pub channels: std::collections::HashMap<u64, ChannelSettings>,

    #[prost(uint32, tag = "2")]
    pub hub_progress: u32,

    #[prost(uint32, tag = "3")]
    pub guild_onboarding_progress: u32,

    #[prost(message, optional, tag = "4")]
    pub guild_recents_dismissed_at: Option<Timestamp>,

    #[prost(bytes, tag = "5")]
    pub dismissed_guild_content: Vec<u8>,

    #[prost(message, optional, tag = "6")]
    pub join_sound: Option<CustomCallSound>,

    #[prost(message, optional, tag = "7")]
    pub mobile_redesign_channel_list_settings: Option<ChannelListSettings>,

    #[prost(bool, tag = "8")]
    pub disable_raid_alert_push: bool,

    #[prost(bool, tag = "9")]
    pub disable_raid_alert_nag: bool,

    #[prost(message, optional, tag = "10")]
    pub custom_notification_sound_config: Option<CustomNotificationSoundConfig>,

    #[prost(bool, tag = "11")]
    pub leaderboards_disabled: bool,

    #[prost(map = "int32, message", tag = "12")]
    pub guild_dismissible_content_states:
        std::collections::HashMap<i32, GuildDismissibleContentState>,

    #[prost(enumeration = "GuildThemeSourcePreference", tag = "13")]
    pub guild_theme_source_preference: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct AllGuildSettings {
    #[prost(map = "fixed64, message", tag = "1")]
    pub guilds: std::collections::HashMap<u64, GuildSettings>,
}

#[derive(Clone, PartialEq, Message)]
pub struct RecurringDismissibleContentState {
    #[prost(uint32, tag = "1")]
    pub last_dismissed_version: u32,

    #[prost(uint64, tag = "2")]
    pub last_dismissed_at_ms: u64,

    #[prost(uint64, tag = "3")]
    pub last_dismissed_object_id: u64,

    #[prost(uint32, tag = "4")]
    pub num_times_dismissed: u32,
}

#[derive(Clone, PartialEq, Message)]
pub struct UserContentSettings {
    #[prost(bytes, tag = "1")]
    pub dismissed_contents: Vec<u8>,

    #[prost(message, optional, tag = "2")]
    pub last_dismissed_outbound_promotion_start_date: Option<StringValue>,

    #[prost(message, optional, tag = "3")]
    pub premium_tier_0_modal_dismissed_at: Option<Timestamp>,

    #[prost(message, optional, tag = "4")]
    pub guild_onboarding_upsell_dismissed_at: Option<Timestamp>,

    #[prost(message, optional, tag = "5")]
    pub safety_user_sentiment_notice_dismissed_at: Option<Timestamp>,

    #[prost(fixed64, tag = "6")]
    pub last_received_changelog_id: u64,

    #[prost(map = "int32, message", tag = "7")]
    pub recurring_dismissible_content_states:
        std::collections::HashMap<i32, RecurringDismissibleContentState>,

    #[prost(fixed64, tag = "8")]
    pub last_gift_intent_dismissed_at_ms: u64,
}

#[derive(Clone, PartialEq, Message)]
pub struct VideoFilterBackgroundBlur {
    #[prost(bool, tag = "1")]
    pub use_blur: bool,
}

#[derive(Clone, PartialEq, Message)]
pub struct VideoFilterAsset {
    #[prost(fixed64, tag = "1")]
    pub id: u64,

    #[prost(string, tag = "2")]
    pub asset_hash: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct SoundboardSettings {
    #[prost(float, tag = "1")]
    pub volume: f32,
}

#[derive(Clone, PartialEq, Message)]
pub struct VoiceAndVideoSettings {
    #[prost(oneof = "VideoBackgroundFilterDesktop", tags = "1, 2, 3")]
    pub video_background_filter_desktop: Option<VideoBackgroundFilterDesktop>,

    #[prost(message, optional, tag = "5")]
    pub always_preview_video: Option<BoolValue>,

    #[prost(message, optional, tag = "6")]
    pub afk_timeout: Option<UInt32Value>,

    #[prost(message, optional, tag = "7")]
    pub stream_notifications_enabled: Option<BoolValue>,

    #[prost(message, optional, tag = "8")]
    pub native_phone_integration_enabled: Option<BoolValue>,

    #[prost(message, optional, tag = "9")]
    pub soundboard_settings: Option<SoundboardSettings>,

    #[prost(message, optional, tag = "10")]
    pub disable_stream_previews: Option<BoolValue>,

    #[prost(message, optional, tag = "11")]
    pub soundmoji_volume: Option<FloatValue>,
}

#[derive(Clone, PartialEq, prost::Oneof)]
pub enum VideoBackgroundFilterDesktop {
    #[prost(message, tag = "1")]
    Blur(VideoFilterBackgroundBlur),

    #[prost(uint32, tag = "2")]
    PresetOption(u32),

    #[prost(message, tag = "3")]
    CustomAsset(VideoFilterAsset),
}

#[derive(Clone, PartialEq, Message)]
pub struct ExplicitContentSettings {
    #[prost(enumeration = "ExplicitContentRedaction", tag = "1")]
    pub explicit_content_guilds: i32,

    #[prost(enumeration = "ExplicitContentRedaction", tag = "2")]
    pub explicit_content_friend_dm: i32,

    #[prost(enumeration = "ExplicitContentRedaction", tag = "3")]
    pub explicit_content_non_friend_dm: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
pub enum ExplicitContentRedaction {
    Unset = 0,
    Show = 1,
    Blur = 2,
    Block = 3,
}

#[derive(Clone, PartialEq, Message)]
pub struct KeywordFilterSettings {
    #[prost(message, optional, tag = "1")]
    pub profanity: Option<BoolValue>,

    #[prost(message, optional, tag = "2")]
    pub sexual_content: Option<BoolValue>,

    #[prost(message, optional, tag = "3")]
    pub slurs: Option<BoolValue>,
}

#[derive(Clone, PartialEq, Message)]
pub struct GoreContentSettings {
    #[prost(enumeration = "ExplicitContentRedaction", tag = "1")]
    pub gore_content_guilds: i32,

    #[prost(enumeration = "ExplicitContentRedaction", tag = "2")]
    pub gore_content_friend_dm: i32,

    #[prost(enumeration = "ExplicitContentRedaction", tag = "3")]
    pub gore_content_non_friend_dm: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct DefaultReactionEmoji {
    #[prost(message, optional, tag = "1")]
    pub emoji_id: Option<UInt64Value>,

    #[prost(message, optional, tag = "2")]
    pub emoji_name: Option<StringValue>,

    #[prost(message, optional, tag = "3")]
    pub animated: Option<BoolValue>,

    #[prost(message, optional, tag = "4")]
    pub disable_double_tap: Option<BoolValue>,
}

#[derive(Clone, PartialEq, Message)]
pub struct SelfHarmContentSettings {
    #[prost(enumeration = "ExplicitContentRedaction", tag = "1")]
    pub self_harm_content_guilds: i32,

    #[prost(enumeration = "ExplicitContentRedaction", tag = "2")]
    pub self_harm_content_friend_dm: i32,

    #[prost(enumeration = "ExplicitContentRedaction", tag = "3")]
    pub self_harm_content_non_friend_dm: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
pub enum DmSpamFilterV2 {
    DefaultUnset = 0,
    Disabled = 1,
    NonFriends = 2,
    FriendsAndNonFriends = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
pub enum SearchProvider {
    Unset = 0,
    Google = 1,
    Bing = 2,
    Duckduckgo = 3,
    Custom = 4,
}

#[derive(Clone, PartialEq, Message)]
pub struct TextAndImagesSettings {
    #[prost(message, optional, tag = "1")]
    pub diversity_surrogate: Option<StringValue>,

    #[prost(message, optional, tag = "2")]
    pub use_rich_chat_input: Option<BoolValue>,

    #[prost(message, optional, tag = "3")]
    pub use_thread_sidebar: Option<BoolValue>,

    #[prost(message, optional, tag = "4")]
    pub render_spoilers: Option<StringValue>,

    #[prost(string, repeated, tag = "5")]
    pub emoji_picker_collapsed_sections: Vec<String>,

    #[prost(string, repeated, tag = "6")]
    pub sticker_picker_collapsed_sections: Vec<String>,

    #[prost(message, optional, tag = "7")]
    pub view_image_descriptions: Option<BoolValue>,

    #[prost(message, optional, tag = "8")]
    pub show_command_suggestions: Option<BoolValue>,

    #[prost(message, optional, tag = "9")]
    pub inline_attachment_media: Option<BoolValue>,

    #[prost(message, optional, tag = "10")]
    pub inline_embed_media: Option<BoolValue>,

    #[prost(message, optional, tag = "11")]
    pub gif_auto_play: Option<BoolValue>,

    #[prost(message, optional, tag = "12")]
    pub render_embeds: Option<BoolValue>,

    #[prost(message, optional, tag = "13")]
    pub render_reactions: Option<BoolValue>,

    #[prost(message, optional, tag = "14")]
    pub animate_emoji: Option<BoolValue>,

    #[prost(message, optional, tag = "15")]
    pub animate_stickers: Option<UInt32Value>,

    #[prost(message, optional, tag = "16")]
    pub enable_tts_command: Option<BoolValue>,

    #[prost(message, optional, tag = "17")]
    pub message_display_compact: Option<BoolValue>,

    #[prost(message, optional, tag = "19")]
    pub explicit_content_filter: Option<UInt32Value>,

    #[prost(message, optional, tag = "20")]
    pub view_nsfw_guilds: Option<BoolValue>,

    #[prost(message, optional, tag = "21")]
    pub convert_emoticons: Option<BoolValue>,

    #[prost(message, optional, tag = "22")]
    pub expression_suggestions_enabled: Option<BoolValue>,

    #[prost(message, optional, tag = "23")]
    pub view_nsfw_commands: Option<BoolValue>,

    #[prost(message, optional, tag = "24")]
    pub use_legacy_chat_input: Option<BoolValue>,

    #[prost(string, repeated, tag = "25")]
    pub soundboard_picker_collapsed_sections: Vec<String>,

    #[prost(message, optional, tag = "26")]
    pub dm_spam_filter: Option<UInt32Value>,

    #[prost(enumeration = "DmSpamFilterV2", tag = "27")]
    pub dm_spam_filter_v2: i32,

    #[prost(message, optional, tag = "28")]
    pub include_stickers_in_autocomplete: Option<BoolValue>,

    #[prost(message, optional, tag = "29")]
    pub explicit_content_settings: Option<ExplicitContentSettings>,

    #[prost(message, optional, tag = "30")]
    pub keyword_filter_settings: Option<KeywordFilterSettings>,

    #[prost(message, optional, tag = "31")]
    pub include_soundmoji_in_autocomplete: Option<BoolValue>,

    #[prost(message, optional, tag = "32")]
    pub gore_content_settings: Option<GoreContentSettings>,

    #[prost(message, optional, tag = "33")]
    pub default_reaction_emoji: Option<DefaultReactionEmoji>,

    #[prost(message, optional, tag = "34")]
    pub show_mention_suggestions: Option<BoolValue>,

    #[prost(message, optional, tag = "35")]
    pub self_harm_content_settings: Option<SelfHarmContentSettings>,

    #[prost(message, optional, tag = "36")]
    pub is_cross_dm_search_enabled: Option<BoolValue>,

    #[prost(enumeration = "SearchProvider", tag = "37")]
    pub search_provider: i32,

    #[prost(message, optional, tag = "38")]
    pub custom_search_url: Option<StringValue>,

    #[prost(message, optional, tag = "39")]
    pub include_game_mentions_in_autocomplete: Option<BoolValue>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
pub enum ReactionNotificationType {
    NotificationsEnabled = 0,
    OnlyDms = 1,
    NotificationsDisabled = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
pub enum GameActivityNotificationType {
    Unset = 0,
    Disabled = 1,
    Enabled = 2,
    OnlyGamesPlayed = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
pub enum CustomStatusPushNotificationType {
    Unset = 0,
    Enabled = 1,
    Disabled = 2,
}

#[derive(Clone, PartialEq, Message)]
pub struct NotificationSettings {
    #[prost(message, optional, tag = "1")]
    pub show_in_app_notifications: Option<BoolValue>,

    #[prost(message, optional, tag = "2")]
    pub notify_friends_on_go_live: Option<BoolValue>,

    #[prost(fixed64, tag = "3")]
    pub notification_center_acked_before_id: u64,

    #[prost(message, optional, tag = "4")]
    pub enable_burst_reaction_notifications: Option<BoolValue>,

    #[prost(message, optional, tag = "5")]
    pub quiet_mode: Option<BoolValue>,

    #[prost(fixed64, tag = "6")]
    pub focus_mode_expires_at_ms: u64,

    #[prost(enumeration = "ReactionNotificationType", tag = "7")]
    pub reaction_notifications: i32,

    #[prost(enumeration = "GameActivityNotificationType", tag = "8")]
    pub game_activity_notifications: i32,

    #[prost(enumeration = "CustomStatusPushNotificationType", tag = "9")]
    pub custom_status_push_notifications: i32,

    #[prost(message, optional, tag = "10")]
    pub game_activity_exclude_steam_notifications: Option<BoolValue>,

    #[prost(message, optional, tag = "11")]
    pub enable_voice_activity_notifications: Option<BoolValue>,

    #[prost(message, optional, tag = "12")]
    pub enable_friend_online_notifications: Option<BoolValue>,

    #[prost(message, optional, tag = "13")]
    pub enable_user_resurrection_notifications: Option<BoolValue>,

    #[prost(message, optional, tag = "14")]
    pub enable_friend_anniversary_notifications: Option<BoolValue>,

    #[prost(message, optional, tag = "15")]
    pub enable_game_update_notifications: Option<BoolValue>,

    #[prost(message, optional, tag = "16")]
    pub enable_profile_updates_notifications: Option<BoolValue>,

    #[prost(message, optional, tag = "17")]
    pub enable_server_trending_notifications: Option<BoolValue>,

    #[prost(message, optional, tag = "18")]
    pub enable_dm_reply_nudge_reminders: Option<BoolValue>,

    #[prost(message, optional, tag = "19")]
    pub enable_summary_reminder_notifications: Option<BoolValue>,

    #[prost(message, optional, tag = "20")]
    pub enable_gdm_all_reaction_notifications: Option<BoolValue>,

    #[prost(message, optional, tag = "21")]
    pub enable_friend_gaming_activity_notifications: Option<BoolValue>,

    #[prost(message, optional, tag = "22")]
    pub enable_upcoming_server_event_notifications: Option<BoolValue>,

    #[prost(message, optional, tag = "23")]
    pub enable_screen_downtime_schedule_notifications: Option<BoolValue>,

    #[prost(message, optional, tag = "24")]
    pub notify_friends_on_profile_update: Option<BoolValue>,

    #[prost(message, optional, tag = "25")]
    pub notify_friends_on_come_online: Option<BoolValue>,
}

#[derive(Clone, PartialEq, Message)]
pub struct PrivacySettings {
    #[prost(message, optional, tag = "1")]
    pub allow_activity_party_privacy_friends: Option<BoolValue>,

    #[prost(message, optional, tag = "2")]
    pub allow_activity_party_privacy_voice_channel: Option<BoolValue>,

    #[prost(fixed64, repeated, tag = "3")]
    pub restricted_guild_ids: Vec<u64>,

    #[prost(bool, tag = "4")]
    pub default_guilds_restricted: bool,

    #[prost(bool, tag = "7")]
    pub allow_accessibility_detection: bool,

    #[prost(message, optional, tag = "8")]
    pub detect_platform_accounts: Option<BoolValue>,

    #[prost(message, optional, tag = "9")]
    pub passwordless: Option<BoolValue>,

    #[prost(message, optional, tag = "10")]
    pub contact_sync_enabled: Option<BoolValue>,

    #[prost(message, optional, tag = "11")]
    pub friend_source_flags: Option<UInt32Value>,

    #[prost(message, optional, tag = "12")]
    pub friend_discovery_flags: Option<UInt32Value>,

    #[prost(fixed64, repeated, tag = "13")]
    pub activity_restricted_guild_ids: Vec<u64>,

    #[prost(enumeration = "GuildActivityStatusRestrictionDefault", tag = "14")]
    pub default_guilds_activity_restricted: i32,

    #[prost(fixed64, repeated, tag = "15")]
    pub activity_joining_restricted_guild_ids: Vec<u64>,

    #[prost(fixed64, repeated, tag = "16")]
    pub message_request_restricted_guild_ids: Vec<u64>,

    #[prost(message, optional, tag = "17")]
    pub default_message_request_restricted: Option<BoolValue>,

    #[prost(message, optional, tag = "18")]
    pub drops_opted_out: Option<BoolValue>,

    #[prost(message, optional, tag = "19")]
    pub non_spam_retraining_opt_in: Option<BoolValue>,

    #[prost(message, optional, tag = "20")]
    pub family_center_enabled: Option<BoolValue>,

    #[prost(message, optional, tag = "21")]
    pub family_center_enabled_v2: Option<BoolValue>,

    #[prost(message, optional, tag = "22")]
    pub hide_legacy_username: Option<BoolValue>,

    #[prost(message, optional, tag = "23")]
    pub inappropriate_conversation_warnings: Option<BoolValue>,

    #[prost(message, optional, tag = "24")]
    pub recent_games_enabled: Option<BoolValue>,

    #[prost(enumeration = "GuildsLeaderboardOptOutDefault", tag = "25")]
    pub guilds_leaderboard_opt_out_default: i32,

    #[prost(message, optional, tag = "26")]
    pub allow_game_friend_dms_in_discord: Option<BoolValue>,

    #[prost(message, optional, tag = "27")]
    pub default_guilds_restricted_v2: Option<BoolValue>,

    #[prost(enumeration = "SlayerSDKReceiveInGameDMs", tag = "28")]
    pub slayer_sdk_receive_dms_in_game: i32,

    #[prost(enumeration = "GuildActivityStatusRestrictionDefaultV2", tag = "29")]
    pub default_guilds_activity_restricted_v2: i32,

    #[prost(message, optional, tag = "30")]
    pub quests_3p_data_opted_out: Option<BoolValue>,

    #[prost(message, optional, tag = "31")]
    pub show_local_time: Option<BoolValue>,

    #[prost(enumeration = "ProfileVisibility", tag = "32")]
    pub profile_visibility: i32,

    #[prost(message, optional, tag = "33")]
    pub hide_friend_request_notes: Option<BoolValue>,

    #[prost(enumeration = "AdTopic", repeated, tag = "34")]
    pub ad_topic_opt_outs: Vec<i32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
pub enum GuildActivityStatusRestrictionDefault {
    Off = 0,
    OnForLargeGuilds = 1,
    On = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
pub enum GuildsLeaderboardOptOutDefault {
    OffForNewGuilds = 0,
    OnForNewGuilds = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
pub enum SlayerSDKReceiveInGameDMs {
    Unset = 0,
    All = 1,
    UsersWithGame = 2,
    None = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
pub enum GuildActivityStatusRestrictionDefaultV2 {
    Unset = 0,
    Off = 1,
    OnForLargeGuilds = 2,
    On = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
pub enum ProfileVisibility {
    Unset = 0,
    FriendsOnly = 1,
    FriendsAndSmallGuilds = 2,
    FriendsAndAllGuilds = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
pub enum AdTopic {
    Unspecified = 0,
    RealMoneyGaming = 1,
}

#[derive(Clone, PartialEq, Message)]
pub struct DebugSettings {
    #[prost(message, optional, tag = "1")]
    pub rtc_panel_show_voice_states: Option<BoolValue>,
}

#[derive(Clone, PartialEq, Message)]
pub struct GameLibrarySettings {
    #[prost(message, optional, tag = "1")]
    pub install_shortcut_desktop: Option<BoolValue>,

    #[prost(message, optional, tag = "2")]
    pub install_shortcut_start_menu: Option<BoolValue>,

    #[prost(message, optional, tag = "3")]
    pub disable_games_tab: Option<BoolValue>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CustomStatus {
    #[prost(string, tag = "1")]
    pub text: String,

    #[prost(fixed64, tag = "2")]
    pub emoji_id: u64,

    #[prost(string, tag = "3")]
    pub emoji_name: String,

    #[prost(fixed64, tag = "4")]
    pub expires_at_ms: u64,

    #[prost(fixed64, tag = "5")]
    pub created_at_ms: u64,

    #[prost(message, optional, tag = "6")]
    pub label: Option<StringValue>,
}

#[derive(Clone, PartialEq, Message)]
pub struct StatusSettings {
    #[prost(message, optional, tag = "1")]
    pub status: Option<StringValue>,

    #[prost(message, optional, tag = "2")]
    pub custom_status: Option<CustomStatus>,

    #[prost(message, optional, tag = "3")]
    pub show_current_game: Option<BoolValue>,

    #[prost(fixed64, tag = "4")]
    pub status_expires_at_ms: u64,

    #[prost(message, optional, tag = "5")]
    pub status_created_at_ms: Option<UInt64Value>,
}

#[derive(Clone, PartialEq, Message)]
pub struct LocalizationSettings {
    #[prost(message, optional, tag = "1")]
    pub locale: Option<StringValue>,

    #[prost(message, optional, tag = "2")]
    pub timezone_offset: Option<Int32Value>,

    #[prost(message, optional, tag = "3")]
    pub timezone_name: Option<StringValue>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
pub enum Theme {
    Unset = 0,
    Dark = 1,
    Light = 2,
    Darker = 3,
    Midnight = 4,
}

#[derive(Clone, PartialEq, Message)]
pub struct CustomUserThemeSettings {
    #[prost(string, repeated, tag = "1")]
    pub colors: Vec<String>,

    #[prost(float, repeated, tag = "2")]
    pub gradient_color_stops: Vec<f32>,

    #[prost(int32, tag = "3")]
    pub gradient_angle: i32,

    #[prost(int32, tag = "4")]
    pub base_mix: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct ClientThemeSettings {
    #[prost(message, optional, tag = "2")]
    pub background_gradient_preset_id: Option<UInt32Value>,

    #[prost(message, optional, tag = "4")]
    pub custom_user_theme_settings: Option<CustomUserThemeSettings>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
pub enum TimestampHourCycle {
    Auto = 0,
    H12 = 1,
    H23 = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
pub enum LaunchPadMode {
    Disabled = 0,
    GestureFullScreen = 1,
    GestureRightEdge = 2,
    PullTab = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
pub enum UIDensity {
    Unset = 0,
    Compact = 1,
    Cozy = 2,
    Responsive = 3,
    Default = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
pub enum SwipeRightToLeftMode {
    Unset = 0,
    ChannelDetails = 1,
    Reply = 2,
}

#[derive(Clone, PartialEq, Message)]
pub struct AppearanceSettings {
    #[prost(enumeration = "Theme", tag = "1")]
    pub theme: i32,

    #[prost(bool, tag = "2")]
    pub developer_mode: bool,

    #[prost(message, optional, tag = "3")]
    pub client_theme_settings: Option<ClientThemeSettings>,

    #[prost(bool, tag = "4")]
    pub mobile_redesign_disabled: bool,

    #[prost(message, optional, tag = "6")]
    pub channel_list_layout: Option<StringValue>,

    #[prost(message, optional, tag = "7")]
    pub message_previews: Option<StringValue>,

    #[prost(message, optional, tag = "8")]
    pub search_result_exact_count_enabled: Option<BoolValue>,

    #[prost(enumeration = "TimestampHourCycle", tag = "9")]
    pub timestamp_hour_cycle: i32,

    #[prost(message, optional, tag = "10")]
    pub happening_now_cards_disabled: Option<BoolValue>,

    #[prost(enumeration = "LaunchPadMode", tag = "11")]
    pub launch_pad_mode: i32,

    #[prost(enumeration = "UIDensity", tag = "12")]
    pub ui_density: i32,

    #[prost(enumeration = "SwipeRightToLeftMode", tag = "13")]
    pub swipe_right_to_left_mode: i32,

    #[prost(enumeration = "GuildThemeSourcePreference", tag = "14")]
    pub default_guild_theme_preference: i32,

    #[prost(bool, tag = "15")]
    pub dark_sidebar: bool,
}

#[derive(Clone, PartialEq, Message)]
pub struct GuildFolder {
    #[prost(fixed64, repeated, tag = "1")]
    pub guild_ids: Vec<u64>,

    #[prost(message, optional, tag = "2")]
    pub id: Option<Int64Value>,

    #[prost(message, optional, tag = "3")]
    pub name: Option<StringValue>,

    #[prost(message, optional, tag = "4")]
    pub color: Option<UInt64Value>,
}

#[derive(Clone, PartialEq, Message)]
pub struct GuildFolders {
    #[prost(message, repeated, tag = "1")]
    pub folders: Vec<GuildFolder>,

    #[prost(fixed64, repeated, tag = "2")]
    pub guild_positions: Vec<u64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
pub enum FavoriteChannelType {
    Unset = 0,
    ReferenceOriginal = 1,
    Category = 2,
}

#[derive(Clone, PartialEq, Message)]
pub struct FavoriteChannel {
    #[prost(string, tag = "1")]
    pub nickname: String,

    #[prost(enumeration = "FavoriteChannelType", tag = "2")]
    pub r#type: i32,

    #[prost(uint32, tag = "3")]
    pub position: u32,

    #[prost(fixed64, tag = "4")]
    pub parent_id: u64,

    #[prost(message, optional, tag = "5")]
    pub channel_type: Option<UInt32Value>,

    #[prost(bool, tag = "6")]
    pub collapsed: bool,
}

#[derive(Clone, PartialEq, Message)]
pub struct Favorites {
    #[prost(map = "fixed64, message", tag = "1")]
    pub favorite_channels: std::collections::HashMap<u64, FavoriteChannel>,

    #[prost(bool, tag = "2")]
    pub muted: bool,

    #[prost(message, optional, tag = "3")]
    pub guild_visible: Option<BoolValue>,
}

#[derive(Clone, PartialEq, Message)]
pub struct AudioContextSetting {
    #[prost(bool, tag = "1")]
    pub muted: bool,

    #[prost(float, tag = "2")]
    pub volume: f32,

    #[prost(fixed64, tag = "3")]
    pub modified_at: u64,

    #[prost(bool, tag = "4")]
    pub soundboard_muted: bool,
}

#[derive(Clone, PartialEq, Message)]
pub struct AudioSettings {
    #[prost(map = "fixed64, message", tag = "1")]
    pub user: std::collections::HashMap<u64, AudioContextSetting>,

    #[prost(map = "fixed64, message", tag = "2")]
    pub stream: std::collections::HashMap<u64, AudioContextSetting>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CommunitiesSettings {
    #[prost(message, optional, tag = "1")]
    pub disable_home_auto_nav: Option<BoolValue>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BroadcastSettings {
    #[prost(message, optional, tag = "1")]
    pub allow_friends: Option<BoolValue>,

    #[prost(fixed64, repeated, tag = "2")]
    pub allowed_guild_ids: Vec<u64>,

    #[prost(fixed64, repeated, tag = "3")]
    pub allowed_user_ids: Vec<u64>,

    #[prost(message, optional, tag = "4")]
    pub auto_broadcast: Option<BoolValue>,
}

#[derive(Clone, PartialEq, Message)]
pub struct ClipsSettings {
    #[prost(message, optional, tag = "1")]
    pub allow_voice_recording: Option<BoolValue>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
pub enum ForLaterTab {
    Unspecified = 0,
    All = 1,
    Bookmarks = 2,
    Reminders = 3,
}

#[derive(Clone, PartialEq, Message)]
pub struct ForLaterSettings {
    #[prost(enumeration = "ForLaterTab", tag = "1")]
    pub current_tab: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
pub enum SafetySettingsPresetType {
    Unset = 0,
    Balanced = 1,
    Strict = 2,
    Relaxed = 3,
    Custom = 4,
}

#[derive(Clone, PartialEq, Message)]
pub struct SpendingLimit {
    #[prost(uint64, tag = "1")]
    pub amount: u64,

    #[prost(string, tag = "2")]
    pub currency: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct SpendingLimitSettings {
    #[prost(message, optional, tag = "1")]
    pub one_time_purchase_limit: Option<SpendingLimit>,
}

#[derive(Clone, PartialEq, Message)]
pub struct SafetySettings {
    #[prost(enumeration = "SafetySettingsPresetType", tag = "1")]
    pub safety_settings_preset: i32,

    #[prost(bool, tag = "2")]
    pub ignore_profile_speedbump_disabled: bool,

    #[prost(message, optional, tag = "3")]
    pub spending_limit_settings: Option<SpendingLimitSettings>,
}

#[derive(Clone, PartialEq, Message)]
pub struct IcymiSettings {
    #[prost(fixed64, tag = "1")]
    pub feed_generated_at: u64,
}

#[derive(Clone, PartialEq, Message)]
pub struct ApplicationDMSettings {
    #[prost(bool, tag = "2")]
    pub allow_mobile_push: bool,
}

#[derive(Clone, PartialEq, Message)]
pub struct ApplicationSharingSettings {
    #[prost(bool, tag = "1")]
    pub disable_application_activity_sharing: bool,
}

#[derive(Clone, PartialEq, Message)]
pub struct ApplicationSettings {
    #[prost(message, optional, tag = "1")]
    pub app_dm_settings: Option<ApplicationDMSettings>,

    #[prost(message, optional, tag = "2")]
    pub app_sharing_settings: Option<ApplicationSharingSettings>,
}

#[derive(Clone, PartialEq, Message)]
pub struct AllApplicationSettings {
    #[prost(map = "fixed64, message", tag = "1")]
    pub app_settings: std::collections::HashMap<u64, ApplicationSettings>,
}

#[derive(Clone, PartialEq, Message)]
pub struct AdsSettings {
    #[prost(bool, tag = "1")]
    pub always_deliver: bool,
}

#[derive(Clone, PartialEq, Message)]
pub struct InAppFeedbackState {
    #[prost(message, optional, tag = "1")]
    pub last_impression_time: Option<UInt64Value>,

    #[prost(message, optional, tag = "2")]
    pub opt_out_expiry_time: Option<UInt64Value>,
}

#[derive(Clone, PartialEq, Message)]
pub struct InAppFeedbackSettings {
    #[prost(map = "int32, message", tag = "1")]
    pub in_app_feedback_states: std::collections::HashMap<i32, InAppFeedbackState>,
}

#[derive(Clone, PartialEq, Message)]
pub struct AppVersionSettings {
    #[prost(bool, tag = "1")]
    pub is_using_outdated_mobile_version: bool,
}

#[derive(Clone, PartialEq, Message)]
pub struct BoolValue {
    #[prost(bool, tag = "1")]
    pub value: bool,
}

#[derive(Clone, PartialEq, Message)]
pub struct UInt32Value {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}

#[derive(Clone, PartialEq, Message)]
pub struct UInt64Value {
    #[prost(uint64, tag = "1")]
    pub value: u64,
}

#[derive(Clone, PartialEq, Message)]
pub struct Int32Value {
    #[prost(int32, tag = "1")]
    pub value: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct Int64Value {
    #[prost(int64, tag = "1")]
    pub value: i64,
}

#[derive(Clone, PartialEq, Message)]
pub struct FloatValue {
    #[prost(float, tag = "1")]
    pub value: f32,
}

#[derive(Clone, PartialEq, Message)]
pub struct StringValue {
    #[prost(string, tag = "1")]
    pub value: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct Timestamp {
    #[prost(int64, tag = "1")]
    pub seconds: i64,

    #[prost(int32, tag = "2")]
    pub nanos: i32,
}
