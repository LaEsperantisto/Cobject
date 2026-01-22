use crate::cdrawable::CDrawable;
use crate::cwindow::CWindow;

pub struct CObject {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color: u32,
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
}

impl CDrawable for CObject {
    fn draw(&self, pixels: &mut Vec<u32>, width: usize, height: usize) {
        for x in 0..self.width {
            for y in 0..self.height {
                pixels[CWindow::coord_to_index(x + self.x, y + self.y, width, height)] = self.color;
            }
        }
    }
}
