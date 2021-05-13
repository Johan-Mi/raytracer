use crate::{color::Color, hittable::HitRecord, materials::Material, ray::Ray};

pub struct DiffuseLight {
    pub color: Color,
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self) -> Color {
        self.color
    }
}
