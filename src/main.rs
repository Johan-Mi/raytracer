mod camera;
mod constants;
mod drawable;
mod hittable;
mod materials;
mod math;
mod ray;
mod raytracer;
mod shapes;

use camera::Camera;
use constants::ASPECT_RATIO;
use drawable::Drawable;
use materials::{Dielectric, Lambertian, Metal};
use math::{Color, Point3, Vec3};
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
            x: 0.1,
            y: 0.2,
            z: 0.5,
        },
    });
    let material_left = Rc::new(Dielectric { ir: 1.5 });
    let material_right = Rc::new(Metal {
        albedo: Color {
            x: 0.8,
            y: 0.6,
            z: 0.2,
        },
        fuzz: 0.0,
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
            material: material_left.clone(),
        }),
        Box::new(Sphere {
            center: Point3 {
                x: -1.0,
                y: 0.0,
                z: -1.0,
            },
            radius: -0.4,
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

    let lookfrom = Point3 {
        x: 3.0,
        y: 3.0,
        z: 2.0,
    };
    let lookat = Point3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let vup = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let dist_to_focus = (lookfrom - lookat).len();
    let aperture = 2.0;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    let tracer = RayTracer::new(camera, world);
    tracer.write_ppm("image.ppm");
}
