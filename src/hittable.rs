use crate::{aabb::Aabb, materials::Material, ray::Ray, Point3, Vec3};
use std::ops::Range;

pub trait Hittable {
    fn gets_hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<Aabb>;
}

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub material: &'a Material<'a>,
    pub t: f32,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point3,
        outward_normal: Vec3,
        material: &'a Material<'a>,
        t: f32,
        r: &Ray,
    ) -> Self {
        let front_face = r.dir.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            p,
            normal,
            material,
            t,
            front_face,
        }
    }
}
