use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    materials::Material,
    ray::Ray,
    Point3, Vec3,
};
use std::ops::Range;

pub struct Plane<'a> {
    pub pos: Point3,
    pub normal: Vec3,
    pub material: &'a Material<'a>,
}

impl Hittable for Plane<'_> {
    fn gets_hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let diff = ray.origin - self.pos;
        let prod1 = diff.dot(self.normal);
        let prod2 = ray.dir.dot(self.normal);
        if prod2 == 0.0 {
            return None;
        }
        let prod3 = prod1 / prod2;

        let t = diff.length();
        if !t_range.contains(&t) {
            return None;
        }

        let p = ray.origin - ray.dir * prod3;

        Some(HitRecord::new(p, self.normal, self.material, t, ray))
    }

    fn bounding_box(&self) -> Option<Aabb> {
        None
    }
}
