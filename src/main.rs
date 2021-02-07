mod color;
mod drawable;
mod math;
mod ray;
mod raytracer;
mod viewport;

use drawable::Drawable;
use raytracer::RayTracer;

fn main() {
    let tracer = RayTracer::new();
    tracer.write_ppm("image.ppm");
}
