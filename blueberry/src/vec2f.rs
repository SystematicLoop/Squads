use std::ops::{
    Add,
    AddAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Neg,
    Sub,
    SubAssign,
};

#[macro_export]
macro_rules! vec2f {
    // Vector whose components all equal zero.
    () => {
        Vec2f::new(0.0, 0.0)
    };

    // Vector whose components all equal a given value.
    ($value:expr) => {
        Vec2f::new($value, $value)
    };

    // Vector whose components correspond to the given values.
    ($x:expr, $y:expr) => {
        Vec2f::new($x, $y)
    };
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        vec2f!(0.0)
    }

    pub fn one() -> Self {
        vec2f!(1.0)
    }

    pub fn splat(value: f32) -> Self {
        vec2f!(value)
    }

    pub fn unit_x() -> Self {
        vec2f!(1.0, 0.0)
    }

    pub fn unit_y() -> Self {
        vec2f!(0.0, 1.0)
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        (self - other) * t
    }

    pub fn mag(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn norm(self) -> Self {
        self / self.mag()
    }

    pub fn dist(self, other: Self) -> f32 {
        (other - self).mag()
    }

    pub fn abs(self) -> Self {
        vec2f!(self.x.abs(), self.y.abs())
    }

    pub fn ratio(self) -> f32 {
        self.x / self.y
    }

    pub fn area(self) -> f32 {
        self.x * self.y
    }

    pub fn min() -> Self {
        vec2f!(std::f32::MIN)
    }

    pub fn max() -> Self {
        vec2f!(std::f32::MAX)
    }
}

impl Add for Vec2f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vec2f {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2f {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vec2f {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<f32> for Vec2f {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul for Vec2f {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl MulAssign<f32> for Vec2f {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl MulAssign for Vec2f {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Div<f32> for Vec2f {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl Div for Vec2f {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl DivAssign<f32> for Vec2f {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl DivAssign for Vec2f {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Neg for Vec2f {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.x * -1.0, self.y * -1.0)
    }
}

impl std::fmt::Display for Vec2f {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl From<(f32, f32)> for Vec2f {
    fn from(components: (f32, f32)) -> Self {
        Self::new(components.0, components.1)
    }
}

impl Into<(f32, f32)> for Vec2f {
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
    }
}
