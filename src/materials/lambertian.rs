use crate::{
    color::Color, hittable::HitRecord, materials::Material, ray::Ray,
    vec3::Vec3,
};

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut rng = rand::thread_rng();

        let mut scatter_direction = rec.normal + Vec3::random_unit(&mut rng);

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
