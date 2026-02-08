use crate::{
    ccolor, empty_area, get_window_height, get_window_width, CArea, CDrawable, CObject, CQuad,
};
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct CBox {
    pub x: f32,
    pub y: f32,
    pub face: CQuad,
    pub x_velocity: f32,
    pub y_velocity: f32,
    pub is_visible: bool,
    pub has_gravity: bool,
    pub is_solid: bool,
    pub can_be_pushed: bool,
    pub confined_to_window: bool,
    pub gravity: f32,
    pub on_ground: bool,
    pub bounciness: f32,
    pub relative_gravity: f32,
    pub window_loops: bool,
    pub charge: f32,
}

impl CBox {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: u32) -> Self {
        Self {
            x,
            y,
            face: CQuad::new(x, y, width, height, color),
            x_velocity: 0.0,
            y_velocity: 0.0,
            is_visible: true,
            has_gravity: true,
            is_solid: true,
            can_be_pushed: true,
            confined_to_window: false,
            gravity: 1.0,
            on_ground: false,
            bounciness: 1.0,
            relative_gravity: 0.0,
            window_loops: false,
            charge: 0.0,
        }
    }

    pub fn collision_normal(&self, other: &CArea) -> (f32, f32) {
        let a = self.get_hitbox();

        let dx = a.center_x() - other.center_x();
        let dy = a.center_y() - other.center_y();

        let px = (a.width() + other.width()) * 0.5 - dx.abs();
        let py = (a.height() + other.height()) * 0.5 - dy.abs();

        if px < py {
            (dx.signum(), 0.0)
        } else {
            (0.0, dy.signum())
        }
    }

    pub fn collision_vector(&self, other: &CArea) -> (f32, f32) {
        let a = self.get_hitbox();

        let dx = a.center_x() - other.center_x();
        let dy = a.center_y() - other.center_y();

        let px = (a.width() + other.width()) * 0.5 - dx.abs();
        let py = (a.height() + other.height()) * 0.5 - dy.abs();

        (dx.signum(), dy.signum())
    }

    fn resolve_push(&mut self, other: &mut dyn CObject, normal: (f32, f32)) {
        if !self.can_be_pushed && !other.is_pushable() {
            return;
        }

        let push_strength = 0.5 * self.bounciness;

        if self.can_be_pushed && other.is_pushable() {
            self.x_velocity += normal.0 * push_strength;
            self.y_velocity += normal.1 * push_strength;
            if normal.1 != 0.0 {
                self.x_velocity = (self.x_velocity + other.get_velocity().0) / 2.0;
            }
            other.push(-normal.0 * push_strength, -normal.1 * push_strength);
        }
        if self.can_be_pushed {
            if normal.1 != 0.0 {
                self.x_velocity /= 1.0 + other.get_friction();
            }
        }
    }

    pub fn get_distance(&mut self, other: &mut dyn CObject) -> f32 {
        let x1 = self.x;
        let y1 = self.y;
        let x2 = other.get_hitbox().min_x();
        let y2 = other.get_hitbox().min_y();

        (x1 - x2).hypot(y1 - y2).abs()
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
        self.on_ground = false;

        self.x += self.x_velocity;
        self.y += self.y_velocity;

        self.face.set_pos(self.x, self.y);

        for object in objects {
            let hitbox = object.borrow().get_hitbox();
            if hitbox.collides(&self.get_hitbox()) {
                let mut other = object.borrow_mut();

                let normal = self.collision_normal(&hitbox);

                self.face.set_pos(self.x, self.y);

                if self.is_pushable() {
                    if normal.0 < 0.0 {
                        self.x = hitbox.min_x() - self.face.width();
                        self.x_velocity = 0.0;
                    } else if normal.0 > 0.0 {
                        self.x = hitbox.max_x();
                        self.x_velocity = 0.0;
                    }

                    if normal.1 < 0.0 && self.y_velocity >= 0.0 {
                        self.y = hitbox.min_y() - self.face.height();
                        self.y_velocity = 0.0;
                        self.on_ground = true;
                    } else if normal.1 > 0.0 {
                        self.y = hitbox.max_y();
                        self.y_velocity = 0.0;
                    }

                    self.face.set_pos(self.x, self.y);
                }
                self.resolve_push(&mut *other, normal);
            } else if self.is_pushable() {
                let hitbox = object.borrow().get_hitbox();

                let dx = hitbox.center_x() - self.get_hitbox().center_x();
                let dy = hitbox.center_y() - self.get_hitbox().center_y();

                let dist_sq = dx * dx + dy * dy;
                let dist = dist_sq.sqrt();

                let gravity_force =
                    self.relative_gravity * self.get_mass() * object.borrow().get_mass() / dist_sq;

                let charge_force = self.get_charge() * object.borrow().get_charge() / dist_sq;

                let force = gravity_force * 0.0000001 + charge_force * 100000000000.0;

                let dt = 0.016;

                self.x_velocity += force * dx / dist * dt;
                self.y_velocity += force * dy / dist * dt;
            }
        }

        let area = CArea::from(self.face.clone());

        if area.min_x() < 0.0 {
            if self.window_loops {
                self.x = get_window_width() as f32 - area.width();
            } else {
                self.x_velocity = self.bounciness - 1.0;
                self.x = 0.0;
            }
        } else if area.max_x() > get_window_width() as f32 {
            if self.window_loops {
                self.x = 0.0;
            } else {
                self.x_velocity = -self.bounciness + 1.0;
                self.x = get_window_width() as f32 - area.width();
            }
        }

        if area.max_y() > get_window_height() as f32 {
            if self.window_loops {
                self.y = 0.0;
            } else {
                self.y = get_window_height() as f32 - area.height();
                self.y_velocity = -self.bounciness + 1.0;
            }
            self.on_ground = true;
        } else if area.min_y() < 0.0 {
            if self.window_loops {
                self.y = get_window_height() as f32 - area.height();
            } else {
                self.y = 0.0;
                self.y_velocity = self.bounciness - 1.0;
            }
        }

        if self.has_gravity {
            self.y_velocity += 0.1 * self.gravity;
        }

        self.face.set_pos(self.x, self.y);
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn is_pushable(&self) -> bool {
        self.can_be_pushed
    }

    fn get_hitbox(&self) -> CArea {
        self.is_visible
            .then(|| self.face.clone().into())
            .unwrap_or(empty_area())
    }

    fn get_velocity(&self) -> (f32, f32) {
        (self.x_velocity, self.y_velocity)
    }

    fn push(&mut self, x_dir: f32, y_dir: f32) {
        let old_x_velocity = self.x_velocity;
        self.x_velocity += x_dir;
        if self.get_terminal_horizontal_velocity() < self.x_velocity {
            self.x_velocity = old_x_velocity;
        }

        self.y_velocity += y_dir;
    }

    fn get_weight(&self) -> i64 {
        let hitbox = self.get_hitbox();

        (hitbox.width() * hitbox.height()) as i64
    }

    fn get_density(&self) -> f64 {
        1000.0
    }

    fn get_horizontal_drag_force(&self) -> f64 {
        6.0 * PI
            * 0.000018
            * (3.0_f64.sqrt() * ((self.face.width() + self.face.height()) / 2.0) as f64)
            / 2.0
            * self.x_velocity as f64
    }

    fn get_volume(&self) -> u64 {
        (self.face.height() * self.face.width()) as u64
    }

    fn get_side_face_area(&self) -> f32 {
        self.face.height()
    }

    fn get_charge(&self) -> f32 {
        self.charge
    }
}

pub fn get_platform(x: f32, y: f32, width: f32, height: f32) -> CBox {
    let mut my_box = CBox::new(x, y, width, height, ccolor::BROWN);
    my_box.is_solid = true;
    my_box.can_be_pushed = false;
    my_box.has_gravity = false;
    my_box
}
