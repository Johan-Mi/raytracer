use super::math::{Point3, Vec3};
use super::ray::Ray;

pub trait Hittable {
    fn gets_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
}