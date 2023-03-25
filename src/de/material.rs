use super::color::Color;
use crate::materials::{
    Dielectric, DiffuseLight, Isotropic, Lambertian, Material as DynMaterial,
    Metal, MixedMaterial,
};
use bumpalo::Bump;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Material {
    Lambertian {
        albedo: Color,
    },
    Metal {
        albedo: Color,
        fuzz: f32,
    },
    Dielectric {
        ir: f32,
    },
    DiffuseLight {
        color: Color,
    },
    Isotropic {
        albedo: Color,
    },
    #[serde(rename = "MixedMaterial")]
    Mixed {
        primary: Box<Material>,
        secondary: Box<Material>,
        chance: f32,
    },
}

impl Material {
    pub fn build(self, arena: &Bump) -> &(dyn DynMaterial + Sync) {
        match self {
            Self::Lambertian { albedo } => arena.alloc(Lambertian {
                albedo: albedo.into(),
            }),
            Self::Metal { albedo, fuzz } => arena.alloc(Metal {
                albedo: albedo.into(),
                fuzz,
            }),
            Self::Dielectric { ir } => arena.alloc(Dielectric { ir }),
            Self::DiffuseLight { color } => arena.alloc(DiffuseLight {
                color: color.into(),
            }),
            Self::Isotropic { albedo } => arena.alloc(Isotropic {
                albedo: albedo.into(),
            }),
            Self::Mixed {
                primary,
                secondary,
                chance,
            } => arena.alloc(MixedMaterial {
                primary: primary.build(arena),
                secondary: secondary.build(arena),
                chance,
            }),
        }
    }
}
