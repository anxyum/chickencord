use iced::widget::svg::Handle;

const FOLDER_SVG: &[u8] = include_bytes!("../icons/folder.svg");
const VOICE_CHANNEL_SVG: &[u8] = include_bytes!("../icons/voice_channel.svg");
const UNFOLD_CATEGORY_SVG: &[u8] = include_bytes!("../icons/unfold_category.svg");
const TEXT_CHANNEL_SVG: &[u8] = include_bytes!("../icons/text_channel.svg");
const PRIVATE_VOICE_CHANNEL_SVG: &[u8] = include_bytes!("../icons/private_voice_channel.svg");
const PRIVATE_TEXT_CHANNEL_SVG: &[u8] = include_bytes!("../icons/private_text_channel.svg");
const PLUS_SVG: &[u8] = include_bytes!("../icons/plus.svg");

#[derive(Debug, Clone)]
pub struct Icons {
    pub folder: Handle,
    pub voice_channel: Handle,
    pub unfold_category: Handle,
    pub text_channel: Handle,
    pub private_voice_channel: Handle,
    pub private_text_channel: Handle,
    pub plus: Handle,
}

impl Default for Icons {
    fn default() -> Self {
        Self::new()
    }
}

impl Icons {
    pub fn new() -> Self {
        Self {
            folder: Handle::from_memory(FOLDER_SVG),
            voice_channel: Handle::from_memory(VOICE_CHANNEL_SVG),
            unfold_category: Handle::from_memory(UNFOLD_CATEGORY_SVG),
            text_channel: Handle::from_memory(TEXT_CHANNEL_SVG),
            private_voice_channel: Handle::from_memory(PRIVATE_VOICE_CHANNEL_SVG),
            private_text_channel: Handle::from_memory(PRIVATE_TEXT_CHANNEL_SVG),
            plus: Handle::from_memory(PLUS_SVG),
        }
    }
}
