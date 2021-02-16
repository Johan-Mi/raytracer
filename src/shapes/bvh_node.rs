use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    ray::Ray,
    shapes::Unhittable,
};
use rand::Rng;
use std::{cmp::Ordering, mem};

pub struct BvhNode {
    left: Box<dyn Hittable + Sync>,
    right: Option<Box<dyn Hittable + Sync>>,
    boundry: AABB,
}

impl Hittable for BvhNode {
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

        if let Some(right) = &self.right {
            let rec_right = right.gets_hit(ray, t_min, t_max);
            rec_right.or(rec_left)
        } else {
            rec_left
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.boundry.clone())
    }
}

fn box_compare(a: &AABB, b: &AABB, axis: usize) -> bool {
    a.minimum[axis] < b.minimum[axis]
}

impl BvhNode {
    pub fn subdivide_objects(
        objects: &mut [Box<dyn Hittable + Sync>],
    ) -> Option<Self> {
        let mut rng = rand::thread_rng();
        let axis = rng.gen_range(0..3);

        match objects {
            [] => None,

            [single] => {
                let single = mem::replace(single, Box::new(Unhittable));
                let boundry = single.bounding_box()?;
                Some(Self {
                    left: single,
                    right: None,
                    boundry,
                })
            }

            [a, b] => {
                let a = mem::replace(a, Box::new(Unhittable));
                let b = mem::replace(b, Box::new(Unhittable));
                let boundry_a = a.bounding_box()?;
                let boundry_b = b.bounding_box()?;

                let (left, right) = if box_compare(&boundry_a, &boundry_b, axis)
                {
                    (a, b)
                } else {
                    (b, a)
                };

                Some(Self {
                    left,
                    right: Some(right),
                    boundry: boundry_a.surrounding_box(&boundry_b),
                })
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
                let left = BvhNode::subdivide_objects(left_half)?;

                let right_half = &mut objects[middle..];
                let right = BvhNode::subdivide_objects(right_half)?;

                let boundry = left.boundry.surrounding_box(&right.boundry);

                Some(Self {
                    left: Box::new(left),
                    right: Some(Box::new(right)),
                    boundry,
                })
            }
        }
    }
}
