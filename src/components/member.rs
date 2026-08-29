use iced::widget::image::Handle;

#[derive(Debug, Clone)]
pub struct Member {
    pub id: u64,
    pub nick: Option<String>,
    pub avatar: Option<Handle>,
}
