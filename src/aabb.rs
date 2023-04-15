use crate::{ray::Ray, Point3, Vec3};
use std::ops::Range;

#[derive(Clone)]
pub struct Aabb {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl Aabb {
    pub fn collides(&self, ray: &Ray, t_range: Range<f32>) -> bool {
        let inv_d = 1.0 / ray.dir;

        let t0 =
            Vec3::select(inv_d.cmplt(Vec3::ZERO), self.maximum, self.minimum);
        let t1 =
            Vec3::select(inv_d.cmplt(Vec3::ZERO), self.minimum, self.maximum);

        let t0 = (t0 - ray.origin) * inv_d;
        let t1 = (t1 - ray.origin) * inv_d;

        let t_min = t0.x.max(t_range.start);
        let t_max = t1.x.min(t_range.end);
        let a = t_max > t_min;

        let t_min = t0.y.max(t_min);
        let t_max = t1.y.min(t_max);
        let b = t_max > t_min;

        let t_min = t0.z.max(t_min);
        let t_max = t1.z.min(t_max);
        let c = t_max > t_min;

        a & b & c
    }

    pub fn surrounding_box(&self, other: &Self) -> Self {
        Self {
            minimum: self.minimum.min(other.minimum),
            maximum: self.maximum.max(other.maximum),
        }
    }
}
