use super::color::Color;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f32 },
    Dielectric { ir: f32 },
}
