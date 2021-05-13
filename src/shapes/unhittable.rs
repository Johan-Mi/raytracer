use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    ray::Ray,
};
use std::ops::Range;

pub struct Unhittable;

impl Hittable for Unhittable {
    fn gets_hit(
        &self,
        _ray: &Ray,
        _t_range: Range<f32>,
        _rng: &mut crate::rng::Rng,
    ) -> Option<HitRecord> {
        None
    }

    fn bounding_box(&self) -> Option<Aabb> {
        None
    }
}
