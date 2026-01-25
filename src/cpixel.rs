use crate::cdrawable::CDrawable;
use crate::cutils::coords_to_index;

pub struct CPixel {
    pub x: usize,
    pub y: usize,
    pub color: u32,
}

impl CPixel {
    pub fn new(x: usize, y: usize, color: u32) -> CPixel {
        CPixel { x, y, color }
    }
}

impl CDrawable for CPixel {
    fn draw(&self, pixels: &mut Vec<u32>, width: usize, height: usize) {
        pixels[coords_to_index(self.x as u32, self.y as u32, width)] = self.color;
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
