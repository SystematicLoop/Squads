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
macro_rules! vec3f {
    // Vector whose components all equal zero.
    () => {
        Vec3f::new(0.0, 0.0, 0.0)
    };

    // Vector whose components all equal a given value.
    ($value:expr) => {
        Vec3f::new($value, $value, $value)
    };

    // Vector whose components correspond to the given values.
    ($x:expr, $y:expr, $z:expr) => {
        Vec3f::new($x, $y, $z)
    };
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        vec3f!(0.0)
    }

    pub fn one() -> Self {
        vec3f!(1.0)
    }

    pub fn splat(value: f32) -> Self {
        vec3f!(value)
    }

    pub fn unit_x() -> Self {
        vec3f!(1.0, 0.0, 0.0)
    }

    pub fn unit_y() -> Self {
        vec3f!(0.0, 1.0, 0.0)
    }

    pub fn unit_z() -> Self {
        vec3f!(0.0, 0.0, 1.0)
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            -(self.x * rhs.z - self.z * rhs.x),
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        (self - other) * t
    }

    pub fn mag(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn norm(self) -> Self {
        self / self.mag()
    }

    pub fn dist(self, other: Self) -> f32 {
        (other - self).mag()
    }

    pub fn abs(self) -> Self {
        vec3f!(self.x.abs(), self.y.abs(), self.z.abs())
    }

    pub fn volume(self) -> f32 {
        self.x * self.y * self.z
    }

    pub fn min() -> Self {
        vec3f!(std::f32::MIN)
    }

    pub fn max() -> Self {
        vec3f!(std::f32::MAX)
    }
}

impl Add for Vec3f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vec3f {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3f {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vec3f {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f32> for Vec3f {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul for Vec3f {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl MulAssign<f32> for Vec3f {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl MulAssign for Vec3f {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Div<f32> for Vec3f {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Div for Vec3f {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl DivAssign<f32> for Vec3f {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl DivAssign for Vec3f {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Neg for Vec3f {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.x * -1.0, self.y * -1.0, self.z * -1.0)
    }
}

impl std::fmt::Display for Vec3f {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl From<(f32, f32, f32)> for Vec3f {
    fn from(components: (f32, f32, f32)) -> Self {
        Self::new(components.0, components.1, components.2)
    }
}

impl Into<(f32, f32, f32)> for Vec3f {
    fn into(self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
}
