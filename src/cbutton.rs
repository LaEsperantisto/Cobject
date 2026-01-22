use crate::cobject::CObject;

pub trait CButton {
    fn clicked(&self);
    fn hitbox(&self) -> &CObject;
}
