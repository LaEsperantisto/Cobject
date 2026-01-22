use crate::cdrawable::CDrawable;

#[derive(Debug, Clone)]
pub struct CObject {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub color: u32,
}

impl CObject {
    pub fn new(x: usize, y: usize, width: usize, height: usize, color: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
        }
    }
    pub fn contains_point(&self, px: f32, py: f32) -> bool {
        px >= self.x as f32
            && px <= (self.x + self.width) as f32
            && py >= self.y as f32
            && py <= (self.y + self.height) as f32
    }
}

impl CDrawable for CObject {
    fn draw(&self, pixels: &mut Vec<u32>, screen_width: usize, screen_height: usize) {
        let max_x = (self.x + self.width).min(screen_width);
        let max_y = (self.y + self.height).min(screen_height);

        for y in self.y..max_y {
            let row_start = y * screen_width;
            for x in self.x..max_x {
                pixels[row_start + x] = self.color;
            }
        }
    }
}
