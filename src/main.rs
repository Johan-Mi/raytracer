mod camera;
mod drawable;
mod hittable;
mod materials;
mod math;
mod ray;
mod raytracer;
mod shapes;

use drawable::Drawable;
use materials::{Lambertian, Metal};
use math::{Color, Point3};
use raytracer::RayTracer;
use shapes::{HittableList, Sphere};
use std::rc::Rc;

fn main() {
    let material_ground = Rc::new(Lambertian {
        albedo: Color {
            x: 0.8,
            y: 0.8,
            z: 0.0,
        },
    });
    let material_center = Rc::new(Lambertian {
        albedo: Color {
            x: 0.7,
            y: 0.3,
            z: 0.3,
        },
    });
    let material_left = Rc::new(Metal {
        albedo: Color {
            x: 0.8,
            y: 0.8,
            z: 0.8,
        },
        fuzz: 0.3,
    });
    let material_right = Rc::new(Metal {
        albedo: Color {
            x: 0.8,
            y: 0.6,
            z: 0.2,
        },
        fuzz: 1.0,
    });

    let world = Box::new(HittableList::new(vec![
        Box::new(Sphere {
            center: Point3 {
                x: 0.0,
                y: -100.5,
                z: -1.0,
            },
            radius: 100.0,
            material: material_ground,
        }),
        Box::new(Sphere {
            center: Point3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
            material: material_center,
        }),
        Box::new(Sphere {
            center: Point3 {
                x: -1.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
            material: material_left,
        }),
        Box::new(Sphere {
            center: Point3 {
                x: 1.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
            material: material_right,
        }),
    ]));

    let tracer = RayTracer::new(world);
    tracer.write_ppm("image.ppm");
}
