use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct HittableList {
    list: Vec<Box<dyn Hittable + Sync>>,
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable + Sync>>) -> Self {
        Self { list }
    }
}

impl Hittable for HittableList {
    fn gets_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.list
            .iter()
            .filter_map(|h| h.gets_hit(ray, t_min, t_max))
            .fold(None, |old: Option<HitRecord>, new| match old {
                Some(old) if old.t < new.t => Some(old),
                _ => Some(new),
            })
    }

    fn bounding_box(&self) -> Option<AABB> {
        None // TODO: Actually create a bounding box
             // This isn't actually needed right now because HittableList is
             // currently only used by Cuboid, which generates its own
             // bounding box.
    }
}
