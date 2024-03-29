use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    materials::Material,
    ray::Ray,
    Point3, Vec3,
};
use std::ops::Range;

pub struct XyRect<'a> {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub material: &'a Material<'a>,
}

impl Hittable for XyRect<'_> {
    fn gets_hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.dir.z;

        if !t_range.contains(&t) {
            return None;
        }

        let x = t.mul_add(ray.dir.x, ray.origin.x);
        let y = t.mul_add(ray.dir.y, ray.origin.y);

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let p = ray.at(t);

        let outward_normal = Vec3::Z;

        Some(HitRecord::new(p, outward_normal, self.material, t, ray))
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb {
            minimum: Point3::new(self.x0, self.y0, self.k - 0.0001),
            maximum: Point3::new(self.x1, self.y1, self.k + 0.0001),
        })
    }
}
