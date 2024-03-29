use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    materials::Material,
    ray::Ray,
    Point3, Vec3,
};
use std::ops::Range;

pub struct XzRect<'a> {
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: &'a Material<'a>,
}

impl Hittable for XzRect<'_> {
    fn gets_hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y) / ray.dir.y;

        if !t_range.contains(&t) {
            return None;
        }

        let x = t.mul_add(ray.dir.x, ray.origin.x);
        let z = t.mul_add(ray.dir.z, ray.origin.z);

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let p = ray.at(t);

        let outward_normal = Vec3::Y;

        Some(HitRecord::new(p, outward_normal, self.material, t, ray))
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb {
            minimum: Point3::new(self.x0, self.k - 0.0001, self.z0),
            maximum: Point3::new(self.x1, self.k + 0.0001, self.z1),
        })
    }
}
