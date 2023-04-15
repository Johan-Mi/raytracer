use crate::{color::Color, hittable::HitRecord, materials::Material, ray::Ray};
use fastrand::Rng;

pub struct DiffuseLight {
    pub color: Color,
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _rng: &Rng,
    ) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, _rng: &Rng) -> Color {
        self.color
    }
}
