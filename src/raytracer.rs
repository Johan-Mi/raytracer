use super::camera::Camera;
use super::constants::{HEIGHT, MAX_DEPTH, SAMPLES_PER_PIXEL, WIDTH};
use super::drawable::Drawable;
use super::hittable::Hittable;
use super::math::Color;
use super::ray::Ray;
use rand::Rng;
use std::f32;

pub struct RayTracer {
    camera: Camera,
    world: Box<dyn Hittable + Sync>,
}

impl RayTracer {
    pub fn new(camera: Camera, world: Box<dyn Hittable + Sync>) -> Self {
        Self { camera, world }
    }

    fn sky_color_at_ray(&self, ray: &Ray) -> Color {
        const SKY_COLOR_TOP: Color = Color {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        };
        const SKY_COLOR_BOTTOM: Color = Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let t = (0.5 * (ray.dir.y / ray.dir.x.hypot(ray.dir.z) + 1.0))
            .clamp(0.0, 1.0);

        Color::lerp(&SKY_COLOR_TOP, &SKY_COLOR_BOTTOM, t)
    }

    fn color_at_ray(&self, ray: &Ray, depth: i32) -> Color {
        if depth <= 0 {
            Color {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        } else if let Some(hit) = self.world.gets_hit(ray, 0.001, f32::INFINITY)
        {
            if let Some((scattered, attenuation)) =
                hit.material.scatter(ray, &hit)
            {
                self.color_at_ray(&scattered, depth - 1)
                    .elementwise_mul(&attenuation)
            } else {
                Color {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }
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
        (0..SAMPLES_PER_PIXEL)
            .map(|_| {
                let mut rng = rand::thread_rng();

                let u =
                    (x as f32 + rng.gen_range(0.0..1.0)) / (WIDTH as f32 - 1.0);
                let v = ((HEIGHT - y) as f32 + rng.gen_range(0.0..1.0))
                    / (HEIGHT as f32 - 1.0);

                let ray = self.camera.get_ray(u, v);
                self.color_at_ray(&ray, MAX_DEPTH)
            })
            .fold(
                Color {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                |l, r| l + r,
            )
            / SAMPLES_PER_PIXEL as f32
    }
}
