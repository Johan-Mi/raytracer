mod aabb;
mod args;
mod camera;
mod de;
mod drawable;
mod hittable;
mod materials;
mod math;
mod ray;
mod raytracer;
mod shapes;

use args::Args;
use de::scene::Scene;
use drawable::Drawable;
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
    let (mut world, camera) = scene.build(&args);

    let subdivided = Box::new(BvhNode::subdivide_objects(&mut world).unwrap());

    let tracer = RayTracer::new(camera, subdivided, args);
    tracer.write_ppm(&tracer.args.outfile, tracer.args.quiet);
}
