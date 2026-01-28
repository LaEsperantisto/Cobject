use crate::carea::CArea;

pub trait CButton {
    fn clicked(&self) {}
    fn released(&self) {}
    fn held(&self) {}
    fn hitbox(&self) -> &CArea;
}
