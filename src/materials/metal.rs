use crate::{
    color::Color, hittable::HitRecord, materials::Material, ray::Ray,
    raytracer::reflect,
};
use rand::rngs::ThreadRng;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Color)> {
        let reflected = reflect(r_in.dir.normalize(), rec.normal);

        if reflected.dot(rec.normal) > 0.0 {
            Some((
                Ray {
                    origin: rec.p,
                    dir: reflected + crate::random::unit(rng) * self.fuzz,
                },
                self.albedo,
            ))
        } else {
            None
        }
    }
}
