use crate::cdrawable::CDrawable;
use crate::cpixel::CPixel;
use crate::CRect;
use std::fs::File;
use std::io::Read;

pub struct CImage {
    pixels: Vec<CPixel>,
    x: f32,
    y: f32,
    x_scale: f32,
    y_scale: f32,
}

impl CImage {
    pub fn get_pixels(&self) -> &Vec<CPixel> {
        &self.pixels
    }

    pub fn new(x: f32, y: f32, path: String) -> Option<CImage> {
        let mut file = File::open(path.clone())
            .ok()
            .expect(("File not found at ".to_owned() + path.as_str()).as_str());

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .ok()
            .expect("File read error");

        let mut tokens = Vec::new();
        let mut current_token = String::new();
        for c in contents.chars() {
            if c == ' ' || c == '\n' {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            } else {
                current_token.push(c);
            }
        }

        if !current_token.is_empty() {
            tokens.push(current_token);
        }

        let mut pixels = Vec::new();
        let mut x_cursor = 0;
        let mut y_cursor = 0;

        for token in tokens {
            if token == "EOL" {
                y_cursor += 1;
                x_cursor = 0;
            } else {
                let color = u32::from_str_radix(token.as_str(), 16).ok().expect(
                    ("Color '".to_owned() + token.as_str() + "' Could not be parsed").as_str(),
                );
                pixels.push(CPixel::new(x + x_cursor as f32, y + y_cursor as f32, color));
                x_cursor += 1;
            }
        }

        Some(CImage {
            pixels,
            x,
            y,
            x_scale: 1.0,
            y_scale: 1.0,
        })
    }

    pub fn set_x(&mut self, x: f32) {
        let old_x = self.x;
        self.x = x;
        for pixel in &mut self.pixels {
            pixel.x = old_x + pixel.x;
        }
    }
    pub fn set_y(&mut self, y: f32) {
        let old_y = self.y;
        self.y = y;
        for pixel in &mut self.pixels {
            pixel.y = old_y + pixel.y;
        }
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn scale(&mut self, x: f32, y: f32) {
        self.x_scale = x;
        self.y_scale = y;
    }
}

impl CDrawable for CImage {
    fn draw(&self, pixels: &mut Vec<u32>, width: usize, height: usize) {
        let top_left_x = self.x;
        let top_left_y = self.y;
        for pixel in self.pixels.clone() {
            CRect::new(
                top_left_x + (pixel.x - self.x) * self.x_scale,
                top_left_y + (pixel.y - self.y) * self.y_scale,
                self.x_scale as usize,
                self.y_scale as usize,
                pixel.color,
            )
            .draw(pixels, width, height);
        }
    }
}
