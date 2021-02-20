use crate::color::Color;

/// https://www.youtube.com/watch?v=eXU-6_jmw7Q
pub fn to_rgb(mut color: Color, gamma: f32) -> [u8; 3] {
    let inv_gamma = 1.0 / gamma;

    color.x = color.x.powf(inv_gamma);
    color.y = color.y.powf(inv_gamma);
    color.z = color.z.powf(inv_gamma);

    let mut sat: f32 = 1.0;
    let luma = color.dot(&Color {
        x: 0.299,
        y: 0.587,
        z: 0.114,
    });

    for n in 0..3 {
        if color[n] > 1.0 {
            sat = sat.min((luma - 1.0) / (luma - color[n]));
        } else if color[n] < 0.0 {
            sat = sat.min(luma / (luma - color[n]));
        }
    }

    sat = sat.clamp(0.0, 1.0);

    color.x = ((color.x - luma) * sat + luma).clamp(0.0, 1.0);
    color.y = ((color.y - luma) * sat + luma).clamp(0.0, 1.0);
    color.z = ((color.z - luma) * sat + luma).clamp(0.0, 1.0);

    [
        (color.x * (256.0 - 1e-5)) as u8,
        (color.y * (256.0 - 1e-5)) as u8,
        (color.z * (256.0 - 1e-5)) as u8,
    ]
}
