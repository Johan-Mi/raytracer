use crate::{color::Color, hittable::HitRecord, materials::Material, ray::Ray};
use rand::{rngs::ThreadRng, Rng};

pub struct MixedMaterial<'a> {
    pub primary: &'a (dyn Material + Sync),
    pub secondary: &'a (dyn Material + Sync),
    pub chance: f32,
}

impl Material for MixedMaterial<'_> {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Color)> {
        if rng.gen::<f32>() < self.chance {
            self.secondary
        } else {
            self.primary
        }
        .scatter(r_in, rec, rng)
    }

    fn emitted(&self, rng: &mut ThreadRng) -> Color {
        if rng.gen::<f32>() < self.chance {
            self.secondary
        } else {
            self.primary
        }
        .emitted(rng)
    }
}
