use super::camera::Camera;
use super::color::Color;
use super::drawable::Drawable;
use super::hittable::Hittable;
use super::ray::Ray;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const ASPECT_RATIO: f32 = WIDTH as f32 / HEIGHT as f32;

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
                r: hit_record.normal.x,
                g: hit_record.normal.y,
                b: hit_record.normal.z,
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
        let v = (HEIGHT - y) as f32 / (HEIGHT as f32 - 1.0);

        let ray = self.camera.get_ray(u, v);
        self.color_at_ray(&ray)
    }
}
