use std::cmp::Ordering;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct HittableList<'a> {
    list: &'a [&'a (dyn Hittable + Sync)],
}

impl<'a> HittableList<'a> {
    pub fn new(list: &'a [&'a (dyn Hittable + Sync)]) -> Self {
        Self { list }
    }
}

impl Hittable for HittableList<'_> {
    fn gets_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.list
            .iter()
            .filter_map(|h| h.gets_hit(ray, t_min, t_max))
            .min_by(|old, new| {
                old.t.partial_cmp(&new.t).unwrap_or(Ordering::Equal)
            })
    }

    fn bounding_box(&self) -> Option<Aabb> {
        None // TODO: Actually create a bounding box
             // This isn't actually needed right now because HittableList is
             // currently only used by Cuboid, which generates its own
             // bounding box.
    }
}
