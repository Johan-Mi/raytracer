use super::color::Color;
use crate::materials::{
    Dielectric, DiffuseLight, Lambertian, Material as DynMaterial, Metal,
};
use bumpalo::Bump;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f32 },
    Dielectric { ir: f32 },
    DiffuseLight { color: Color },
}

impl Material {
    pub fn build(self, arena: &Bump) -> &(dyn DynMaterial + Sync) {
        match self {
            Material::Lambertian { albedo } => arena.alloc(Lambertian {
                albedo: albedo.into(),
            }),
            Material::Metal { albedo, fuzz } => arena.alloc(Metal {
                albedo: albedo.into(),
                fuzz,
            }),
            Material::Dielectric { ir } => arena.alloc(Dielectric { ir }),
            Material::DiffuseLight { color } => arena.alloc(DiffuseLight {
                color: color.into(),
            }),
        }
    }
}
