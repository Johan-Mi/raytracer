use crate::{color::Color, hittable::HitRecord, materials::Material, ray::Ray};
use rand::rngs::ThreadRng;

pub struct Isotropic {
    pub albedo: Color,
}

impl Material for Isotropic {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Color)> {
        Some((
            Ray {
                origin: rec.p,
                dir: crate::random::unit(rng),
            },
            self.albedo,
        ))
    }
}
