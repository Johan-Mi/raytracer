use std::ops::Range;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    materials::Material,
    ray::Ray,
    shapes::{HittableList, XyRect, XzRect, YzRect},
    Point3,
};
use bumpalo::Bump;

pub struct Cuboid<'a> {
    minimum: Point3,
    maximum: Point3,
    sides: HittableList<'a>,
}

impl<'a> Cuboid<'a> {
    pub fn new(
        minimum: Point3,
        maximum: Point3,
        material: &'a (dyn Material + Sync),
        arena: &'a Bump,
    ) -> Self {
        let sides = arena.alloc::<[&(dyn Hittable + Sync); 6]>([
            arena.alloc(XyRect {
                x0: minimum.x,
                x1: maximum.x,
                y0: minimum.y,
                y1: maximum.y,
                k: maximum.z,
                material,
            }),
            arena.alloc(XyRect {
                x0: minimum.x,
                x1: maximum.x,
                y0: minimum.y,
                y1: maximum.y,
                k: minimum.z,
                material,
            }),
            arena.alloc(XzRect {
                x0: minimum.x,
                x1: maximum.x,
                z0: minimum.z,
                z1: maximum.z,
                k: maximum.y,
                material,
            }),
            arena.alloc(XzRect {
                x0: minimum.x,
                x1: maximum.x,
                z0: minimum.z,
                z1: maximum.z,
                k: minimum.y,
                material,
            }),
            arena.alloc(YzRect {
                y0: minimum.y,
                y1: maximum.y,
                z0: minimum.z,
                z1: maximum.z,
                k: maximum.x,
                material,
            }),
            arena.alloc(YzRect {
                y0: minimum.y,
                y1: maximum.y,
                z0: minimum.z,
                z1: maximum.z,
                k: minimum.x,
                material,
            }),
        ]);

        Self {
            minimum,
            maximum,
            sides: HittableList::new(sides),
        }
    }
}

impl Hittable for Cuboid<'_> {
    fn gets_hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        self.sides.gets_hit(ray, t_range)
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb {
            minimum: self.minimum,
            maximum: self.maximum,
        })
    }
}
