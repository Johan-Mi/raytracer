use crate::{color::Color, hittable::HitRecord, materials::Material, ray::Ray};
use fastrand::Rng;

pub struct MixedMaterial<'a> {
    pub primary: &'a Material<'a>,
    pub secondary: &'a Material<'a>,
    pub chance: f32,
}

impl MixedMaterial<'_> {
    pub(super) fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &Rng,
    ) -> Option<(Ray, Color)> {
        if rng.f32() < self.chance {
            self.secondary
        } else {
            self.primary
        }
        .scatter(r_in, rec, rng)
    }

    pub(super) fn emitted(&self, rng: &Rng) -> Color {
        if rng.f32() < self.chance {
            self.secondary
        } else {
            self.primary
        }
        .emitted(rng)
    }
}
