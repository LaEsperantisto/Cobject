use crate::{CInputListener, CKey};
use minifb::{MouseButton, MouseMode, Window};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub struct CInput {
    pub mouse_pos: Option<(f32, f32)>,
    pub mouse_down: bool,
    pub mouse_clicked: bool,
    pub mouse_released: bool,

    prev_mouse_down: bool,

    keys_down: HashSet<CKey>,
    prev_keys_down: HashSet<CKey>,
    new_keys_down: HashSet<CKey>,
    new_keys_up: HashSet<CKey>,

    listeners: Vec<Rc<RefCell<dyn CInputListener>>>,
}

impl CInput {
    pub fn new() -> Self {
        Self {
            mouse_pos: None,
            mouse_down: false,
            mouse_clicked: false,
            mouse_released: false,
            prev_mouse_down: false,

            keys_down: HashSet::new(),
            prev_keys_down: HashSet::new(),
            new_keys_down: HashSet::new(),
            new_keys_up: HashSet::new(),

            listeners: Vec::new(),
        }
    }

    pub fn add_listener(&mut self, listener: Rc<RefCell<dyn CInputListener>>) {
        self.listeners.push(listener);
    }

    pub fn poll(&mut self, window: &Window) {
        /* ------------ Mouse ------------ */

        let old_pos = self.mouse_pos;
        self.mouse_pos = window.get_mouse_pos(MouseMode::Discard);
        self.mouse_down = window.get_mouse_down(MouseButton::Left);

        self.mouse_clicked = self.mouse_down && !self.prev_mouse_down;
        self.mouse_released = !self.mouse_down && self.prev_mouse_down;

        if let (Some((x, y)), true) = (self.mouse_pos, self.mouse_pos != old_pos) {
            for l in &self.listeners {
                l.borrow_mut().mouse_moved(x, y);
            }
        }

        if self.mouse_clicked {
            for l in &self.listeners {
                let mut l = l.borrow_mut();
                l.mouse_click(MouseButton::Left);
                l.mouse_down(MouseButton::Left);
            }
        }

        if self.mouse_released {
            for l in &self.listeners {
                l.borrow_mut().mouse_released(MouseButton::Left);
            }
        }

        self.prev_mouse_down = self.mouse_down;

        /* ------------ Keyboard ------------ */

        self.keys_down.clear();
        self.new_keys_down.clear();
        self.new_keys_up.clear();

        let keys = window.get_keys();

        for key in &keys {
            let key = CKey::from(*key);
            self.keys_down.insert(key);

            if !self.prev_keys_down.contains(&key) {
                // Key just pressed
                self.new_keys_down.insert(key);
                for l in &self.listeners {
                    l.borrow_mut().key_pressed(key);
                }
            }

            // Key is held down
            for l in &self.listeners {
                l.borrow_mut().key_held(key);
            }
        }

        // Detect released keys
        for key in self.prev_keys_down.iter() {
            if !self.keys_down.contains(&key) {
                self.new_keys_up.insert(*key);
                for l in &self.listeners {
                    l.borrow_mut().key_released(*key);
                }
            }
        }

        // Save state for next frame
        self.prev_keys_down = self.keys_down.clone();
    }
}
