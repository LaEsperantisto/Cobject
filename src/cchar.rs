use crate::cdrawable::CDrawable;
use crate::cfont::CFont;
use crate::CPixel;

pub struct CChar {
    c: char,
    font: CFont,
    size: u32,
    x: usize,
    y: usize,
    color: u32,
    bg_color: u32,
}

impl CChar {
    pub fn new(
        c: char,
        font: CFont,
        size: u32,
        x: usize,
        y: usize,
        color: u32,
        bg_color: u32,
    ) -> CChar {
        CChar {
            c,
            font,
            size,
            x,
            y,
            color,
            bg_color,
        }
    }
}

impl CDrawable for CChar {
    fn draw(&self, pixels: &mut Vec<u32>, width: usize, height: usize) {
        let x = self.x;
        let y = self.y;
        let size = self.size;
        let char_pixels =
            self.font
                .get_pixels(self.x, self.y, self.color, self.bg_color, self.c, 1, 1);
        for pixel in char_pixels {
            pixel.draw(pixels, width, height);
        }
    }
}

impl CDrawable for Vec<CPixel> {
    fn draw(&self, pixels: &mut Vec<u32>, width: usize, height: usize) {
        for p in self.iter() {
            p.draw(pixels, width, height);
        }
    }
}
