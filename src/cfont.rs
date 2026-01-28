use crate::CPixel;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct CFont {
    tokens: Vec<String>,
    char_width: usize,
}

impl CFont {
    /// Load a font from a file
    pub fn new(path: String) -> Self {
        let file = File::open(&path).unwrap_or_else(|_| panic!("File not found at '{}'", path));
        let reader = BufReader::new(file);

        let mut tokens = Vec::new();

        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            for token in line.split_whitespace() {
                tokens.push(token.to_string());
            }
        }

        // Determine character width from the FIRST character definition
        let mut char_width = 1;
        let mut in_char = false;

        for token in &tokens {
            if token == ":" {
                in_char = true;
                char_width = 1;
                continue;
            }

            if in_char {
                match token.as_str() {
                    "0" | "1" => char_width += 1,
                    "EOL" => break,
                    _ => {}
                }
            }
        }

        CFont { tokens, char_width }
    }

    /// Generate pixels for a given character at (x, y)
    pub fn get_pixels(
        &self,
        x: usize,
        y: usize,
        fg_color: u32,
        bg_color: u32,
        c: char,
        x_scale: usize,
        y_scale: usize,
    ) -> Vec<CPixel> {
        let mut pixels = Vec::new();

        let mut x_cursor = 0;
        let mut y_cursor = 0;

        let mut found_char = false;
        let mut iter = self.tokens.iter().peekable();

        while let Some(token) = iter.next() {
            // Look for character header
            if !found_char {
                if token == ":" {
                    if let Some(next) = iter.peek() {
                        if next.chars().next() == Some(c) {
                            found_char = true;
                            iter.next(); // consume character token
                            x_cursor = 0;
                            y_cursor = 0;
                        }
                    }
                }
                continue;
            }

            match token.as_str() {
                "EOL" => {
                    y_cursor += y_scale;
                    x_cursor = 0;
                }
                "EOC" => break,
                "0" | "1" => {
                    let color = if token == "1" { fg_color } else { bg_color };

                    let base_x = x + x_cursor;
                    let base_y = y + y_cursor;

                    for sy in 0..y_scale {
                        for sx in 0..x_scale {
                            pixels.push(CPixel::new(
                                (base_x + sx) as f32,
                                (base_y + sy) as f32,
                                color,
                            ));
                        }
                    }

                    x_cursor += x_scale;
                }
                _ => {}
            }
        }

        pixels
    }

    pub fn get_char_width(&self) -> usize {
        self.char_width
    }
}

/// Load the default font
pub fn default_font() -> CFont {
    CFont::new("/home/aster/dev/rust/Cobject/cfonts/default.cfont".to_string())
}
