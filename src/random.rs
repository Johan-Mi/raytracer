use crate::Vec3;
use fastrand::Rng;

pub fn unit(rng: &Rng) -> Vec3 {
    in_unit_sphere(rng).normalize()
}

pub fn in_unit_sphere(rng: &Rng) -> Vec3 {
    loop {
        let p = Vec3::new(rng.f32(), rng.f32(), rng.f32()) * 2.0 - 1.0;

        if p.length_squared() < 1.0 {
            break p;
        }
    }
}

pub fn in_unit_disk(rng: &Rng) -> Vec3 {
    loop {
        let p = Vec3::new(
            rng.f32().mul_add(2.0, -1.0),
            rng.f32().mul_add(2.0, -1.0),
            0.0,
        );

        if p.length_squared() < 1.0 {
            break p;
        }
    }
}
