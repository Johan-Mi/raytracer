use super::color::Color;
use super::drawable::Drawable;
use super::hittable::Hittable;
use super::math::{Point3, Vec3};
use super::ray::Ray;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const ASPECT_RATIO: f32 = WIDTH as f32 / HEIGHT as f32;

pub struct RayTracer {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,

    world: Box<dyn Hittable>,
}

impl RayTracer {
    pub fn new(world: Box<dyn Hittable>) -> Self {
        let viewport_width = 2.0;
        let viewport_height = viewport_width / ASPECT_RATIO;

        let focal_length = 1.0;

        let origin = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        let horizontal = Vec3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };

        let vertical = Vec3 {
            x: 0.0,
            y: viewport_height,
            z: 0.0,
        };

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: focal_length,
                },
            world,
        }
    }

    fn sky_color_at_ray(&self, ray: &Ray) -> Color {
        const SKY_COLOR_TOP: Color = Color {
            r: 0.5,
            g: 0.7,
            b: 1.0,
        };
        const SKY_COLOR_BOTTOM: Color = Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        };

        let t = 0.5 * (ray.dir.y / ray.dir.z + 1.0);

        Color::lerp(&SKY_COLOR_BOTTOM, &SKY_COLOR_TOP, t)
    }

    fn color_at_ray(&self, ray: &Ray) -> Color {
        if let Some(hit_record) = self.world.gets_hit(ray, 0.0, 100.0) {
            Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
            }
        } else {
            self.sky_color_at_ray(ray)
        }
    }
}

impl Drawable for RayTracer {
    const WIDTH: usize = WIDTH;
    const HEIGHT: usize = HEIGHT;

    fn get_color_at(&self, x: usize, y: usize) -> Color {
        let u = x as f32 / (WIDTH as f32 - 1.0);
        let v = y as f32 / (HEIGHT as f32 - 1.0);

        let ray = Ray {
            origin: self.origin,
            dir: self.lower_left_corner
                + self.horizontal * u
                + self.vertical * v
                - self.origin,
        };

        self.color_at_ray(&ray)
    }
}
