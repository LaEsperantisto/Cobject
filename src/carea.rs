use crate::{get_window_height, get_window_width};

#[derive(Debug, Clone)]
pub struct CArea {
    pub x: f32,
    pub y: f32,
    pub width: usize,
    pub height: usize,
}

impl CArea {
    pub fn new(x: f32, y: f32, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub fn contains_point(&self, px: f32, py: f32) -> bool {
        px >= self.x
            && px <= (self.x as usize + self.width) as f32
            && py >= self.y
            && py <= (self.y as usize + self.height) as f32
    }
    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
    pub fn collides(&self, other: &CArea) -> bool {
        self.x <= other.x + other.width as f32
            && self.x + self.width as f32 >= other.x
            && self.y <= other.y + other.height as f32
            && self.y + self.height as f32 >= other.y
    }

    pub fn is_in_window(&self) -> bool {
        self.x > 0.0
            && self.x as usize + self.width < get_window_width()
            && self.y > 0.0
            && self.y as usize + self.height < get_window_height()
    }

    pub fn center_x(&self) -> f32 {
        self.x + self.width as f32 / 2.0
    }

    pub fn center_y(&self) -> f32 {
        self.y + self.height as f32 / 2.0
    }
}

pub fn empty_area() -> CArea {
    CArea::new(0.0, 0.0, 0, 0)
}
