use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    materials::Material,
    point3::Point3,
    ray::Ray,
    vec3::Vec3,
};

pub struct Plane<'a> {
    pub pos: Point3,
    pub normal: Vec3,
    pub material: &'a (dyn Material + Sync),
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

        Some(HitRecord::new(p, &self.normal, self.material, t, ray))
    }

    fn bounding_box(&self) -> Option<Aabb> {
        None
    }
}
