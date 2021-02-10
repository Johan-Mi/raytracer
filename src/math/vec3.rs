use derive_more::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Add, Sub, Mul, Div)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn lerp(&self, other: &Vec3, t: f32) -> Vec3 {
        Vec3 {
            x: (1.0 - t) * self.x + t * other.x,
            y: (1.0 - t) * self.y + t * other.y,
            z: (1.0 - t) * self.z + t * other.z,
        }
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;
