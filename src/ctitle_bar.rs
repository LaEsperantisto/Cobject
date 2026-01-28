use crate::{
    get_window_width, CArea, CBasicButton, CDrawable, CRect, CWindow, CLOSE_REQUESTED,
    TITLE_BAR_HEIGHT,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::Ordering;

pub struct CTitleBar {
    _title: String,
    bg: CRect,
}

impl CTitleBar {
    pub fn new(title: String, _window: &CWindow) -> Self {
        Self {
            _title: title,
            bg: CRect::new(0.0, 0.0, get_window_width(), TITLE_BAR_HEIGHT, 0x00FFFFFF),
        }
    }

    pub fn init(&self, window: &mut CWindow) {
        let close_hitbox = CArea::new((get_window_width() - 25) as f32, 3.0, 22, 22);

        let close_button = CBasicButton::new(
            || {
                CLOSE_REQUESTED.store(true, Ordering::Relaxed);
            },
            || {},
            || {},
            close_hitbox,
        );

        window.add_button(Rc::new(RefCell::new(close_button)));
    }

    pub fn update_width(&mut self, width: usize) {
        self.bg.width = width;
    }
}

impl CDrawable for CTitleBar {
    fn draw(&self, pixels: &mut Vec<u32>, width: usize, height: usize) {
        // draw background
        self.bg.draw(pixels, width, height);

        // draw close "X"
        let cx = width.saturating_sub(20);
        let cy = 8;
        let size = 10;
        let thickness = 2;

        for i in 0..size {
            for dx in 0..thickness {
                for dy in 0..thickness {
                    // Diagonal \
                    let x1 = cx + i + dx;
                    let y1 = cy + i + dy;

                    // Diagonal /
                    let x2 = cx + size - i + dx;
                    let y2 = cy + i + dy;

                    if x1 < width && y1 < height {
                        pixels[y1 * width + x1] = 0x00FF0000;
                    }

                    if x2 < width && y2 < height {
                        pixels[y2 * width + x2] = 0x00FF0000;
                    }
                }
            }
        }
    }
}
