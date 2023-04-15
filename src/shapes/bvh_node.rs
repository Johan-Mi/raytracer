use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    ray::Ray,
    shapes::Unhittable,
};
use bumpalo::Bump;
use std::{cmp::Ordering, mem, ops::Range};

pub struct BvhNode<'a> {
    left: &'a (dyn Hittable + Sync),
    right: &'a (dyn Hittable + Sync),
    boundary: Aabb,
}

impl Hittable for BvhNode<'_> {
    fn gets_hit(
        &self,
        ray: &Ray,
        mut t_range: Range<f32>,
    ) -> Option<HitRecord> {
        if !self.boundary.collides(ray, t_range.clone()) {
            return None;
        }

        let rec_left = self.left.gets_hit(ray, t_range.clone());

        if let Some(rec_left) = &rec_left {
            t_range.end = rec_left.t;
        }

        let rec_right = self.right.gets_hit(ray, t_range);
        rec_right.or(rec_left)
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.boundary.clone())
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
        let rng = fastrand::Rng::new();
        let axis = rng.usize(0..3);

        match objects {
            [] => None,

            [single] => Some(mem::replace(single, &Unhittable)),

            [a, b] => {
                let a = mem::replace(a, &Unhittable);
                let b = mem::replace(b, &Unhittable);
                let boundary_a = a.bounding_box()?;
                let boundary_b = b.bounding_box()?;

                let (left, right) =
                    if box_compare(&boundary_a, &boundary_b, axis) {
                        (a, b)
                    } else {
                        (b, a)
                    };

                Some(arena.alloc(BvhNode {
                    left,
                    right,
                    boundary: boundary_a.surrounding_box(&boundary_b),
                }))
            }

            objects => {
                objects.sort_unstable_by(|a, b| {
                    let boundary_a = a.bounding_box().unwrap();
                    let boundary_b = b.bounding_box().unwrap();

                    if box_compare(&boundary_a, &boundary_b, axis) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });

                let middle = objects.len() / 2;

                let left_half = &mut objects[..middle];
                let left = BvhNode::subdivide_objects(left_half, arena)?;
                let left_boundary = left.bounding_box()?;

                let right_half = &mut objects[middle..];
                let right = BvhNode::subdivide_objects(right_half, arena)?;
                let right_boundary = right.bounding_box()?;

                let boundary = left_boundary.surrounding_box(&right_boundary);

                Some(arena.alloc(BvhNode {
                    left,
                    right,
                    boundary,
                }))
            }
        }
    }
}
