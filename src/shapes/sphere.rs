use crate::{
    hittable::{HitRecord, Hittable},
    materials::Material,
    math::{Point3, Vec3},
    ray::Ray,
};
use std::rc::Rc;

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn gets_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.dir.len_squared();
        let half_b = Vec3::dot(&oc, &ray.dir);
        let c = oc.len_squared() - self.radius * self.radius;

        #[allow(clippy::suspicious_operation_groupings)]
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = ray.at(root);

        let outward_normal = (p - self.center) / self.radius;

        let mut rec = HitRecord {
            p,
            t: root,
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            material: self.material.clone(),
            front_face: false,
        };

        rec.set_face_normal(ray, &outward_normal);

        Some(rec)
    }
}
