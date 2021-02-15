use super::color::Color;
use crate::materials::{
    Dielectric, Lambertian, Material as DynMaterial, Metal,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f32 },
    Dielectric { ir: f32 },
}

impl Into<Arc<dyn DynMaterial + Send + Sync>> for Material {
    fn into(self) -> Arc<dyn DynMaterial + Send + Sync> {
        match self {
            Material::Lambertian { albedo } => Arc::new(Lambertian {
                albedo: albedo.into(),
            }),
            Material::Metal { albedo, fuzz } => Arc::new(Metal {
                albedo: albedo.into(),
                fuzz,
            }),
            Material::Dielectric { ir } => Arc::new(Dielectric { ir }),
        }
    }
}
