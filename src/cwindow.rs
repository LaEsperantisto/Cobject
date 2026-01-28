use crate::ctitle_bar::CTitleBar;
use crate::{
    cbutton::CButton, cdrawable::CDrawable, cinput::CInput, cpoint::CPoint, CInputListener, CObject,
};
use minifb::{Window, WindowOptions};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub static CLOSE_REQUESTED: AtomicBool = AtomicBool::new(false);
pub static WINDOW_HEIGHT: AtomicUsize = AtomicUsize::new(0);
pub static WINDOW_WIDTH: AtomicUsize = AtomicUsize::new(0);
pub const TITLE_BAR_HEIGHT: usize = 30;

pub struct CWindow {
    window: Window,
    width: usize,
    height: usize,
    pixels: Vec<u32>,
    pub input: CInput,
    running: bool,
    buttons: Vec<Rc<RefCell<dyn CButton>>>,
    drawables: Vec<Rc<RefCell<dyn CDrawable>>>,
    objects: Vec<Rc<RefCell<dyn CObject>>>,
}

impl CWindow {
    pub fn new(width: usize, height: usize, title: String) -> Self {
        Self {
            window: Window::new(
                &title,
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
            drawables: Vec::new(),
            objects: Vec::new(),
        }
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

        // --- Button input ---
        if let Some((mx, my)) = self.input.mouse_pos {
            let point = CPoint::new(mx, my);
            if self.input.mouse_down {
                for button in &self.buttons {
                    if button.borrow().hitbox().contains_point(point.x, point.y) {
                        button.borrow().held();
                        if self.input.mouse_clicked {
                            button.borrow().clicked();
                        }
                        if self.input.mouse_released {
                            button.borrow().released();
                        }
                    }
                }
            }
        }

        let len = self.objects.len();

        for i in 0..len {
            let mut object = self.objects[i].borrow_mut();
            let mut objects_without_object = vec![];
            for j in 0..len {
                if j != i {
                    objects_without_object.push(self.objects[j].clone());
                }
            }

            object.update(&objects_without_object);
        }
    }

    pub fn fill(&mut self, color: u32) {
        self.pixels.fill(color);
    }

    pub fn draw(&mut self, obj: &dyn CDrawable) {
        obj.draw(&mut self.pixels, self.width, self.height);
    }

    pub fn show_window(&mut self) {
        for drawable in &self.drawables {
            drawable
                .borrow()
                .draw(&mut self.pixels, self.width, self.height);
        }
        // for object in &self.objects {
        //     object.borrow_mut().update(self.objects.as_slice());
        // }

        self.window
            .update_with_buffer(&self.pixels, self.width, self.height)
            .unwrap();
    }

    pub fn close(&mut self) {
        self.running = false;
    }

    pub fn add_button(&mut self, button: Rc<RefCell<dyn CButton>>) {
        self.buttons.push(button);
    }

    pub fn init(&mut self) {
        set_window_height(self.height);
        set_window_width(self.width);

        let title = CTitleBar::new("Test".into(), self);
        title.init(self);
        self.drawables.push(Rc::new(RefCell::new(title)));
        self.window.set_position(0, 0);
    }

    pub fn add_drawable(&mut self, drawable: Rc<RefCell<dyn CDrawable>>) {
        self.drawables.push(drawable);
    }

    pub fn add_object(&mut self, object: Rc<RefCell<dyn CObject>>) {
        self.drawables.push(object.clone());
        self.objects.push(object);
    }

    pub fn add_input_listener(&mut self, listener: Rc<RefCell<dyn CInputListener>>) {
        self.input.add_listener(listener);
    }
}

pub fn get_window_width() -> usize {
    WINDOW_WIDTH.load(Ordering::Relaxed)
}
pub fn get_window_height() -> usize {
    WINDOW_HEIGHT.load(Ordering::Relaxed)
}

fn set_window_width(width: usize) {
    WINDOW_WIDTH.store(width, Ordering::Relaxed);
}

fn set_window_height(height: usize) {
    WINDOW_HEIGHT.store(height, Ordering::Relaxed);
}
