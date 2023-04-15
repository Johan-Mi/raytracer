pub type Color = glam::Vec3A;

/// <https://www.youtube.com/watch?v=eXU-6_jmw7Q>
pub fn to_rgb(color: Color, gamma: f32) -> [u8; 3] {
    let inv_gamma = 1.0 / gamma;

    let color = color.powf(inv_gamma);

    let mut sat: f32 = 1.0;
    let luma = color.dot(Color::new(0.299, 0.587, 0.114));

    for chan in color.to_array() {
        if chan > 1.0 {
            sat = sat.min((luma - 1.0) / (luma - chan));
        } else if chan < 0.0 {
            sat = sat.min(luma / (luma - chan));
        }
    }

    sat = sat.clamp(0.0, 1.0);

    let color = ((color - luma) * sat + luma).clamp(Color::ZERO, Color::ONE);

    (color * (256.0 - 1e-5)).to_array().map(|chan| chan as u8)
}
