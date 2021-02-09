use super::{
    math::{Point3, Vec3},
    ray::Ray,
};
use std::f32;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(vfov: f32, aspect_ratio: f32) -> Self {
        let theta = vfov.to_radians();
        let h = (theta * 0.5).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let focal_length = 1.0;

        let origin = Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let horizontal = Point3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let vertical = Point3 {
            x: 0.0,
            y: viewport_height,
            z: 0.0,
        };
        let lower_left_corner = origin
            - horizontal * 0.5
            - vertical * 0.5
            - Vec3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            };

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            dir: self.lower_left_corner
                + self.horizontal * u
                + self.vertical * v
                - self.origin,
        }
    }
}
