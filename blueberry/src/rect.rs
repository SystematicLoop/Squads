use super::Vec2f;

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        let left = self.x < other.x && self.x + self.w < other.x;
        let right = self.x > other.x && self.x > other.x + other.w;
        let above = self.y < other.y && self.y + self.h < other.y;
        let below = self.y > other.y && self.y > other.y + other.h;

        !(left || right || above || below)
    }

    pub fn contains(&self, p: Vec2f) -> bool {
        self.x <= p.x && self.x + self.w >= p.x && self.y <= p.y && self.y + self.h >= p.y
    }
}
