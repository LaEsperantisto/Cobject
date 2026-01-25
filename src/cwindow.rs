use crate::ctitle_bar::CTitleBar;
use crate::{cbutton::CButton, cdrawable::CDrawable, cinput::CInput, cpoint::CPoint};
use minifb::{Window, WindowOptions};
use std::sync::atomic::{AtomicBool, Ordering};

pub static CLOSE_REQUESTED: AtomicBool = AtomicBool::new(false);
pub const TITLE_BAR_HEIGHT: usize = 30;

pub struct CWindow {
    window: Window,
    width: usize,
    height: usize,
    pixels: Vec<u32>,
    pub input: CInput,
    running: bool,
    buttons: Vec<Box<dyn CButton>>,
    objects: Vec<Box<dyn CDrawable>>,
    close_requested: bool,
}

impl CWindow {
    pub fn new(width: usize, height: usize, title: String) -> Self {
        Self {
            window: Window::new(
                (&title).as_ref(),
                width,
                height,
                WindowOptions {
                    resize: true,
                    ..WindowOptions::default()
                },
            )
            .unwrap(),
            width,
            height,
            pixels: vec![0; width * height],
            input: CInput::new(),
            running: true,
            buttons: Vec::new(),
            objects: vec![],
            close_requested: false,
        }
    }
    #[inline(always)]
    pub fn get_width(&self) -> usize {
        self.width
    }
    #[inline(always)]
    pub fn get_height(&self) -> usize {
        self.height
    }
    #[inline(always)]
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
                if button.hitbox().contains_point(point.x, point.y) {
                    button.held();
                    if self.input.mouse_clicked {
                        button.clicked();
                    }
                    if self.input.mouse_released {
                        button.released();
                    }
                }
            }
        }
    }

    #[inline(always)]
    pub fn fill(&mut self, color: u32) {
        self.pixels.fill(color);
    }

    #[inline(always)]
    pub fn draw(&mut self, obj: &dyn CDrawable) {
        obj.draw(&mut self.pixels, self.width, self.height);
    }

    pub fn show_window(&mut self) {
        for object in &self.objects {
            object.draw(&mut self.pixels, self.width, self.height);
        }

        self.window
            .update_with_buffer((&self.pixels).as_ref(), self.width, self.height)
            .unwrap();
    }
    #[inline(always)]
    pub fn close(&mut self) {
        self.running = false;
    }

    pub fn add_button(&mut self, button: Box<dyn CButton>) {
        self.buttons.push(button);
    }

    pub fn request_close(&mut self) {
        self.close_requested = true;
    }

    pub fn init(&mut self) {
        let title = CTitleBar::new("Test".into(), self);
        title.init(self);
        self.objects.push(Box::new(title));
    }

    pub fn add_object(&mut self, object: Box<dyn CDrawable>) {
        self.objects.push(object);
    }

    /*pub fn handle_resize(&mut self) {
        let (new_width, new_height) = self.window.get_size();

        if new_width != self.width || new_height != self.height {
            self.width = new_width;
            self.height = new_height;
            self.pixels.resize(self.width * self.height, ccolor::BLACK);
        }
    }*/
}
