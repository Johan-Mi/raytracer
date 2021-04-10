use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct Unhittable;

impl Hittable for Unhittable {
    fn gets_hit(
        &self,
        _ray: &Ray,
        _t_min: f32,
        _t_max: f32,
    ) -> Option<HitRecord> {
        None
    }

    fn bounding_box(&self) -> Option<Aabb> {
        None
    }
}
