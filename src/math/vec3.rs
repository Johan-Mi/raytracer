use derive_more::{Add, Div, Mul, Sub};
use rand::Rng;

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

    pub fn normalized(&self) -> Vec3 {
        *self / self.len()
    }

    pub fn random_unit() -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-1.0..1.0);
        let y = rng.gen_range(-1.0..1.0);
        let z = rng.gen_range(-1.0..1.0);

        Vec3 { x, y, z }.normalized()
    }

    pub fn near_zero(&self) -> bool {
        const CLOSE: f32 = 1e-8;
        self.x.abs() < CLOSE && self.y.abs() < CLOSE && self.z.abs() < CLOSE
    }

    pub fn elementwise_mul(&self, other: &Self) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;
