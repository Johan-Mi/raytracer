#![warn(missing_docs)]

//! # raytracer
//!
//! A multi-threaded CPU-driven ray tracer based on the [Ray Tracing in One
//! Weekend][1] series by Peter Shirley.  This program implements many of the
//! features from the first two books, along with some new things such as
//! triangles, `.obj` files, Bézier patches and some optimizations. Scenes can
//! be loaded from `.ron` files, which can be written by hand or created with
//! the included Lua API.
//!
//! [1]: https://raytracing.github.io/
//!
//! # Examples
//!
//! For examples of scenes, see the `scenes` directory in this repository.
//!
//! Render a triangulated version of the [Utah teapot][2] in 1080p with 500
//! samples per pixel:
//!
//! [2]: https://en.wikipedia.org/wiki/Utah_teapot
//!
//! ```shell
//! raytracer --width 1920 --height 1080 --samples 500 scenes/teapot.ron
//! ```
//!
//! Shorter versions of the command line parameters are also available:
//!
//! ```shell
//! raytracer -w 1920 -h 1080 -s 500 scenes/teapot.ron
//! ```
//!
//! # Features
//!
//! ## Shapes
//!
//! - Spheres
//! - Planes
//! - Axis aligned rectangles
//! - Cuboids
//! - Triangles
//! - Constant media
//! - Bézier patches
//!
//! ## Transformations of shapes
//!
//! - Translation
//! - Rotation
//!     - Only around the Y axis for now
//!
//! ## Materials
//!
//! - Lambertian (matte) materials
//! - Metal with adjustable fuzz
//! - Dielectric materials with customizable refractive index, such as glass or
//! water
//! - Diffuse lights
//! - Isotropic material that scatters light in any direction
//!     - This can be combined with a constant medium to create fog or colored
//!     glass
//! - Combinations of two materials
//!
//! ## Other features
//!
//! - Multi-threading
//! - Scenes can be loaded from `.ron` files
//! - Lua API to generate said files
//! - Scenes can import basic meshes from `.obj` files
//!
//! # TODO
//!
//! - Textures
//! - Importance sampling
//! - Proper error handling
//!     - Currently, the program just crashes if anything goes wrong.

mod aabb;
mod args;
mod bezier;
mod camera;
mod color;
mod de;
mod hittable;
mod materials;
mod random;
mod ray;
mod raytracer;
mod shapes;

use args::Args;
use bumpalo::Bump;
use clap::Parser;
use de::scene::Scene;
use raytracer::RayTracer;
use shapes::BvhNode;
use std::fs::File;

type Vec3 = glam::Vec3A;
type Point3 = glam::Vec3A;

fn main() {
    let args = Args::parse();

    let file = File::open(&args.infile).unwrap();
    let scene = match ron::de::from_reader::<_, Scene>(file) {
        Ok(scene) => scene,
        Err(err) => {
            eprintln!("{err}");
            return;
        }
    };

    let arena = Bump::new();

    let (mut world, camera, sky_color) = scene.build(&args, &arena);

    let subdivided = BvhNode::subdivide_objects(&mut world, &arena).unwrap();

    let tracer = RayTracer::new(camera, subdivided, sky_color, args);
    tracer.write_ppm();
}
