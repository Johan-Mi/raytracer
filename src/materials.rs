mod dielectric;
mod isotropic;
mod lambertian;
mod metal;
mod mixed_material;
pub use dielectric::*;
pub use isotropic::*;
pub use lambertian::*;
pub use metal::*;
pub use mixed_material::*;

use crate::{color::Color, hittable::HitRecord, ray::Ray};
use fastrand::Rng;

pub enum Material<'a> {
    Dielectric(Dielectric),
    DiffuseLight(Color),
    Isotropic(Isotropic),
    Lambertian(Lambertian),
    Metal(Metal),
    MixedMaterial(MixedMaterial<'a>),
}

impl Material<'_> {
    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &Rng,
    ) -> Option<(Ray, Color)> {
        match self {
            Material::Dielectric(m) => Some(m.scatter(r_in, rec, rng)),
            Material::DiffuseLight(_) => None,
            Material::Isotropic(m) => Some(m.scatter(rec, rng)),
            Material::Lambertian(m) => Some(m.scatter(rec, rng)),
            Material::Metal(m) => m.scatter(r_in, rec, rng),
            Material::MixedMaterial(m) => m.scatter(r_in, rec, rng),
        }
    }

    pub fn emitted(&self, rng: &Rng) -> Color {
        match self {
            Material::Dielectric(_)
            | Material::Isotropic(_)
            | Material::Lambertian(_)
            | Material::Metal(_) => Color::ZERO,
            Material::DiffuseLight(color) => *color,
            Material::MixedMaterial(m) => m.emitted(rng),
        }
    }
}
