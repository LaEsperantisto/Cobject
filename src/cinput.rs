use minifb::{MouseButton, MouseMode, Window};

pub struct CInput {
    pub mouse_pos: Option<(f32, f32)>,
    pub mouse_down: bool,
    pub mouse_clicked: bool,
    pub mouse_released: bool,
    pub mouse_movement: Option<(f32, f32)>,
    prev_mouse_down: bool,
}

impl CInput {
    pub fn new() -> Self {
        Self {
            mouse_pos: None,
            mouse_down: false,
            mouse_clicked: false,
            mouse_released: false,
            mouse_movement: None,
            prev_mouse_down: false,
        }
    }

    pub fn poll(&mut self, window: &Window) {
        self.mouse_pos = window.get_mouse_pos(MouseMode::Discard);
        self.mouse_down = window.get_mouse_down(MouseButton::Left);

        self.mouse_clicked = self.mouse_down && !self.prev_mouse_down;
        self.mouse_released = (!self.mouse_down) && self.prev_mouse_down;

        // store current state for next frame
        self.prev_mouse_down = self.mouse_down;
    }
}
