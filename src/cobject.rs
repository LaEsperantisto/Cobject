use crate::{CArea, CDrawable};
use std::cell::RefCell;
use std::rc::Rc;

pub trait CObject: CDrawable {
    fn update(&mut self, objects: &[Rc<RefCell<dyn CObject>>]);
    fn hitbox(&self) -> CArea;
    fn is_visible(&self) -> bool;
    fn is_pushable(&self) -> bool {
        true
    }
    fn get_velocity(&self) -> (f32, f32);
    fn push(&mut self, x_dir: f32, y_dir: f32);
    fn get_weight(&self) -> i64;
    fn get_density(&self) -> u64;
    fn get_horizontal_drag_force(&self) -> f64;
    fn get_volume(&self) -> u64;
    fn get_mass(&self) -> u64 {
        self.get_density() * self.get_volume()
    }
    fn get_side_face_area(&self) -> f32;

    fn get_terminal_horizontal_velocity(&self) -> f64 {
        ((2 * self.get_mass()) as f64
            / (AIR_DENSITY
                * self.get_side_face_area() as f64
                * 1.05
                * self.get_horizontal_drag_force()))
        .sqrt()
    }
}

pub const GRAVITATIONAL_ACCELERATION: f64 = 9.8;
pub const AIR_DENSITY: f64 = 1.225;
