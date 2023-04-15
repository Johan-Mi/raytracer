use crate::{color::Color, hittable::HitRecord, materials::Material, ray::Ray};
use fastrand::Rng;

pub struct Isotropic {
    pub albedo: Color,
}

impl Material for Isotropic {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        rng: &Rng,
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
