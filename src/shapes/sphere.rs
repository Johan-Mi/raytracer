use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    materials::Material,
    ray::Ray,
    Point3, Vec3,
};
use std::ops::Range;

pub struct Sphere<'a> {
    pub center: Point3,
    pub radius: f32,
    pub material: &'a Material<'a>,
}

impl Hittable for Sphere<'_> {
    fn gets_hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(ray.dir);
        let c = self.radius.mul_add(-self.radius, oc.length_squared());

        #[allow(clippy::suspicious_operation_groupings)]
        let discriminant = half_b.mul_add(half_b, -a * c);
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !t_range.contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !t_range.contains(&root) {
                return None;
            }
        }

        let p = ray.at(root);
        let t = root;
        let normal = (p - self.center) / self.radius;

        Some(HitRecord::new(p, normal, self.material, t, ray))
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb {
            minimum: self.center - Vec3::splat(self.radius),
            maximum: self.center + Vec3::splat(self.radius),
        })
    }
}
