use crate::{CArea, CDrawable, CPoint};

#[derive(Debug, Clone)]
pub struct CQuad {
    pub points: [CPoint; 4], // TL, TR, BR, BL
    pub color: u32,
}

impl CQuad {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: u32) -> Self {
        Self {
            points: [
                CPoint { x, y },
                CPoint { x: x + width, y },
                CPoint {
                    x: x + width,
                    y: y + height,
                },
                CPoint { x, y: y + height },
            ],
            color,
        }
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        let w = self.width();
        let h = self.height();

        self.points[0] = CPoint { x, y };
        self.points[1] = CPoint { x: x + w, y };
        self.points[2] = CPoint { x: x + w, y: y + h };
        self.points[3] = CPoint { x, y: y + h };
    }

    pub fn width(&self) -> f32 {
        self.points[1].x - self.points[0].x
    }

    pub fn height(&self) -> f32 {
        self.points[3].y - self.points[0].y
    }

    pub fn rotate(&mut self, angle: f32) {
        for point in &mut self.points {
            let old_x = point.x;
            let old_y = point.y;
            point.x = old_x * angle.cos() - old_y * angle.sin();
            point.y = old_x * angle.sin() + old_y * angle.cos();
        }
    }
}

impl CDrawable for CQuad {
    fn draw(&self, pixels: &mut Vec<u32>, screen_width: usize, screen_height: usize) {
        let min_x = self.points[0].x.max(0.0) as usize;
        let max_x = self.points[2].x.min(screen_width as f32) as usize;
        let min_y = self.points[0].y.max(0.0) as usize;
        let max_y = self.points[2].y.min(screen_height as f32) as usize;

        for y in min_y..max_y {
            let row = y * screen_width;
            for x in min_x..max_x {
                pixels[row + x] = self.color;
            }
        }
    }
}

impl From<CQuad> for CArea {
    fn from(q: CQuad) -> Self {
        CArea {
            p0: q.points[0].clone(),
            p1: q.points[1].clone(),
            p2: q.points[2].clone(),
            p3: q.points[3].clone(),
        }
    }
}
