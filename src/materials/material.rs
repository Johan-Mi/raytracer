use crate::{color::Color, hittable::HitRecord, ray::Ray};
use fastrand::Rng;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &Rng,
    ) -> Option<(Ray, Color)>;

    fn emitted(&self, _rng: &Rng) -> Color {
        Color::ZERO
    }
}
