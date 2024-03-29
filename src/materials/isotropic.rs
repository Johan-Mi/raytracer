use crate::{color::Color, hittable::HitRecord, ray::Ray};
use fastrand::Rng;

pub struct Isotropic {
    pub albedo: Color,
}

impl Isotropic {
    pub(super) fn scatter(&self, rec: &HitRecord, rng: &Rng) -> (Ray, Color) {
        (
            Ray {
                origin: rec.p,
                dir: crate::random::unit(rng),
            },
            self.albedo,
        )
    }
}
