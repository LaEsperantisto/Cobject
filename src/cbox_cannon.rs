use crate::{CArea, CBox, CDrawable, CInputListener, CKey, CObject};
use std::cell::RefCell;
use std::rc::Rc;

pub struct CBoxCannon {
    this_box: CBox,
    x: f32,
    y: f32,
    width: usize,
    height: usize,
    box_counter: u32,
}

impl CBoxCannon {
    pub fn new(x: f32, y: f32, width: usize, height: usize, color: u32) -> Self {
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
            self.this_box = CBox::new(
                self.x,
                self.y,
                self.width,
                self.height,
                self.this_box.face.color,
            )
        }
    }

    fn hitbox(&self) -> CArea {
        self.this_box.hitbox()
    }

    fn is_visible(&self) -> bool {
        self.this_box.is_visible()
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

    fn get_density(&self) -> u64 {
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
    fn key_pressed(&mut self, key: CKey) {
        if key == CKey::R {
            self.box_counter = 100000;
        }
    }
}
