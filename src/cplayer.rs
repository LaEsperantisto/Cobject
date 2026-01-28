use crate::{ccolor, CArea, CBox, CDrawable, CInputListener, CKey, CObject};
use std::cell::RefCell;
use std::rc::Rc;

pub struct CPlayer {
    body: CBox,
    try_jump: bool,
    on_ground: bool,
    jump_charge: f32,
}

impl CPlayer {
    pub fn new(x: f32, y: f32) -> Self {
        let mut this = Self {
            body: CBox::new(x, y, 50, 50, ccolor::GREEN),
            try_jump: false,
            on_ground: false,
            jump_charge: 0.0,
        };
        this.body.confined_to_window = true;

        this
    }
}

impl CInputListener for CPlayer {
    fn key_pressed(&mut self, key: CKey) {
        if key == CKey::Space {
            self.try_jump = true;
        } else if key == CKey::W {
            self.body.gravity = 0.2;
        }
    }

    fn key_held(&mut self, key: CKey) {
        if key == CKey::A {
            self.body.push(-0.5, 0.0);
        } else if key == CKey::D {
            self.body.push(0.5, 0.0);
        } else if key == CKey::S {
            self.body.push(0.0, 0.5);
            self.jump_charge = (self.jump_charge + 0.1).min(3.0);
        }
    }

    fn key_released(&mut self, key: CKey) {
        if key == CKey::Space {
            self.try_jump = false;
        } else if key == CKey::W {
            self.body.gravity = 1.0;
        }
    }
}

impl CDrawable for CPlayer {
    fn draw(&self, pixels: &mut Vec<u32>, width: usize, height: usize) {
        self.body.draw(pixels, width, height);
    }
}

impl CObject for CPlayer {
    fn update(&mut self, objects: &[Rc<RefCell<dyn CObject + 'static>>]) {
        self.body.update(objects);
        self.on_ground = false;
        for object in objects {
            if object.borrow().hitbox().collides(&self.hitbox())
                && &self.body.collision_normal(&object.borrow().hitbox()) == &(0.0, -1.0)
            {
                self.on_ground = true;
            }
        }
        if self.try_jump {
            if self.on_ground {
                self.push(0.0, -7.0 - self.jump_charge);
                self.jump_charge = 0.0;
            }
        }
        self.body.face.width =
            ((50.0 - (self.jump_charge * 10.0)) * (1.0 / self.body.gravity)) as usize;

        self.body.x_velocity /= 1.05;
        self.jump_charge = (self.jump_charge - 0.05).max(0.0);
    }

    fn hitbox(&self) -> CArea {
        self.body.hitbox()
    }

    fn is_visible(&self) -> bool {
        true
    }

    fn get_velocity(&self) -> (f32, f32) {
        self.body.get_velocity()
    }

    fn push(&mut self, x_dir: f32, y_dir: f32) {
        self.body.push(x_dir, y_dir);
    }

    fn get_weight(&self) -> i64 {
        let hitbox = self.hitbox();

        (hitbox.width * hitbox.height) as i64
    }

    fn get_density(&self) -> u64 {
        self.body.get_density()
    }

    fn get_horizontal_drag_force(&self) -> f64 {
        self.body.get_horizontal_drag_force()
    }

    fn get_volume(&self) -> u64 {
        self.body.get_volume()
    }

    fn get_side_face_area(&self) -> f32 {
        self.body.get_side_face_area()
    }
}
