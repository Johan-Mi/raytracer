use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    materials::Material,
    point3::Point3,
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere<'a> {
    pub center: Point3,
    pub radius: f32,
    pub material: &'a (dyn Material + Sync),
}

impl Hittable for Sphere<'_> {
    fn gets_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.dir.len_squared();
        let half_b = Vec3::dot(&oc, &ray.dir);
        let c = oc.len_squared() - self.radius * self.radius;

        #[allow(clippy::suspicious_operation_groupings)]
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = ray.at(root);
        let t = root;
        let normal = (p - self.center) / self.radius;

        Some(HitRecord::new(p, &normal, self.material, t, ray))
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            minimum: self.center
                - Vec3 {
                    x: self.radius,
                    y: self.radius,
                    z: self.radius,
                },
            maximum: self.center
                + Vec3 {
                    x: self.radius,
                    y: self.radius,
                    z: self.radius,
                },
        })
    }
}
