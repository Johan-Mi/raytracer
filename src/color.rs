use std::ops;

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

impl ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl ops::Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Self::Output {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}
