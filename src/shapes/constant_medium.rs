use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    materials::Material,
    ray::Ray,
    Vec3,
};
use std::ops::Range;

pub struct ConstantMedium<'a> {
    pub boundary: &'a (dyn Hittable + Sync),
    pub phase_function: &'a Material<'a>,
    pub neg_inv_density: f32,
}

impl Hittable for ConstantMedium<'_> {
    fn gets_hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let rng = fastrand::Rng::new();

        let mut rec1 = self
            .boundary
            .gets_hit(ray, f32::NEG_INFINITY..f32::INFINITY)?;
        let mut rec2 = self
            .boundary
            .gets_hit(ray, (rec1.t + 0.0001)..f32::INFINITY)?;

        rec1.t = rec1.t.max(t_range.start);
        rec2.t = rec2.t.min(t_range.end);

        if rec1.t >= rec2.t {
            return None;
        }

        rec1.t = rec1.t.max(0.0);

        let ray_length = ray.dir.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rng.f32().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;
        let p = ray.at(t);

        Some(HitRecord {
            t,
            p,
            normal: Vec3::X,
            front_face: true,
            material: self.phase_function,
        })
    }

    fn bounding_box(&self) -> Option<Aabb> {
        self.boundary.bounding_box()
    }
}
