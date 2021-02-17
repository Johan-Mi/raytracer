use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    materials::Material,
    math::Point3,
    ray::Ray,
    shapes::{HittableList, XYRect, XZRect, YZRect},
};
use std::sync::Arc;

pub struct Cuboid {
    minimum: Point3,
    maximum: Point3,
    sides: HittableList,
}

impl Cuboid {
    pub fn new(
        minimum: Point3,
        maximum: Point3,
        material: Arc<dyn Material + Sync + Send>,
    ) -> Self {
        let sides: Vec<Box<dyn Hittable + Sync>> = vec![
            Box::new(XYRect {
                x0: minimum.x,
                x1: maximum.x,
                y0: minimum.y,
                y1: maximum.y,
                k: maximum.z,
                material: material.clone(),
            }),
            Box::new(XYRect {
                x0: minimum.x,
                x1: maximum.x,
                y0: minimum.y,
                y1: maximum.y,
                k: minimum.z,
                material: material.clone(),
            }),
            Box::new(XZRect {
                x0: minimum.x,
                x1: maximum.x,
                z0: minimum.z,
                z1: maximum.z,
                k: maximum.y,
                material: material.clone(),
            }),
            Box::new(XZRect {
                x0: minimum.x,
                x1: maximum.x,
                z0: minimum.z,
                z1: maximum.z,
                k: minimum.y,
                material: material.clone(),
            }),
            Box::new(YZRect {
                y0: minimum.y,
                y1: maximum.y,
                z0: minimum.z,
                z1: maximum.z,
                k: maximum.x,
                material: material.clone(),
            }),
            Box::new(YZRect {
                y0: minimum.y,
                y1: maximum.y,
                z0: minimum.z,
                z1: maximum.z,
                k: minimum.x,
                material,
            }),
        ];

        Self {
            minimum,
            maximum,
            sides: HittableList::new(sides),
        }
    }
}

impl Hittable for Cuboid {
    fn gets_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.gets_hit(ray, t_min, t_max)
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            minimum: self.minimum,
            maximum: self.maximum,
        })
    }
}
