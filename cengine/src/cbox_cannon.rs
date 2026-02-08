use crate::{CArea, CBox, CDrawable, CInputListener, CKey, CObject};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct CBoxCannon {
    pub this_box: CBox,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    box_counter: u32,
}

impl CBoxCannon {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: u32) -> Self {
        Self {
            this_box: CBox::new(x, y, width, height, color),
            x,
            y,
            width,
            height,
            box_counter: 0,
        }
    }
}

impl CDrawable for CBoxCannon {
    fn draw(&self, pixels: &mut Vec<u32>, width: usize, height: usize) {
        self.this_box.draw(pixels, width, height);
    }
}

impl CObject for CBoxCannon {
    fn update(&mut self, objects: &[Rc<RefCell<dyn CObject>>]) {
        self.this_box.update(objects);
        self.box_counter += 1;
        if self.box_counter >= 50000 {
            self.box_counter = 0;
            self.this_box = CBox {
                x: self.x,
                y: self.y,
                x_velocity: self.this_box.x_velocity,
                y_velocity: self.this_box.y_velocity,
                confined_to_window: self.this_box.confined_to_window,
                bounciness: self.this_box.bounciness,
                can_be_pushed: self.this_box.can_be_pushed,
                face: self.this_box.face.clone(),
                relative_gravity: self.this_box.relative_gravity,
                is_visible: self.this_box.is_visible,
                has_gravity: self.this_box.has_gravity,
                is_solid: self.this_box.is_solid,
                gravity: self.this_box.gravity,
                on_ground: false,
                window_loops: self.this_box.window_loops,
                charge: self.this_box.charge,
            }
        }
    }

    fn is_visible(&self) -> bool {
        self.this_box.is_visible()
    }

    fn get_hitbox(&self) -> CArea {
        self.this_box.get_hitbox()
    }

    fn get_velocity(&self) -> (f32, f32) {
        self.this_box.get_velocity()
    }

    fn push(&mut self, x_dir: f32, y_dir: f32) {
        self.this_box.push(x_dir, y_dir);
    }

    fn get_weight(&self) -> i64 {
        self.this_box.get_weight()
    }

    fn get_density(&self) -> f64 {
        self.this_box.get_density()
    }

    fn get_horizontal_drag_force(&self) -> f64 {
        self.this_box.get_horizontal_drag_force()
    }

    fn get_volume(&self) -> u64 {
        self.this_box.get_volume()
    }

    fn get_side_face_area(&self) -> f32 {
        self.this_box.get_side_face_area()
    }
}

impl CInputListener for CBoxCannon {
    fn key_held(&mut self, key: CKey) {
        if key == CKey::R {
            self.box_counter = 100000;
        }
    }
}
