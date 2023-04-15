use crate::{color::Color, hittable::HitRecord, materials::Material, ray::Ray};
use rand::rngs::ThreadRng;

pub struct DiffuseLight {
    pub color: Color,
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _rng: &mut ThreadRng,
    ) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, _rng: &mut ThreadRng) -> Color {
        self.color
    }
}
