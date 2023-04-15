use crate::Vec3;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Color(f32, f32, f32);

impl Default for Color {
    fn default() -> Self {
        Self(0.0, 0.0, 0.0)
    }
}

impl From<Color> for Vec3 {
    fn from(val: Color) -> Self {
        Self::new(val.0, val.1, val.2)
    }
}
