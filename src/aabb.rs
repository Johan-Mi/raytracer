use crate::{point3::Point3, ray::Ray, vec3::Vec3};

#[derive(Clone)]
pub struct Aabb {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl Aabb {
    pub fn collides(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let inv_d = Vec3 {
            x: 1.0 / ray.dir.x,
            y: 1.0 / ray.dir.y,
            z: 1.0 / ray.dir.z,
        };

        let (t0_x, t1_x) = if inv_d.x < 0.0 {
            (self.maximum.x, self.minimum.x)
        } else {
            (self.minimum.x, self.maximum.x)
        };

        let (t0_y, t1_y) = if inv_d.y < 0.0 {
            (self.maximum.y, self.minimum.y)
        } else {
            (self.minimum.y, self.maximum.y)
        };

        let (t0_z, t1_z) = if inv_d.z < 0.0 {
            (self.maximum.z, self.minimum.z)
        } else {
            (self.minimum.z, self.maximum.z)
        };

        let t0_x = (t0_x - ray.origin.x) * inv_d.x;
        let t1_x = (t1_x - ray.origin.x) * inv_d.x;

        let t0_y = (t0_y - ray.origin.y) * inv_d.y;
        let t1_y = (t1_y - ray.origin.y) * inv_d.y;

        let t0_z = (t0_z - ray.origin.z) * inv_d.z;
        let t1_z = (t1_z - ray.origin.z) * inv_d.z;

        let t_min = t0_x.max(t_min);
        let t_max = t1_x.min(t_max);
        let a = t_max > t_min;

        let t_min = t0_y.max(t_min);
        let t_max = t1_y.min(t_max);
        let b = t_max > t_min;

        let t_min = t0_z.max(t_min);
        let t_max = t1_z.min(t_max);
        let c = t_max > t_min;

        a & b & c
    }

    pub fn surrounding_box(&self, other: &Self) -> Aabb {
        Aabb {
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
