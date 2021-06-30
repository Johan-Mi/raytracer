use vec3::Vec3;

pub type Color = Vec3<f32>;

/// <https://www.youtube.com/watch?v=eXU-6_jmw7Q>
pub fn to_rgb(mut color: Color, gamma: f32) -> [u8; 3] {
    let inv_gamma = 1.0 / gamma;

    color = color.map(|chan| chan.powf(inv_gamma));

    let mut sat: f32 = 1.0;
    let luma = color.dot(&Color {
        x: 0.299,
        y: 0.587,
        z: 0.114,
    });

    for chan in color {
        if chan > 1.0 {
            sat = sat.min((luma - 1.0) / (luma - chan));
        } else if chan < 0.0 {
            sat = sat.min(luma / (luma - chan));
        }
    }

    sat = sat.clamp(0.0, 1.0);

    color = color.map(|chan| ((chan - luma) * sat + luma).clamp(0.0, 1.0));

    let rgb = color.map(|chan| (chan * (256.0 - 1e-5)) as u8);

    [rgb.x, rgb.y, rgb.z]
}
