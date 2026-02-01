use crate::{ccolor, CArea, CBox, CDrawable, CInputListener, CKey, CObject, CQuad};
use std::cell::RefCell;
use std::rc::Rc;

pub struct CPlayer {
    body: CBox,
    try_jump: bool,
    jump_charge: f32,
}

impl CPlayer {
    pub fn new(x: f32, y: f32) -> Self {
        let mut this = Self {
            body: CBox::new(x, y, 50.0, 50.0, ccolor::GREEN),
            try_jump: false,
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
        } else if key == CKey::G {
            self.body.face.points[3] = (0.0, 0.0).into();
        } else if key == CKey::Unknown {
            self.body.face = CQuad::new(self.body.x, self.body.y, 50.0, 50.0, ccolor::GREEN);
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
        } else if key == CKey::E {
            self.body.face.rotate(0.05);
        } else if key == CKey::Q {
            self.body.face.rotate(0.05);
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
        if self.try_jump {
            if self.body.on_ground {
                self.push(0.0, -7.0 - self.jump_charge);
                self.jump_charge = 0.0;
            }
        }

        self.body.x_velocity /= 1.05;
        self.jump_charge = (self.jump_charge - 0.05).max(0.0);
    }

    fn is_visible(&self) -> bool {
        true
    }

    fn get_hitbox(&self) -> CArea {
        self.body.get_hitbox()
    }

    fn get_velocity(&self) -> (f32, f32) {
        self.body.get_velocity()
    }

    fn push(&mut self, x_dir: f32, y_dir: f32) {
        self.body.push(x_dir, y_dir);
    }

    fn get_weight(&self) -> i64 {
        let hitbox = self.get_hitbox();

        (hitbox.width() * hitbox.height()) as i64
    }

    fn get_density(&self) -> f64 {
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
