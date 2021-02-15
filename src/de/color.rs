use crate::math::Vec3;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Color(f32, f32, f32);

impl Into<Vec3> for Color {
    fn into(self) -> Vec3 {
        Vec3 {
            x: self.0,
            y: self.1,
            z: self.2,
        }
    }
}
