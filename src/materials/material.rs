use crate::{hittable::HitRecord, math::Color, ray::Ray};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;

    fn emitted(&self) -> Color {
        Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
