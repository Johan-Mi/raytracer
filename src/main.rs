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
use std::fs::File;
use structopt::StructOpt;

fn main() {
    let args = Args::from_args();

    let file = File::open("scene.ron").unwrap();
    let scene = match ron::de::from_reader::<_, Scene>(file) {
        Ok(scene) => scene,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };
    let (world, camera) = scene.build(&args);

    let tracer = RayTracer::new(camera, world, args);
    tracer.write_ppm(&tracer.args.outfile, tracer.args.quiet);
}
