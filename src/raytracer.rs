use super::camera::Camera;
use super::drawable::Drawable;
use super::hittable::Hittable;
use super::math::Color;
use super::ray::Ray;
use rand::Rng;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const ASPECT_RATIO: f32 = WIDTH as f32 / HEIGHT as f32;
const SAMPLES_PER_PIXEL: usize = 50;

pub struct RayTracer {
    camera: Camera,
    world: Box<dyn Hittable>,
}

impl RayTracer {
    pub fn new(world: Box<dyn Hittable>) -> Self {
        let camera = Camera::new(90.0, ASPECT_RATIO);

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

        let t = 0.5 * (ray.dir.y / ray.dir.z + 1.0);

        Color::lerp(&SKY_COLOR_BOTTOM, &SKY_COLOR_TOP, t)
    }

    fn color_at_ray(&self, ray: &Ray) -> Color {
        if let Some(hit_record) = self.world.gets_hit(ray, 0.0, 100.0) {
            hit_record.normal
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
                self.color_at_ray(&ray)
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
