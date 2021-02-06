use super::color::Color;
use super::drawable::Drawable;

const WIDTH: usize = 64;
const HEIGHT: usize = 64;

pub struct Checkerboard {}

impl Checkerboard {
    pub fn new() -> Self {
        Self {}
    }
}

impl Drawable for Checkerboard {
    const WIDTH: usize = WIDTH;
    const HEIGHT: usize = HEIGHT;

    fn get_color_at(&self, x: usize, y: usize) -> Color {
        if x % 2 == y % 2 {
            Color { r: 255, g: 0, b: 0 }
        } else {
            Color { r: 0, g: 255, b: 0 }
        }
    }
}
