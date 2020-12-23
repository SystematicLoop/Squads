#[derive(Debug, Default, Copy, Clone)]
pub struct Clip {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub invert: bool,
}

impl Clip {
    pub fn new(x: i32, y: i32, w: i32, h: i32, invert: bool) -> Self {
        Self { x, y, w, h, invert }
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        let inside = x >= self.x && x < self.x + self.w && y >= self.y && y < self.y + self.h;
        (inside && !self.invert) || (!inside && self.invert)
    }
}
