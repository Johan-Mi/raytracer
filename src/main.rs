mod color;
mod drawable;
mod hittable;
mod math;
mod ray;
mod raytracer;
mod shapes;
mod viewport;

use drawable::Drawable;
use math::Point3;
use raytracer::RayTracer;
use shapes::Sphere;

fn main() {
    let world = Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    });

    let tracer = RayTracer::new(world);
    tracer.write_ppm("image.ppm");
}
