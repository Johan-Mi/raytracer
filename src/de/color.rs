use crate::vec3::Vec3;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Color(f32, f32, f32);

impl From<Color> for Vec3 {
    fn from(val: Color) -> Self {
        Vec3 {
            x: val.0,
            y: val.1,
            z: val.2,
        }
    }
}
