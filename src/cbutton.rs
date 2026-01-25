use crate::cobject::CObject;

pub trait CButton {
    fn clicked(&self);
    fn released(&self);
    fn held(&self);
    fn hitbox(&self) -> &CObject;
}
