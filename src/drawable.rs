use super::color::Color;
use std::fs;
use std::io::Write;

pub trait Drawable {
    const WIDTH: usize;
    const HEIGHT: usize;

    fn get_color_at(&self, x: usize, y: usize) -> Color;

    fn write_ppm<P: AsRef<std::path::Path>>(&self, filename: P) {
        let mut f = fs::File::create(filename).unwrap();
        writeln!(f, "P6 {} {} 255", Self::WIDTH, Self::HEIGHT).unwrap();
        for y in 0..Self::HEIGHT {
            for x in 0..Self::WIDTH {
                let color = self.get_color_at(x, y);
                f.write(&[color.r, color.g, color.b]).unwrap();
            }
        }
    }
}
