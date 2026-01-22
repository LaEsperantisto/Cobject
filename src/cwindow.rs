use crate::cdrawable::CDrawable;
use crate::ckey::CKey;
use crate::cmouse::CMouse;
use crate::cobject::CObject;
use crate::cpoint::CPoint;
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

pub struct CWindow {
    window: Window,
    width: usize,
    height: usize,
    title: String,
    pixels: Vec<u32>,
}

impl CWindow {
    pub fn new(width: usize, height: usize, title: String) -> Self {
        Self {
            window: Window::new(title.as_str(), width, height, WindowOptions::default()).unwrap(),
            width,
            height,
            title,
            pixels: vec![0u32; width * height],
        }
    }

    pub fn get_size(&mut self) -> (usize, usize) {
        self.window.get_size()
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn set_pixel(&mut self, i: usize, color: u32) {
        self.pixels[i] = color;
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer((&self.pixels).as_ref(), self.width, self.height)
            .unwrap();
    }

    pub fn is_pressed(&self, key: CKey) -> bool {
        self.window.is_key_down(Key::from(key))
    }

    pub fn is_clicked(&self, button: CMouse) -> bool {
        self.window.get_mouse_down(MouseButton::from(button))
    }

    pub fn coord_to_index(x: usize, y: usize, width: usize, _height: usize) -> usize {
        x + y * width
    }

    pub fn draw<T: CDrawable>(&mut self, obj: &T) {
        obj.draw(&mut self.pixels, self.width, self.height);
    }

    pub fn mouse_pos(&self) -> Option<(f32, f32)> {
        self.window.get_mouse_pos(MouseMode::Discard)
    }

    pub fn point_collides(&self, point: &CPoint, obj: &CObject) -> bool {
        if (obj.x < point.x as usize) && (point.x < (obj.x + obj.width) as f32) {
            if (obj.x < point.y as usize && obj.y < obj.y + obj.height) {
                return true;
            }
        }
        false
    }
}
