use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    math::{Point3, Vec3},
    ray::Ray,
};

pub struct RotateY {
    inner: Box<dyn Hittable + Sync>,
    sin_theta: f32,
    cos_theta: f32,
    boundry: Option<AABB>,
}

impl RotateY {
    pub fn new(inner: Box<dyn Hittable + Sync>, angle: f32) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let boundry = if let Some(boundry) = inner.bounding_box() {
            let mut min = Point3 {
                x: f32::INFINITY,
                y: f32::INFINITY,
                z: f32::INFINITY,
            };
            let mut max = Point3 {
                x: f32::NEG_INFINITY,
                y: f32::NEG_INFINITY,
                z: f32::NEG_INFINITY,
            };

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let i_float = i as f32;
                        let j_float = j as f32;
                        let k_float = k as f32;

                        let x = i_float * boundry.maximum.x
                            + (1.0 - i_float) * boundry.minimum.x;
                        let y = j_float * boundry.maximum.x
                            + (1.0 - j_float) * boundry.minimum.y;
                        let z = k_float * boundry.maximum.x
                            + (1.0 - k_float) * boundry.minimum.z;

                        let new_x = cos_theta * x + sin_theta * z;
                        let new_z = -sin_theta * x + cos_theta * z;

                        let tester = Vec3 {
                            x: new_x,
                            y,
                            z: new_z,
                        };

                        min.x = min.x.min(tester.x);
                        max.x = max.x.max(tester.x);
                        min.y = min.y.min(tester.y);
                        max.y = max.y.max(tester.y);
                        min.z = min.z.min(tester.z);
                        max.z = max.z.max(tester.z);
                    }
                }
            }

            Some(AABB {
                minimum: min,
                maximum: max,
            })
        } else {
            None
        };

        Self {
            inner,
            sin_theta,
            cos_theta,
            boundry,
        }
    }
}

impl Hittable for RotateY {
    fn gets_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.dir;

        origin.x =
            self.cos_theta * ray.origin.x - self.sin_theta * ray.origin.z;
        origin.z =
            self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.z;

        direction.x = self.cos_theta * ray.dir.x - self.sin_theta * ray.dir.z;
        direction.z = self.sin_theta * ray.dir.x + self.cos_theta * ray.dir.z;

        let rotated_ray = Ray {
            origin,
            dir: direction,
        };

        let inner_rec = self.inner.gets_hit(&rotated_ray, t_min, t_max)?;

        let mut p = inner_rec.p;
        let mut normal = inner_rec.normal;

        p.x = self.cos_theta * inner_rec.p.x + self.sin_theta * inner_rec.p.z;
        p.z = -self.sin_theta * inner_rec.p.x + self.cos_theta * inner_rec.p.z;

        normal.x = self.cos_theta * inner_rec.normal.x
            - self.sin_theta * inner_rec.normal.z;
        normal.z = self.sin_theta * inner_rec.normal.x
            + self.cos_theta * inner_rec.normal.z;

        let mut rec = HitRecord {
            p,
            normal: Vec3::default(),
            material: inner_rec.material,
            t: inner_rec.t,
            front_face: bool::default(),
        };

        rec.set_face_normal(&rotated_ray, &normal);

        Some(rec)
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.boundry.clone()
    }
}