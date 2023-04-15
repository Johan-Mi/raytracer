use super::color::Color;
use crate::materials::{
    Dielectric, Isotropic, Lambertian, Material as RealMaterial, Metal,
    MixedMaterial,
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
    pub fn build(self, arena: &Bump) -> &RealMaterial {
        match self {
            Self::Lambertian { albedo } => {
                arena.alloc(RealMaterial::Lambertian(Lambertian {
                    albedo: albedo.into(),
                }))
            }
            Self::Metal { albedo, fuzz } => {
                arena.alloc(RealMaterial::Metal(Metal {
                    albedo: albedo.into(),
                    fuzz,
                }))
            }
            Self::Dielectric { ir } => {
                arena.alloc(RealMaterial::Dielectric(Dielectric { ir }))
            }
            Self::DiffuseLight { color } => {
                arena.alloc(RealMaterial::DiffuseLight(color.into()))
            }
            Self::Isotropic { albedo } => {
                arena.alloc(RealMaterial::Isotropic(Isotropic {
                    albedo: albedo.into(),
                }))
            }
            Self::Mixed {
                primary,
                secondary,
                chance,
            } => arena.alloc(RealMaterial::MixedMaterial(MixedMaterial {
                primary: primary.build(arena),
                secondary: secondary.build(arena),
                chance,
            })),
        }
    }
}
