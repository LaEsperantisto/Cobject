use crate::{
    ccolor, empty_area, get_window_height, get_window_width, CArea, CDrawable, CObject, CRect,
};
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;

#[derive(Debug)]
pub struct CBox {
    pub x: f32,
    pub y: f32,
    pub face: CRect,
    pub x_velocity: f32,
    pub y_velocity: f32,
    pub is_visible: bool,
    pub has_gravity: bool,
    pub is_solid: bool,
    pub can_be_pushed: bool,
    pub confined_to_window: bool,
    pub gravity: f32,
    pub on_ground: bool,
}

impl CBox {
    pub fn new(x: f32, y: f32, width: usize, height: usize, color: u32) -> Self {
        Self {
            x,
            y,
            face: CRect::new(x, y, width, height, color),
            x_velocity: 0.0,
            y_velocity: 0.0,
            is_visible: true,
            has_gravity: true,
            is_solid: true,
            can_be_pushed: true,
            confined_to_window: false,
            gravity: 1.0,
            on_ground: false,
        }
    }

    pub fn collision_normal(&self, other: &CArea) -> (f32, f32) {
        let dx = self.hitbox().center_x() - other.center_x();
        let dy = self.hitbox().center_y() - other.center_y();

        let px = (self.hitbox().width as f32 + other.width as f32) / 2.0 - dx.abs();
        let py = (self.hitbox().height as f32 + other.height as f32) / 2.0 - dy.abs();

        if px < py {
            (dx.signum(), 0.0)
        } else {
            (0.0, dy.signum())
        }
        // (0.0, -1.0) == top of self
    }

    fn resolve_push(&mut self, other: &mut dyn CObject, normal: (f32, f32)) {
        if !self.can_be_pushed && !other.is_pushable() {
            return;
        }

        let push_strength = 0.5; // default 0.5

        if self.can_be_pushed && other.is_pushable() {
            self.x_velocity += normal.0 * push_strength;
            self.y_velocity += normal.1 * push_strength;
        }
        if self.can_be_pushed {
            if normal.1 != 0.0 {
                self.x_velocity = (self.x_velocity + other.get_velocity().0) * push_strength / 2.0;
            }
        }
        if other.is_pushable() {
            other.push(-normal.0 * push_strength, -normal.1 * push_strength);
            if normal.1 != 0.0 {
                other.push(
                    (self.x_velocity + other.get_velocity().0) * push_strength / 1.0
                        - other.get_velocity().0,
                    0.0,
                );
            }
        }
    }
}

impl CDrawable for CBox {
    fn draw(&self, pixels: &mut Vec<u32>, width: usize, height: usize) {
        if self.is_visible {
            self.face.draw(pixels, width, height);
        }
    }
}

impl CObject for CBox {
    fn update(&mut self, objects: &[Rc<RefCell<dyn CObject + 'static>>]) {
        self.x += self.x_velocity;
        self.y += self.y_velocity;

        self.face.set_pos(self.x, self.y);

        self.on_ground = false;

        for object in objects {
            let hitbox = object.borrow().hitbox();
            if hitbox.collides(&self.hitbox()) {
                let mut other = object.borrow_mut();

                let normal = self.collision_normal(&hitbox);

                self.face.set_pos(self.x, self.y);

                if self.is_pushable() {
                    if normal.0 < 0.0 {
                        self.x = hitbox.x - self.face.width as f32;
                        self.x_velocity = 0.0;
                    } else if normal.0 > 0.0 {
                        self.x = hitbox.x + hitbox.width as f32;
                        self.x_velocity = 0.0;
                    }

                    if normal.1 != 0.0 {
                        self.y_velocity = 0.0;
                        if normal.1 < 0.0 {
                            self.y = hitbox.y - self.face.height as f32;
                            self.on_ground = true;
                        }
                    }
                }

                if normal.1 == 0.0 {
                    self.resolve_push(&mut *other, normal);
                }
            }
        }

        if self.x + self.face.width as f32 > get_window_width() as f32 {
            self.x = (get_window_width() - self.face.width) as f32;
            self.x_velocity = 0.0;
        } else if self.x < 0.0 {
            self.x = 0.0;
            self.x_velocity = 0.0;
        } else if self.y > get_window_height() as f32 {
            self.y = get_window_height() as f32;
            self.y_velocity = 0.0;
        } else if self.y + (self.face.height as f32) < 0.0 {
            self.y = 0.0;
            self.y_velocity = 0.0;
        }

        if self.has_gravity {
            self.push(0.0, 0.1 * self.gravity);
        }

        self.face.set_pos(self.x, self.y);
    }

    fn hitbox(&self) -> CArea {
        self.is_visible
            .then(|| self.face.clone().into())
            .unwrap_or(empty_area())
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn is_pushable(&self) -> bool {
        self.can_be_pushed
    }

    fn get_velocity(&self) -> (f32, f32) {
        (self.x_velocity, self.y_velocity)
    }

    fn push(&mut self, x_dir: f32, y_dir: f32) {
        let old_x_velocity = self.x_velocity;
        self.x_velocity += x_dir;
        if self.get_terminal_horizontal_velocity() < self.x_velocity as f64 {
            self.x_velocity = old_x_velocity;
        }

        self.y_velocity += y_dir;
    }

    fn get_weight(&self) -> i64 {
        let hitbox = self.hitbox();

        (hitbox.width * hitbox.height) as i64
    }

    fn get_density(&self) -> u64 {
        1000
    }

    fn get_horizontal_drag_force(&self) -> f64 {
        6.0 * PI * 0.000018 * (3.0_f64.sqrt() * ((self.face.width + self.face.height) / 2) as f64)
            / 2.0
            * self.x_velocity as f64
    }

    fn get_volume(&self) -> u64 {
        (self.face.height * self.face.width) as u64
    }

    fn get_side_face_area(&self) -> f32 {
        self.face.height as f32
    }
}

pub fn get_platform(x: f32, y: f32, width: usize, height: usize) -> CBox {
    let mut my_box = CBox::new(x, y, width, height, ccolor::BROWN);
    my_box.is_solid = true;
    my_box.can_be_pushed = false;
    my_box.has_gravity = false;
    my_box
}
