use super::math::Color;
use std::fs;
use std::io::{BufWriter, Write};

pub trait Drawable {
    const WIDTH: usize;
    const HEIGHT: usize;

    fn get_color_at(&self, x: usize, y: usize) -> Color;

    fn write_ppm<P: AsRef<std::path::Path>>(&self, filename: P) {
        let f = fs::File::create(filename).unwrap();
        let mut buf = BufWriter::new(f);

        writeln!(buf, "P6 {} {} 255", Self::WIDTH, Self::HEIGHT).unwrap();
        for y in 0..Self::HEIGHT {
            println!("Rows remaining: {}", Self::HEIGHT - y);
            for x in 0..Self::WIDTH {
                let color = self.get_color_at(x, y);
                let x = (color.x.sqrt() * 255.0) as u8;
                let y = (color.y.sqrt() * 255.0) as u8;
                let z = (color.z.sqrt() * 255.0) as u8;
                buf.write_all(&[x, y, z]).unwrap();
            }
        }

        buf.flush().unwrap();
    }
}
