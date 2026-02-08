use crate::{CArea, CDrawable};
use std::cell::RefCell;
use std::rc::Rc;

pub trait CObject: CDrawable {
    fn update(&mut self, objects: &[Rc<RefCell<dyn CObject>>]);
    fn is_visible(&self) -> bool;
    fn is_pushable(&self) -> bool {
        true
    }
    fn get_hitbox(&self) -> CArea;
    fn get_velocity(&self) -> (f32, f32);
    fn push(&mut self, x_dir: f32, y_dir: f32);
    fn get_weight(&self) -> i64;
    fn get_density(&self) -> f64;
    fn get_horizontal_drag_force(&self) -> f64;
    fn get_volume(&self) -> u64;
    fn get_mass(&self) -> f32 {
        self.get_density() as f32 * self.get_volume() as f32
    }
    fn get_side_face_area(&self) -> f32;

    fn get_terminal_horizontal_velocity(&self) -> f32 {
        ((2.0 * self.get_mass())
            / (AIR_DENSITY
                * self.get_side_face_area() as f64
                * 1.05
                * self.get_horizontal_drag_force()) as f32)
            .sqrt()
    }

    fn get_friction(&self) -> f32 {
        0.1
    }

    fn get_charge(&self) -> f32 {
        0.0
    }
}

pub const GRAVITATIONAL_ACCELERATION: f64 = 9.8;
pub const AIR_DENSITY: f64 = 1.225;
