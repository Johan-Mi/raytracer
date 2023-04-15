use crate::{color::Color, hittable::HitRecord, ray::Ray};
use rand::rngs::ThreadRng;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Color)>;

    fn emitted(&self, _rng: &mut ThreadRng) -> Color {
        Color::ZERO
    }
}
