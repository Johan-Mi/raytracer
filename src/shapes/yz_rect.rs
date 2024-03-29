use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    materials::Material,
    ray::Ray,
    Point3, Vec3,
};
use std::ops::Range;

pub struct YzRect<'a> {
    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: &'a Material<'a>,
}

impl Hittable for YzRect<'_> {
    fn gets_hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let t = (self.k - ray.origin.x) / ray.dir.x;

        if !t_range.contains(&t) {
            return None;
        }

        let y = t.mul_add(ray.dir.y, ray.origin.y);
        let z = t.mul_add(ray.dir.z, ray.origin.z);

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let p = ray.at(t);

        let outward_normal = Vec3::X;

        Some(HitRecord::new(p, outward_normal, self.material, t, ray))
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb {
            minimum: Point3::new(self.k - 0.0001, self.y0, self.z0),
            maximum: Point3::new(self.k + 0.0001, self.y1, self.z1),
        })
    }
}
