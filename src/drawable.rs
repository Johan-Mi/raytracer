use super::math::Color;
use rayon::prelude::*;
use std::fs;
use std::io::{BufWriter, Write};

pub trait Drawable {
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;

    fn get_color_at(&self, x: usize, y: usize) -> Color;

    fn write_ppm<P: AsRef<std::path::Path>>(&self, filename: P, quiet: bool)
    where
        Self: Sync,
    {
        let width = self.get_width();
        let height = self.get_height();

        let mut buf = Vec::with_capacity(width * height);

        (0..(width * height))
            .into_par_iter()
            .map(|i| {
                let y = i / width;
                let x = i % width;

                if !quiet && x == 0 {
                    println!("Current row: {}", height - y);
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
        writeln!(wbuf, "P6 {} {} 255", width, height).unwrap();
        for c in buf {
            wbuf.write_all(&c).unwrap();
        }
        wbuf.flush().unwrap();
    }
}
