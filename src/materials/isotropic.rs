use crate::{color::Color, hittable::HitRecord, materials::Material, ray::Ray};

pub struct Isotropic {
    pub albedo: Color,
}

impl Material for Isotropic {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut rng = rand::thread_rng();

        Some((
            Ray {
                origin: rec.p,
                dir: crate::random::unit(&mut rng),
            },
            self.albedo,
        ))
    }
}
