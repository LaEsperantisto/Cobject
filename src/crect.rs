use crate::cdrawable::CDrawable;
use crate::CArea;

#[derive(Debug, Clone)]
pub struct CRect {
    pub x: f32,
    pub y: f32,
    pub width: usize,
    pub height: usize,
    pub color: u32,
}

impl CRect {
    pub fn new(x: f32, y: f32, width: usize, height: usize, color: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
        }
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

impl CDrawable for CRect {
    fn draw(&self, pixels: &mut Vec<u32>, screen_width: usize, screen_height: usize) {
        let max_x = ((self.x as usize + self.width) as usize).min(screen_width);
        let max_y = ((self.y as usize + self.height) as usize).min(screen_height);

        for y in self.y as usize..max_y {
            let row_start = y * screen_width;
            for x in self.x as usize..max_x {
                pixels[row_start + x] = self.color;
            }
        }
    }
}

impl Into<CArea> for CRect {
    fn into(self) -> CArea {
        CArea::new(self.x, self.y, self.width, self.height)
    }
}
