use super::vec::{Direction, Point};
use crate::{args::Args, camera::Camera as RealCamera};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Camera {
    look_from: Point,
    look_at: Point,
    vup: Direction,
    vfov: f32,
    aperture: f32,
    focus_dist: f32,
}

impl Camera {
    pub fn build(self, args: &Args) -> RealCamera {
        RealCamera::new(
            self.look_from.into(),
            self.look_at.into(),
            self.vup.into(),
            self.vfov,
            args.width as f32 / args.height as f32,
            self.aperture,
            self.focus_dist,
        )
    }
}
