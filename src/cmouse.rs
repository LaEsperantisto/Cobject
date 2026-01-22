use minifb::MouseButton;

pub enum CMouse {
    Left,
    Middle,
    Right,
}

impl From<CMouse> for MouseButton {
    fn from(button: CMouse) -> minifb::MouseButton {
        match button {
            CMouse::Left => minifb::MouseButton::Left,
            CMouse::Middle => minifb::MouseButton::Middle,
            CMouse::Right => minifb::MouseButton::Right,
        }
    }
}
