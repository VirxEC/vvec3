use std::{
    f32::consts::PI,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// a 3d vector
#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    /// Position on the x-axis
    pub x: f32,
    /// Position on the y-axis
    pub y: f32,
    /// Position on the z-axis
    pub z: f32,
}

impl Vec3 {
    /// Get a new instance of Vec3
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    /// Get the dot product of the two vectors
    pub fn dot(&self, vec: &Self) -> f32 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }

    /// Get the dot product of the two vectors
    ///
    /// Ignores the z axis
    pub fn dot_2d(&self, vec2: &Self) -> f32 {
        self.x * vec2.x + self.y * vec2.y
    }

    /// Get the magnitude (length) of the vector
    pub fn magnitude(&self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Get the magnitude (length) of the vector
    ///
    /// Ignores the z axis
    pub fn magnitude_2d(&self) -> f32 {
        self.dot_2d(self).sqrt()
    }

    /// Get the normailzed vector
    pub fn normalize(&self) -> Self {
        let size = self.magnitude();
        if size == 0. {
            return Self::default();
        }

        *self / size
    }

    /// Set the length of the vector
    pub fn scale(&self, value: f32) -> Self {
        self.normalize() * value
    }

    /// Get the cross product of the two vectors
    pub fn cross(&self, vec: &Self) -> Self {
        Self {
            x: self.y * vec.z - self.z * vec.y,
            y: self.z * vec.x - self.x * vec.z,
            z: self.x * vec.y - self.y * vec.x,
        }
    }

    /// Get a vector with minimum values set
    pub fn min(&self, vec: &Self) -> Self {
        Self {
            x: self.x.min(vec.x),
            y: self.y.min(vec.y),
            z: self.z.min(vec.z),
        }
    }

    /// Get a vector with maximum values set
    pub fn max(&self, vec: &Self) -> Self {
        Self {
            x: self.x.max(vec.x),
            y: self.y.max(vec.y),
            z: self.z.max(vec.z),
        }
    }

    /// Get a vector with maximum and minimum values set
    pub fn clamp(&self, min: &Self, max: &Self) -> Self {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
            z: self.z.clamp(min.z, max.z),
        }
    }

    /// Get the vector with a z value of 0
    pub fn flatten(self) -> Self {
        Vec3 {
            x: self.x,
            y: self.y,
            z: 0.,
        }
    }

    /// Get the angle between two vectors
    ///
    /// Ignores the z axis
    ///
    /// Returns a value in between [0, pi]
    pub fn angle_2d(&self, vec2: &Self) -> f32 {
        self.dot_2d(vec2).clamp(-1., 1.).acos()
    }

    /// Get the angle between two vectors
    ///
    /// Ignores the z axis
    ///
    /// Returns a value in between [0, 2pi]
    pub fn angle_tau_2d(&self, vec2: &Self) -> f32 {
        let angle = vec2.y.atan2(vec2.x) - self.y.atan2(self.x);

        if angle < 0. {
            return angle + 2. * PI;
        }

        angle
    }

    /// Get the distance between the tips of two vectors
    pub fn dist(self, vec2: Self) -> f32 {
        (vec2 - self).magnitude()
    }

    /// Get the distance between the tips of two vectors
    ///
    /// Ignores the z axis
    pub fn dist_2d(self, vec2: Self) -> f32 {
        (vec2 - self).magnitude_2d()
    }

    /// Get the vector rotated by some radians
    ///
    /// Ignores the z axis
    pub fn rotate_2d(&self, angle: &f32) -> Self {
        Self {
            x: angle.cos() * self.x - angle.sin() * self.y,
            y: angle.sin() * self.x + angle.cos() * self.y,
            z: self.z,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl Add<Vec3> for f32 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self + other.x,
            y: self + other.y,
            z: self + other.z,
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, other: f32) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl Sub<Vec3> for f32 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self - other.x,
            y: self - other.y,
            z: self - other.z,
        }
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl SubAssign<f32> for Vec3 {
    fn sub_assign(&mut self, other: f32) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self / other.x,
            y: self / other.y,
            z: self / other.z,
        }
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}
