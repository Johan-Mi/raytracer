mod aabb;
mod args;
mod camera;
mod color;
mod de;
mod hittable;
mod materials;
mod point3;
mod ray;
mod raytracer;
mod shapes;
mod vec3;

use args::Args;
use bumpalo::Bump;
use de::scene::Scene;
use raytracer::RayTracer;
use shapes::BvhNode;
use std::fs::File;
use structopt::StructOpt;

fn main() {
    let args = Args::from_args();

    let file = File::open(&args.infile).unwrap();
    let scene = match ron::de::from_reader::<_, Scene>(file) {
        Ok(scene) => scene,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let arena = Bump::new();

    let (mut world, camera) = scene.build(&args, &arena);

    let subdivided = BvhNode::subdivide_objects(&mut world, &arena).unwrap();

    let tracer = RayTracer::new(camera, subdivided, args);
    tracer.write_ppm(&tracer.args.outfile, tracer.args.quiet);
}
