use std::ops::Mul;

#[derive(Debug, Copy, Clone)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Colour {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const GRAY: Colour = Colour::new(192, 192, 192);
    pub const DARK_GRAY: Colour = Colour::new(128, 128, 128);
    pub const VERY_DARK_GRAY: Colour = Colour::new(64, 64, 64);
    pub const RED: Colour = Colour::new(255, 0, 0);
    pub const DARK_RED: Colour = Colour::new(128, 0, 0);
    pub const VERY_DARK_RED: Colour = Colour::new(64, 0, 0);
    pub const YELLOW: Colour = Colour::new(255, 255, 0);
    pub const DARK_YELLOW: Colour = Colour::new(128, 128, 0);
    pub const VERY_DARK_YELLOW: Colour = Colour::new(64, 64, 0);
    pub const GREEN: Colour = Colour::new(0, 255, 0);
    pub const DARK_GREEN: Colour = Colour::new(0, 128, 0);
    pub const VERY_DARK_GREEN: Colour = Colour::new(0, 64, 0);
    pub const CYAN: Colour = Colour::new(0, 255, 255);
    pub const DARK_CYAN: Colour = Colour::new(0, 128, 128);
    pub const VERY_DARK_CYAN: Colour = Colour::new(0, 64, 64);
    pub const BLUE: Colour = Colour::new(0, 0, 255);
    pub const DARK_BLUE: Colour = Colour::new(0, 0, 128);
    pub const VERY_DARK_BLUE: Colour = Colour::new(0, 0, 64);
    pub const MAGENTA: Colour = Colour::new(255, 0, 255);
    pub const DARK_MAGENTA: Colour = Colour::new(128, 0, 128);
    pub const VERY_DARK_MAGENTA: Colour = Colour::new(64, 0, 64);
    pub const BLACK: Colour = Colour::new(0, 0, 0);
    pub const WHITE: Colour = Colour::new(255, 255, 255);
}


impl Mul<f32> for Colour {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Colour::new(
            (self.r as f32 * rhs).min(255.0).max(0.0) as u8,
            (self.g as f32 * rhs).min(255.0).max(0.0) as u8,
            (self.b as f32 * rhs).min(255.0).max(0.0) as u8
        )
    }
}