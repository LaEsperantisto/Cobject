use crate::ccolor;
use crate::cdrawable::CDrawable;
use crate::cfont::CFont;

pub struct CText {
    pub text: String,
    pub font: CFont,
    pub x: usize,
    pub y: usize,
    pub color: u32,
    x_scale: usize,
    y_scale: usize,
}

impl CText {
    pub fn new(x: usize, y: usize, text: String, font: CFont, color: u32) -> Self {
        Self {
            text,
            font,
            x,
            y,
            color,
            x_scale: 1,
            y_scale: 1,
        }
    }

    pub fn scale(&mut self, x: usize, y: usize) {
        self.x_scale = x;
        self.y_scale = y;
    }
}

impl CDrawable for CText {
    fn draw(&self, pixels: &mut Vec<u32>, width: usize, height: usize) {
        let mut cursor_x = self.x;

        for c in self.text.chars() {
            let glyph = self.font.get_pixels(
                cursor_x,
                self.y,
                self.color,
                ccolor::BLACK,
                c,
                self.x_scale,
                self.y_scale,
            );

            glyph.draw(pixels, width, height);

            cursor_x += self.font.get_char_width() * self.x_scale;
        }
    }
}
