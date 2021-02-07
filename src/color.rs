#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn lerp(&self, other: &Color, t: f32) -> Color {
        Color {
            r: (1.0 - t) * self.r + t * other.r,
            g: (1.0 - t) * self.g + t * other.g,
            b: (1.0 - t) * self.b + t * other.b,
        }
    }
}
