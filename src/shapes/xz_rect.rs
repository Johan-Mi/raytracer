use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    materials::Material,
    point3::Point3,
    ray::Ray,
    vec3::Vec3,
};

pub struct XzRect<'a> {
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: &'a (dyn Material + Sync),
}

impl Hittable for XzRect<'_> {
    fn gets_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y) / ray.dir.y;

        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin.x + t * ray.dir.x;
        let z = ray.origin.z + t * ray.dir.z;

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let p = ray.at(t);

        let outward_normal = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };

        Some(HitRecord::new(p, &outward_normal, self.material, t, ray))
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb {
            minimum: Point3 {
                x: self.x0,
                y: self.k - 0.0001,
                z: self.z0,
            },
            maximum: Point3 {
                x: self.x1,
                y: self.k + 0.0001,
                z: self.z1,
            },
        })
    }
}
