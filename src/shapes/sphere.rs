use crate::{
    hittable::{HitRecord, Hittable},
    math::{Point3, Vec3},
    ray::Ray,
};

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn gets_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.dir.len_squared();
        let half_b = Vec3::dot(&oc, &ray.dir);
        let c = oc.len_squared() - self.radius * self.radius;

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

        Some(HitRecord {
            p,
            t: root,
            normal: (p - self.center) / self.radius,
        })
    }
}
