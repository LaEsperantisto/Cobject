use crate::CKey;
use minifb::MouseButton;

pub trait CInputListener {
    fn mouse_moved(&mut self, _x: f32, _y: f32) {}
    fn mouse_released(&mut self, _button: MouseButton) {}
    fn mouse_down(&mut self, _button: MouseButton) {}
    fn mouse_click(&mut self, _button: MouseButton) {}

    fn key_pressed(&mut self, _key: CKey) {}
    fn key_held(&mut self, _key: CKey) {}
    fn key_released(&mut self, _key: CKey) {}
}
