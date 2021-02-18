use super::args::Args;
use super::camera::Camera;
use super::drawable::Drawable;
use super::hittable::Hittable;
use super::math::Color;
use super::ray::Ray;
use rand::Rng;
use std::f32;

pub const MAX_DEPTH: i32 = 50;

pub struct RayTracer<'a> {
    camera: Camera,
    world: &'a (dyn Hittable + Sync),
    pub args: Args,
}

impl<'a> RayTracer<'a> {
    pub fn new(
        camera: Camera,
        world: &'a (dyn Hittable + Sync),
        args: Args,
    ) -> Self {
        Self {
            camera,
            world,
            args,
        }
    }

    fn sky_color_at_ray(&self, _ray: &Ray) -> Color {
        Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
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
                let emitted = hit.material.emitted();

                self.color_at_ray(&scattered, depth - 1)
                    .elementwise_mul(&attenuation)
                    + emitted
            } else {
                hit.material.emitted()
            }
        } else {
            self.sky_color_at_ray(ray)
        }
    }
}

impl Drawable for RayTracer<'_> {
    fn get_width(&self) -> usize {
        self.args.width
    }

    fn get_height(&self) -> usize {
        self.args.height
    }

    fn get_color_at(&self, x: usize, y: usize) -> Color {
        let width = self.get_width();
        let height = self.get_height();

        (0..self.args.samples)
            .map(|_| {
                let mut rng = rand::thread_rng();

                let u =
                    (x as f32 + rng.gen_range(0.0..1.0)) / (width as f32 - 1.0);
                let v = ((height - y) as f32 + rng.gen_range(0.0..1.0))
                    / (height as f32 - 1.0);

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
            / self.args.samples as f32
    }
}
