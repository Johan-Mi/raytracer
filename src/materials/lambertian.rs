use crate::{
    color::Color, hittable::HitRecord, materials::Material, ray::Ray, Vec3,
};
use fastrand::Rng;

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        rng: &Rng,
    ) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + crate::random::unit(rng);

        if scatter_direction.abs().cmplt(Vec3::splat(1e-8)).all() {
            scatter_direction = rec.normal;
        }

        Some((
            Ray {
                origin: rec.p,
                dir: scatter_direction,
            },
            self.albedo,
        ))
    }
}
