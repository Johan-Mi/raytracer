use crate::{math::Point3, ray::Ray};
use std::mem;

#[derive(Clone)]
pub struct AABB {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl AABB {
    pub fn collides(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for i in 0..3 {
            let inv_d = 1.0 / ray.dir[i];
            let mut t0 = (self.minimum[i] - ray.origin[i]) * inv_d;
            let mut t1 = (self.maximum[i] - ray.origin[i]) * inv_d;
            if inv_d < 0.0 {
                mem::swap(&mut t0, &mut t1);
            }

            let t_min = t0.max(t_min);
            let t_max = t1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(&self, other: &Self) -> AABB {
        AABB {
            minimum: Point3 {
                x: self.minimum.x.min(other.minimum.x),
                y: self.minimum.y.min(other.minimum.y),
                z: self.minimum.z.min(other.minimum.z),
            },
            maximum: Point3 {
                x: self.maximum.x.max(other.maximum.x),
                y: self.maximum.y.max(other.maximum.y),
                z: self.maximum.z.max(other.maximum.z),
            },
        }
    }
}
