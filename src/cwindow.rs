use crate::{
    cbutton::CButton, cdrawable::CDrawable, cinput::CInput, cobject::CObject, cpoint::CPoint,
};
use minifb::{Window, WindowOptions};
use std::sync::atomic::{AtomicBool, Ordering};

pub static CLOSE_REQUESTED: AtomicBool = AtomicBool::new(false);

pub struct CWindow {
    window: Window,
    width: usize,
    height: usize,
    pixels: Vec<u32>,
    pub input: CInput,
    running: bool,
    buttons: Vec<Box<dyn CButton>>,
    close_requested: bool,
}

impl CWindow {
    pub fn new(width: usize, height: usize, title: String) -> Self {
        Self {
            window: Window::new((&title).as_ref(), width, height, WindowOptions::default())
                .unwrap(),
            width,
            height,
            pixels: vec![0; width * height],
            input: CInput::new(),
            running: true,
            buttons: Vec::new(),
            close_requested: false,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn is_open(&self) -> bool {
        self.running && self.window.is_open()
    }

    pub fn poll_input(&mut self) {
        self.input.poll(&self.window);
    }

    pub fn update(&mut self) {
        if CLOSE_REQUESTED.swap(false, Ordering::Relaxed) {
            self.running = false;
        }

        let Some((mx, my)) = self.input.mouse_pos else {
            return;
        };
        let point = CPoint::new(mx, my);

        if self.input.mouse_down {
            for button in &self.buttons {
                if CWindow::point_collides(&point, button.hitbox()) {
                    button.clicked();
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.pixels.fill(0x00000000);
    }

    pub fn draw<T: CDrawable>(&mut self, obj: &T) {
        obj.draw(&mut self.pixels, self.width, self.height);
    }

    pub fn draw_window(&mut self) {
        self.window
            .update_with_buffer((&self.pixels).as_ref(), self.width, self.height)
            .unwrap();
    }

    pub fn close(&mut self) {
        self.running = false;
    }

    pub fn add_button(&mut self, button: Box<dyn CButton>) {
        self.buttons.push(button);
    }

    pub fn point_collides(point: &CPoint, obj: &CObject) -> bool {
        point.x >= obj.x as f32
            && point.x <= (obj.x + obj.width) as f32
            && point.y >= obj.y as f32
            && point.y <= (obj.y + obj.height) as f32
    }

    pub fn request_close(&mut self) {
        self.close_requested = true;
    }
}
