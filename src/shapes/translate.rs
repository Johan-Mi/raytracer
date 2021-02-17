use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    math::Vec3,
    ray::Ray,
};

pub struct Translate {
    pub inner: Box<dyn Hittable + Sync>,
    pub offset: Vec3,
}

impl Hittable for Translate {
    fn gets_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_ray = Ray {
            origin: ray.origin - self.offset,
            dir: ray.dir,
        };

        let mut rec = self.inner.gets_hit(&moved_ray, t_min, t_max)?;
        rec.p += self.offset;
        let normal = rec.normal;
        rec.set_face_normal(&moved_ray, &normal);

        Some(rec)
    }

    fn bounding_box(&self) -> Option<AABB> {
        let boundry = self.inner.bounding_box()?;

        Some(AABB {
            minimum: boundry.minimum + self.offset,
            maximum: boundry.maximum + self.offset,
        })
    }
}
