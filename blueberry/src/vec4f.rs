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
macro_rules! vec4f {
    // Vector whose components all equal zero.
    () => {
        Vec4f::new(0.0, 0.0, 0.0, 0.0)
    };

    // Vector whose components all equal a given value.
    ($value:expr) => {
        Vec4f::new($value, $value, $value, $value)
    };

    // Vector whose components correspond to the given values
    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        Vec4f::new($x, $y, $z, $w)
    };
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4f {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn zero() -> Self {
        vec4f!(0.0)
    }

    pub fn one() -> Self {
        vec4f!(1.0)
    }

    pub fn splat(value: f32) -> Self {
        vec4f!(value)
    }

    pub fn unit_x() -> Self {
        vec4f!(1.0, 0.0, 0.0, 0.0)
    }

    pub fn unit_y() -> Self {
        vec4f!(0.0, 1.0, 0.0, 0.0)
    }

    pub fn unit_z() -> Self {
        vec4f!(0.0, 0.0, 1.0, 0.0)
    }

    pub fn unit_w() -> Self {
        vec4f!(0.0, 0.0, 0.0, 1.0)
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        (self - other) * t
    }

    pub fn mag(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn norm(self) -> Self {
        self / self.mag()
    }

    pub fn dist(self, other: Self) -> f32 {
        (other - self).mag()
    }

    pub fn abs(self) -> Self {
        vec4f!(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
    }

    pub fn volume(self) -> f32 {
        self.x * self.y * self.z * self.w
    }

    pub fn min() -> Self {
        vec4f!(std::f32::MIN)
    }

    pub fn max() -> Self {
        vec4f!(std::f32::MAX)
    }
}

impl Add for Vec4f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl AddAssign for Vec4f {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl Sub for Vec4f {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl SubAssign for Vec4f {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl Mul<f32> for Vec4f {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Mul for Vec4f {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.z * rhs.z,
            self.w * rhs.w,
        )
    }
}

impl MulAssign<f32> for Vec4f {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl MulAssign for Vec4f {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}

impl Div<f32> for Vec4f {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl Div for Vec4f {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x / rhs.x,
            self.y / rhs.y,
            self.z / rhs.z,
            self.w / rhs.w,
        )
    }
}

impl DivAssign<f32> for Vec4f {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}

impl DivAssign for Vec4f {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}

impl Neg for Vec4f {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.x * -1.0, self.y * -1.0, self.z * -1.0, self.w * -1.0)
    }
}

impl std::fmt::Display for Vec4f {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}

impl From<(f32, f32, f32, f32)> for Vec4f {
    fn from(components: (f32, f32, f32, f32)) -> Self {
        Self::new(components.0, components.1, components.2, components.3)
    }
}

impl Into<(f32, f32, f32, f32)> for Vec4f {
    fn into(self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.w)
    }
}
