use crate::vec3::Vec3;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Point {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Deserialize)]
pub struct Direction {
    x: f32,
    y: f32,
    z: f32,
}

impl Into<Vec3> for Point {
    fn into(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Into<Vec3> for Direction {
    fn into(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}
