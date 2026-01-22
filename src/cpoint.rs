pub struct CPoint {
    pub x: f32,
    pub y: f32,
}

impl CPoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<Option<(f32, f32)>> for CPoint {
    fn from(o: Option<(f32, f32)>) -> Self {
        let (x, y) = o.unwrap();
        Self { x, y }
    }
}
