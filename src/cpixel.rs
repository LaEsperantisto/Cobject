use crate::cdrawable::CDrawable;
use crate::cutils::coords_to_index;

pub struct CPixel {
    pub x: f32,
    pub y: f32,
    pub color: u32,
}

impl CPixel {
    pub fn new(x: f32, y: f32, color: u32) -> CPixel {
        CPixel { x, y, color }
    }
}

impl CDrawable for CPixel {
    fn draw(&self, pixels: &mut Vec<u32>, width: usize, _height: usize) {
        pixels[coords_to_index(self.x, self.y, width)] = self.color;
    }
}

impl Clone for CPixel {
    fn clone(&self) -> CPixel {
        CPixel {
            x: self.x,
            y: self.y,
            color: self.color,
        }
    }
}
