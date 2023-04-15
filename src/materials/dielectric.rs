use crate::{
    color::Color, hittable::HitRecord, materials::Material, ray::Ray,
    raytracer::reflect, Vec3,
};
use fastrand::Rng;

pub struct Dielectric {
    pub ir: f32,
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &Rng,
    ) -> Option<(Ray, Color)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.dir.normalize();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = cos_theta.mul_add(-cos_theta, 1.0).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract
            || reflectance(cos_theta, refraction_ratio) > rng.f32()
        {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, refraction_ratio)
        };

        Some((
            Ray {
                origin: rec.p,
                dir: direction,
            },
            Color::ONE,
        ))
    }
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
    let r_out_parallel =
        n * -(((1.0 - r_out_perp.length_squared()).abs()).sqrt());
    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    (1.0 - r0).mul_add((1.0 - cosine).powi(5), r0)
}
