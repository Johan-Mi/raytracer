use super::math::Color;
use rayon::prelude::*;
use std::fs;
use std::io::{BufWriter, Write};

pub trait Drawable {
    const WIDTH: usize;
    const HEIGHT: usize;

    fn get_color_at(&self, x: usize, y: usize) -> Color;

    fn write_ppm<P: AsRef<std::path::Path>>(&self, filename: P)
    where
        Self: Sync,
    {
        let mut buf = Vec::with_capacity(Self::WIDTH * Self::HEIGHT);

        (0..(Self::WIDTH * Self::HEIGHT))
            .into_par_iter()
            .map(|i| {
                let y = i / Self::WIDTH;
                let x = i % Self::WIDTH;

                if x == 0 {
                    println!("Current row: {}", Self::HEIGHT - y);
                }

                let color = self.get_color_at(x, y);
                let x = (color.x.sqrt() * 255.0) as u8;
                let y = (color.y.sqrt() * 255.0) as u8;
                let z = (color.z.sqrt() * 255.0) as u8;
                [x, y, z]
            })
            .collect_into_vec(&mut buf);

        let f = fs::File::create(filename).unwrap();
        let mut wbuf = BufWriter::new(f);
        writeln!(wbuf, "P6 {} {} 255", Self::WIDTH, Self::HEIGHT).unwrap();
        for c in buf {
            wbuf.write_all(&c).unwrap();
        }
        wbuf.flush().unwrap();
    }
}
