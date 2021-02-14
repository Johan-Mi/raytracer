// mod args;
// mod camera;
// mod drawable;
// mod hittable;
// mod materials;
// mod math;
// mod ray;
// mod raytracer;
// mod shapes;
mod de;

// use args::Args;
// use camera::Camera;
// use drawable::Drawable;
// use hittable::Hittable;
// use materials::{Dielectric, Lambertian, Material, Metal};
// use math::{Color, Point3, Vec3};
// use rand::Rng;
// use raytracer::RayTracer;
// use shapes::{HittableList, Sphere};
// use std::sync::Arc;
// use structopt::StructOpt;
use de::scene::Scene;
use std::fs::File;

/*
fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();

    let mut world: Vec<Box<dyn Hittable + Sync>> = Vec::new();

    let ground_material: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian {
            albedo: Color {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
        });
    world.push(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        material: ground_material,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0..1.0);
            let center = Point3 {
                x: a as f32 + 0.9 * rng.gen_range(0.0..1.0),
                y: 0.2,
                z: b as f32 + 0.9 * rng.gen_range(0.0..1.0),
            };

            if (center
                - Point3 {
                    x: 4.0,
                    y: 0.2,
                    z: 0.0,
                })
            .len()
                > 0.9
            {
                let sphere_material: Arc<dyn Material + Sync + Send>;

                if choose_mat < 0.8 {
                    let albedo = Color {
                        x: rng.gen_range(0.0..1.0),
                        y: rng.gen_range(0.0..1.0),
                        z: rng.gen_range(0.0..1.0),
                    }
                    .elementwise_mul(&Color {
                        x: rng.gen_range(0.0..1.0),
                        y: rng.gen_range(0.0..1.0),
                        z: rng.gen_range(0.0..1.0),
                    });
                    sphere_material = Arc::new(Lambertian { albedo });
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material,
                    }));
                } else if choose_mat < 0.95 {
                    let albedo = Color {
                        x: rng.gen_range(0.5..1.0),
                        y: rng.gen_range(0.5..1.0),
                        z: rng.gen_range(0.5..1.0),
                    };
                    let fuzz = rng.gen_range(0.0..0.5);
                    sphere_material = Arc::new(Metal { albedo, fuzz });
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material,
                    }));
                } else {
                    sphere_material = Arc::new(Dielectric { ir: 1.5 });
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material,
                    }));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric { ir: 1.5 });
    world.push(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: material1,
    }));

    let material2 = Arc::new(Lambertian {
        albedo: Color {
            x: 0.4,
            y: 0.2,
            z: 0.1,
        },
    });
    world.push(Box::new(Sphere {
        center: Point3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: material2,
    }));

    let material3 = Arc::new(Metal {
        albedo: Color {
            x: 0.7,
            y: 0.6,
            z: 0.5,
        },
        fuzz: 0.0,
    });
    world.push(Box::new(Sphere {
        center: Point3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: material3,
    }));

    HittableList::new(world)
}
*/

fn main() {
    /*
    let args = Args::from_args();

    let world = Box::new(random_scene());

    let lookfrom = Point3 {
        x: 13.0,
        y: 2.0,
        z: 3.0,
    };
    let lookat = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let vup = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        args.width as f32 / args.height as f32,
        aperture,
        dist_to_focus,
    );

    let tracer = RayTracer::new(camera, world, args);
    tracer.write_ppm(&tracer.args.outfile, tracer.args.quiet);
    */

    let file = File::open("scene.ron").unwrap();
    match ron::de::from_reader::<_, Scene>(file) {
        Ok(scene) => {
            println!("{:#?}", scene);
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}
