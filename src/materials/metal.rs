use crate::{
    hittable::HitRecord,
    materials::Material,
    math::{Color, Vec3},
    ray::Ray,
};

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = r_in.dir.normalized().reflect(&rec.normal);

        if reflected.dot(&rec.normal) > 0.0 {
            Some((
                Ray {
                    origin: rec.p,
                    dir: reflected + Vec3::random_unit() * self.fuzz,
                },
                self.albedo,
            ))
        } else {
            None
        }
    }
}
