use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    materials::Material,
    ray::Ray,
    Point3,
};
use std::ops::Range;

pub struct Triangle<'a> {
    pub points: [Point3; 3],
    pub material: &'a (dyn Material + Sync),
}

impl Hittable for Triangle<'_> {
    fn gets_hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        const EPSILON: f32 = 0.000_000_1;

        let [p0, p1, p2] = self.points;

        let edge_1 = p1 - p0;
        let edge_2 = p2 - p0;

        let dir_cross_e2 = ray.dir.cross(edge_2);
        let a = edge_1.dot(dir_cross_e2);
        if a > -EPSILON && a < EPSILON {
            return None;
        }

        let inv_a = 1.0 / a;
        let offset = ray.origin - p0;
        let u = inv_a * offset.dot(dir_cross_e2);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }
        let s_cross_e1 = offset.cross(edge_1);
        let v = inv_a * ray.dir.dot(s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = inv_a * edge_2.dot(s_cross_e1);
        if t_range.contains(&t) {
            let hit_point = ray.at(t);
            let normal = edge_2.cross(edge_1).normalize();

            Some(HitRecord::new(hit_point, normal, self.material, t, ray))
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<Aabb> {
        let [p0, p1, p2] = self.points;

        Some(Aabb {
            minimum: p0.min(p1).min(p2),
            maximum: p0.max(p1).max(p2),
        })
    }
}
