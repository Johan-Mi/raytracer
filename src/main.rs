mod camera;
mod drawable;
mod hittable;
mod materials;
mod math;
mod ray;
mod raytracer;
mod shapes;

use drawable::Drawable;
use materials::Lambertian;
use math::{Color, Point3};
use raytracer::RayTracer;
use shapes::{HittableList, Sphere};
use std::rc::Rc;

fn main() {
    let mat = Rc::new(Lambertian {
        albedo: Color {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
    });

    let world = Box::new(HittableList::new(vec![
        Box::new(Sphere {
            center: Point3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
            material: mat.clone(),
        }),
        Box::new(Sphere {
            center: Point3 {
                x: 0.0,
                y: -100.5,
                z: -1.0,
            },
            radius: 100.0,
            material: mat,
        }),
    ]));

    let tracer = RayTracer::new(world);
    tracer.write_ppm("image.ppm");
}
