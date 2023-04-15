use crate::Vec3;
use rand::Rng;

pub fn unit(rng: &mut impl Rng) -> Vec3 {
    in_unit_sphere(rng).normalize()
}

pub fn in_unit_sphere(rng: &mut impl Rng) -> Vec3 {
    loop {
        let p = Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );

        if p.length_squared() < 1.0 {
            break p;
        }
    }
}

pub fn in_unit_disk(rng: &mut impl Rng) -> Vec3 {
    loop {
        let p =
            Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);

        if p.length_squared() < 1.0 {
            break p;
        }
    }
}
