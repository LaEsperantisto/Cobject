use crate::{cbutton::CButton, CArea};

pub struct CBasicButton {
    clicked_action: Box<dyn Fn()>,
    held_action: Box<dyn Fn()>,
    released_action: Box<dyn Fn()>,
    pub hitbox: CArea,
}

impl CBasicButton {
    pub fn new<F1: Fn() + 'static, F2: Fn() + 'static, F3: Fn() + 'static>(
        clicked_action: F1,
        held_action: F2,
        released_action: F3,
        hitbox: CArea,
    ) -> Self {
        Self {
            clicked_action: Box::new(clicked_action),
            held_action: Box::new(held_action),
            released_action: Box::new(released_action),
            hitbox,
        }
    }
}

impl CButton for CBasicButton {
    fn clicked(&self) {
        (self.clicked_action)();
    }

    fn released(&self) {
        (self.released_action)();
    }

    fn held(&self) {
        (self.held_action)();
    }

    fn hitbox(&self) -> &CArea {
        &self.hitbox
    }
}
