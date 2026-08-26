use crate::app_event::{AppEvent, AppMessage};
use iced::{
    Color, Point, Rectangle, Size, mouse,
    widget::canvas::{self, Frame, Geometry},
};

const MIN_WIDTH: f32 = 192.0;
const MAX_WIDTH: f32 = 640.0;
const GRACE_DISTANCE: f32 = 2.0;
const WIDTH: f32 = 4.0;
const THICKNESS: f32 = 1.0;
const HOVER_THICKNESS: f32 = 4.0;
const BORDER_COLOR: Color = Color::from_rgb8(35, 35, 35);

#[derive(Debug)]
pub struct ChannelResizeHandle {
    width: f32,
}

#[derive(Debug, Default)]
pub struct State {
    drag: Option<Drag>,
}

#[derive(Debug)]
struct Drag {
    press_x: f32,
    start_width: f32,
}

impl ChannelResizeHandle {
    pub fn new(width: f32) -> Self {
        Self { width }
    }

    pub fn clamp(width: f32) -> f32 {
        width.clamp(MIN_WIDTH, MAX_WIDTH)
    }
}

impl canvas::Program<AppEvent> for ChannelResizeHandle {
    type State = State;

    fn update(
        &self,
        state: &mut Self::State,
        event: &canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<AppEvent>> {
        match event {
            canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let position = cursor.position()?;

                if !in_bounds(bounds, position) {
                    return None;
                }

                state.drag = Some(Drag {
                    press_x: position.x,
                    start_width: self.width,
                });

                Some(canvas::Action::capture())
            }

            canvas::Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                let drag = state.drag.as_ref()?;
                let position = cursor.position()?;
                let width =
                    ChannelResizeHandle::clamp(drag.start_width + position.x - drag.press_x);

                Some(
                    canvas::Action::publish(AppEvent::Message(AppMessage::ChannelPanelResized(
                        width,
                    )))
                    .and_capture(),
                )
            }

            canvas::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                state.drag.take()?;

                Some(canvas::Action::capture())
            }

            _ => None,
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry<iced::Renderer>> {
        let hovered = cursor.position().is_some_and(|p| in_bounds(bounds, p));

        let thickness = if hovered { HOVER_THICKNESS } else { THICKNESS };

        let mut frame = Frame::new(renderer, bounds.size());

        frame.fill_rectangle(
            Point::new(WIDTH - thickness, 0.0),
            Size::new(thickness, bounds.height),
            BORDER_COLOR,
        );

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if state.drag.is_some() || cursor.position().is_some_and(|p| in_bounds(bounds, p)) {
            return mouse::Interaction::ResizingHorizontally;
        }

        mouse::Interaction::default()
    }
}

fn in_bounds(bounds: Rectangle, point: Point) -> bool {
    let mut bounds = bounds.clone();
    bounds.x -= GRACE_DISTANCE;
    bounds.width += GRACE_DISTANCE * 2.0;
    bounds.contains(point)
}
