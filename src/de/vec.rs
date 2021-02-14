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
