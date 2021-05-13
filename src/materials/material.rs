use crate::{color::Color, hittable::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut crate::rng::Rng,
    ) -> Option<(Ray, Color)>;

    fn emitted(&self, _rng: &mut crate::rng::Rng) -> Color {
        Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
