use crate::{color::Color, hittable::HitRecord, materials::Material, ray::Ray};

pub struct DiffuseLight {
    pub color: Color,
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _rng: &mut crate::rng::Rng,
    ) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, _rng: &mut crate::rng::Rng) -> Color {
        self.color
    }
}
