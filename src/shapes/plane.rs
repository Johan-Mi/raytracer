use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    materials::Material,
    math::{Point3, Vec3},
    ray::Ray,
};

pub struct Plane<'a> {
    pub pos: Point3,
    pub normal: Vec3,
    pub material: &'a (dyn Material + Sync + Send),
}

impl Hittable for Plane<'_> {
    fn gets_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let diff = ray.origin - self.pos;
        let prod1 = Vec3::dot(&diff, &self.normal);
        let prod2 = Vec3::dot(&ray.dir, &self.normal);
        if prod2 == 0.0 {
            return None;
        }
        let prod3 = prod1 / prod2;

        let t = diff.len();
        if t < t_min || t_max < t {
            return None;
        }

        let p = ray.origin - ray.dir * prod3;

        let mut rec = HitRecord {
            p,
            t,
            normal: Vec3::default(),
            material: self.material.clone(),
            front_face: bool::default(),
        };

        rec.set_face_normal(ray, &self.normal);

        Some(rec)
    }

    fn bounding_box(&self) -> Option<AABB> {
        None
    }
}
