use crate::point3::Point3;
use vec3::Vec3;

pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3<f32>,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.dir * t
    }
}
