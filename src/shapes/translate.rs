use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::Vec3,
};

pub struct Translate<'a> {
    pub inner: &'a (dyn Hittable + Sync),
    pub offset: Vec3,
}

impl Hittable for Translate<'_> {
    fn gets_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_ray = Ray {
            origin: ray.origin - self.offset,
            dir: ray.dir,
        };

        let inner_rec = self.inner.gets_hit(&moved_ray, t_min, t_max)?;

        let p = inner_rec.p + self.offset;
        let normal = inner_rec.normal;
        let material = inner_rec.material;
        let t = inner_rec.t;

        Some(HitRecord::new(p, &normal, material, t, ray))
    }

    fn bounding_box(&self) -> Option<Aabb> {
        let boundary = self.inner.bounding_box()?;

        Some(Aabb {
            minimum: boundary.minimum + self.offset,
            maximum: boundary.maximum + self.offset,
        })
    }
}
