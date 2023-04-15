use crate::Vec3;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
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

impl From<Point> for Vec3 {
    fn from(val: Point) -> Self {
        Self::new(val.x, val.y, val.z)
    }
}

impl From<Direction> for Vec3 {
    fn from(val: Direction) -> Self {
        Self::new(val.x, val.y, val.z)
    }
}
