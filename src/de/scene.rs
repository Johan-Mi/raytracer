use super::material::Material;
use super::shape::Shape;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Scene {
    #[serde(default)]
    materials: HashMap<String, Material>,

    shapes: Vec<Shape>,
}
