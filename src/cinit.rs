use crate::cwindow::CWindow;

pub trait CInit {
    fn init(&self, window: &mut CWindow);
}
