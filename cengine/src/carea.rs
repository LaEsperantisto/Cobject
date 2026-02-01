use crate::{get_window_height, get_window_width, CPoint};

#[derive(Debug, Clone)]
pub struct CArea {
    pub p0: CPoint, // top-left
    pub p1: CPoint, // top-right
    pub p2: CPoint, // bottom-right
    pub p3: CPoint, // bottom-left
}

impl CArea {
    pub fn from_rect(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            p0: CPoint { x, y },
            p1: CPoint { x: x + width, y },
            p2: CPoint {
                x: x + width,
                y: y + height,
            },
            p3: CPoint { x, y: y + height },
        }
    }

    pub fn min_x(&self) -> f32 {
        self.p0.x.min(self.p3.x)
    }
    pub fn max_x(&self) -> f32 {
        self.p1.x.max(self.p2.x)
    }
    pub fn min_y(&self) -> f32 {
        self.p0.y.min(self.p1.y)
    }
    pub fn max_y(&self) -> f32 {
        self.p2.y.max(self.p3.y)
    }

    pub fn width(&self) -> f32 {
        self.max_x() - self.min_x()
    }

    pub fn height(&self) -> f32 {
        self.max_y() - self.min_y()
    }

    pub fn center_x(&self) -> f32 {
        (self.min_x() + self.max_x()) * 0.5
    }

    pub fn center_y(&self) -> f32 {
        (self.min_y() + self.max_y()) * 0.5
    }

    pub fn collides(&self, other: &CArea) -> bool {
        self.min_x() <= other.max_x()
            && self.max_x() >= other.min_x()
            && self.min_y() <= other.max_y()
            && self.max_y() >= other.min_y()
    }

    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.min_x() && x <= self.max_x() && y >= self.min_y() && y <= self.max_y()
    }

    pub fn is_in_window(&self) -> bool {
        self.min_x() >= 0.0
            && self.max_x() < get_window_width() as f32
            && self.min_y() >= 0.0
            && self.max_y() < get_window_height() as f32
    }

    pub fn rotate(&mut self, angle: f32) {
        for point in [&mut self.p0, &mut self.p1, &mut self.p2, &mut self.p3] {
            let old_x = point.x;
            let old_y = point.y;
            point.x = old_x * angle.cos() - old_y * angle.sin();
            point.y = old_x * angle.sin() + old_y * angle.cos();
        }
    }
}

pub fn empty_area() -> CArea {
    CArea::from_rect(0.0, 0.0, 0.0, 0.0)
}
