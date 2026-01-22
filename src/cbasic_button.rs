use crate::{cbutton::CButton, cobject::CObject};

pub struct CBasicButton {
    action: Box<dyn Fn()>,
    hitbox: CObject,
}

impl CBasicButton {
    pub fn new<F: Fn() + 'static>(action: F, hitbox: CObject) -> Self {
        Self {
            action: Box::new(action),
            hitbox,
        }
    }
}

impl CButton for CBasicButton {
    fn clicked(&self) {
        (self.action)();
    }

    fn hitbox(&self) -> &CObject {
        &self.hitbox
    }
}
