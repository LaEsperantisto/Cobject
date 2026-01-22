use minifb::{MouseButton, MouseMode, Window};

pub struct CInput {
    pub mouse_pos: Option<(f32, f32)>,
    pub mouse_down: bool,
}

impl CInput {
    pub fn new() -> Self {
        Self {
            mouse_pos: None,
            mouse_down: false,
        }
    }

    pub fn poll(&mut self, window: &Window) {
        self.mouse_pos = window.get_mouse_pos(MouseMode::Discard);
        self.mouse_down = window.get_mouse_down(MouseButton::Left);
    }
}
