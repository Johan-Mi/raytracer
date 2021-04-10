use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    ray::Ray,
    shapes::Unhittable,
};
use bumpalo::Bump;
use rand::Rng;
use std::{cmp::Ordering, mem};

pub struct BvhNode<'a> {
    left: &'a (dyn Hittable + Sync),
    right: &'a (dyn Hittable + Sync),
    boundry: Aabb,
}

impl Hittable for BvhNode<'_> {
    fn gets_hit(
        &self,
        ray: &Ray,
        t_min: f32,
        mut t_max: f32,
    ) -> Option<HitRecord> {
        if !self.boundry.collides(ray, t_min, t_max) {
            return None;
        }

        let rec_left = self.left.gets_hit(ray, t_min, t_max);

        if let Some(rec_left) = &rec_left {
            t_max = rec_left.t;
        }

        let rec_right = self.right.gets_hit(ray, t_min, t_max);
        rec_right.or(rec_left)
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.boundry.clone())
    }
}

fn box_compare(a: &Aabb, b: &Aabb, axis: usize) -> bool {
    a.minimum[axis] < b.minimum[axis]
}

impl<'a> BvhNode<'a> {
    pub fn subdivide_objects(
        objects: &mut [&'a (dyn Hittable + Sync + 'a)],
        arena: &'a Bump,
    ) -> Option<&'a (dyn Hittable + Sync)> {
        let mut rng = rand::thread_rng();
        let axis = rng.gen_range(0..3);

        match objects {
            [] => None,

            [single] => Some(mem::replace(single, &Unhittable)),

            [a, b] => {
                let a = mem::replace(a, &Unhittable);
                let b = mem::replace(b, &Unhittable);
                let boundry_a = a.bounding_box()?;
                let boundry_b = b.bounding_box()?;

                let (left, right) = if box_compare(&boundry_a, &boundry_b, axis)
                {
                    (a, b)
                } else {
                    (b, a)
                };

                Some(arena.alloc(BvhNode {
                    left,
                    right,
                    boundry: boundry_a.surrounding_box(&boundry_b),
                }))
            }

            objects => {
                objects.sort_unstable_by(|a, b| {
                    let boundry_a = a.bounding_box().unwrap();
                    let boundry_b = b.bounding_box().unwrap();

                    if box_compare(&boundry_a, &boundry_b, axis) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });

                let middle = objects.len() / 2;

                let left_half = &mut objects[..middle];
                let left = BvhNode::subdivide_objects(left_half, arena)?;
                let left_boundry = left.bounding_box()?;

                let right_half = &mut objects[middle..];
                let right = BvhNode::subdivide_objects(right_half, arena)?;
                let right_boundry = right.bounding_box()?;

                let boundry = left_boundry.surrounding_box(&right_boundry);

                Some(arena.alloc(BvhNode {
                    left,
                    right,
                    boundry,
                }))
            }
        }
    }
}
