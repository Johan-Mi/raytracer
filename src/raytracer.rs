use crate::{
    args::Args,
    camera::Camera,
    color::{to_rgb, Color},
    hittable::Hittable,
    ray::Ray,
};
use rand::Rng;
use rayon::prelude::*;
use std::{
    f32, fs,
    io::{BufWriter, Write},
    sync::atomic::{AtomicUsize, Ordering},
    time::Instant,
};

pub const MAX_DEPTH: i32 = 50;

pub struct RayTracer<'a> {
    camera: Camera,
    world: &'a (dyn Hittable + Sync),
    args: Args,
}

impl<'a> RayTracer<'a> {
    pub fn new(
        camera: Camera,
        world: &'a (dyn Hittable + Sync),
        args: Args,
    ) -> Self {
        Self {
            camera,
            world,
            args,
        }
    }

    fn sky_color_at_ray(&self, _ray: &Ray) -> Color {
        Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn color_at_ray(&self, ray: &Ray, depth: i32) -> Color {
        if depth <= 0 {
            Color {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        } else if let Some(hit) = self.world.gets_hit(ray, 0.001, f32::INFINITY)
        {
            let emitted = hit.material.emitted();

            if let Some((scattered, attenuation)) =
                hit.material.scatter(ray, &hit)
            {
                self.color_at_ray(&scattered, depth - 1)
                    .elementwise_mul(&attenuation)
                    + emitted
            } else {
                emitted
            }
        } else {
            self.sky_color_at_ray(ray)
        }
    }

    fn color_at_xy(&self, x: usize, y: usize) -> Color {
        let width = self.args.width;
        let height = self.args.height;

        (0..self.args.samples)
            .map(|_| {
                let mut rng = rand::thread_rng();

                let u =
                    (x as f32 + rng.gen_range(0.0..1.0)) / (width as f32 - 1.0);
                let v = ((height - y) as f32 + rng.gen_range(0.0..1.0))
                    / (height as f32 - 1.0);

                let ray = self.camera.get_ray(u, v);
                self.color_at_ray(&ray, MAX_DEPTH)
            })
            .fold(
                Color {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                |l, r| l + r,
            )
            / self.args.samples as f32
    }

    pub fn write_ppm(&self) {
        let start_time = Instant::now();

        let width = self.args.width;
        let height = self.args.height;

        let mut buf = Vec::with_capacity(width * height);

        let counter = AtomicUsize::new(0);

        (0..(width * height))
            .into_par_iter()
            .map(|i| {
                let y = i / width;
                let x = i % width;

                let curr_count = counter.fetch_add(1, Ordering::Relaxed);
                if !self.args.quiet && i % 2000 == 0 {
                    let percentage =
                        curr_count as f32 / (width * height) as f32 * 100.0;
                    println!("Progress: {:3.3}%", percentage);
                }

                let color = self.color_at_xy(x, y);
                to_rgb(color, self.args.gamma)
            })
            .collect_into_vec(&mut buf);

        let f = fs::File::create(&self.args.outfile).unwrap();
        let mut wbuf = BufWriter::new(f);
        writeln!(wbuf, "P6 {} {} 255", width, height).unwrap();
        for c in buf {
            wbuf.write_all(&c).unwrap();
        }
        wbuf.flush().unwrap();

        let elapsed = start_time.elapsed();
        let millis = elapsed.as_millis();
        let (hours, millis) = (millis / 3600000, millis % 3600000);
        let (minutes, millis) = (millis / 60000, millis % 60000);
        let (seconds, millis) = (millis / 1000, millis % 1000);

        if !self.args.quiet {
            println!(
                "Finished in {}:{:02}:{:02}.{:03}",
                hours, minutes, seconds, millis
            )
        }
    }
}
