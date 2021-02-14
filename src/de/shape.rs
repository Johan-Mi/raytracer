use super::{
    material::Material,
    var::Var,
    vec::{Direction, Point},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Shape {
    Sphere {
        center: Point,
        radius: f32,
        material: Var<Material>,
    },
    Plane {
        position: Point,
        normal: Direction,
    },
}
