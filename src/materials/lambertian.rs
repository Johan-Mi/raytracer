use crate::{
    hittable::HitRecord,
    materials::Material,
    math::{Color, Vec3},
    ray::Ray,
};

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit();

        if scatter_direction.near_zero() {
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
