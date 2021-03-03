use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    materials::Material,
    ray::Ray,
    vec3::Vec3,
};
use rand::Rng;

pub struct ConstantMedium<'a> {
    pub boundry: &'a (dyn Hittable + Sync),
    pub phase_function: &'a (dyn Material + Sync),
    pub neg_inv_density: f32,
}

impl Hittable for ConstantMedium<'_> {
    fn gets_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rng = rand::thread_rng();

        let mut rec1 =
            self.boundry
                .gets_hit(ray, f32::NEG_INFINITY, f32::INFINITY)?;
        let mut rec2 =
            self.boundry.gets_hit(ray, rec1.t + 0.0001, f32::INFINITY)?;

        rec1.t = rec1.t.max(t_min);
        rec2.t = rec2.t.min(t_max);

        if rec1.t >= rec2.t {
            return None;
        }

        rec1.t = rec1.t.max(0.0);

        let ray_length = ray.dir.len();
        let distance_inside_boundry = (rec2.t - rec1.t) * ray_length;
        let hit_distance =
            self.neg_inv_density * rng.gen_range(0.0..1.0f32).ln();

        if hit_distance > distance_inside_boundry {
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;
        let p = ray.at(t);

        Some(HitRecord {
            t,
            p,
            normal: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            front_face: true,
            material: self.phase_function,
        })
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.boundry.bounding_box()
    }
}
