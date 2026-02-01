#[derive(Clone, Debug)]
pub struct CPoint {
    pub x: f32,
    pub y: f32,
}

impl CPoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<(f32, f32)> for CPoint {
    fn from(o: (f32, f32)) -> Self {
        let (x, y) = o;
        Self { x, y }
    }
}

impl Into<(f32, f32)> for CPoint {
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
    }
}
