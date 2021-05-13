use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    materials::Material,
    point3::Point3,
    ray::Ray,
    vec3::Vec3,
};
use std::ops::Range;

pub struct XyRect<'a> {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub material: &'a (dyn Material + Sync),
}

impl Hittable for XyRect<'_> {
    fn gets_hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.dir.z;

        if !t_range.contains(&t) {
            return None;
        }

        let x = ray.origin.x + t * ray.dir.x;
        let y = ray.origin.y + t * ray.dir.y;

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let p = ray.at(t);

        let outward_normal = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };

        Some(HitRecord::new(p, &outward_normal, self.material, t, ray))
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb {
            minimum: Point3 {
                x: self.x0,
                y: self.y0,
                z: self.k - 0.0001,
            },
            maximum: Point3 {
                x: self.x1,
                y: self.y1,
                z: self.k + 0.0001,
            },
        })
    }
}
