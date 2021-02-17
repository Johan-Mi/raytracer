use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    materials::Material,
    math::{Point3, Vec3},
    ray::Ray,
};
use std::sync::Arc;

pub struct XZRect {
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: Arc<dyn Material + Sync + Send>,
}

impl Hittable for XZRect {
    fn gets_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y) / ray.dir.y;

        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin.x + t * ray.dir.x;
        let z = ray.origin.z + t * ray.dir.z;

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let mut rec = HitRecord {
            p: ray.at(t),
            normal: Vec3::default(),
            material: self.material.clone(),
            t,
            front_face: bool::default(),
        };

        let outward_normal = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };

        rec.set_face_normal(ray, &outward_normal);

        Some(rec)
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            minimum: Point3 {
                x: self.x0,
                y: self.k - 0.0001,
                z: self.z0,
            },
            maximum: Point3 {
                x: self.x1,
                y: self.k + 0.0001,
                z: self.z1,
            },
        })
    }
}