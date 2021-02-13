use crate::{
    hittable::{HitRecord, Hittable},
    materials::Material,
    math::{Point3, Vec3},
    ray::Ray,
};
use std::sync::Arc;

pub struct Plane {
    pub pos: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material + Sync + Send>,
}

impl Hittable for Plane {
    fn gets_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let diff = ray.origin - self.pos;
        let prod1 = Vec3::dot(&diff, &self.normal);
        let prod2 = Vec3::dot(&ray.dir, &self.normal);
        if prod2 == 0.0 {
            return None;
        }
        let prod3 = prod1 / prod2;

        let t = diff.len();
        if t < t_min || t_max < t {
            return None;
        }

        let p = ray.origin - ray.dir * prod3;

        let mut rec = HitRecord {
            p,
            t,
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            material: self.material.clone(),
            front_face: false,
        };

        rec.set_face_normal(ray, &self.normal);

        Some(rec)
    }
}
