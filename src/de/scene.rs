use super::{camera::Camera, material::Material, shape::Shape};
use crate::{args::Args, camera::Camera as RealCamera, hittable::Hittable};
use bumpalo::Bump;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Scene {
    #[serde(default)]
    materials: HashMap<String, Material>,

    shapes: Vec<Shape>,

    camera: Camera,
}

impl Scene {
    pub fn build<'a>(
        self,
        args: &Args,
        arena: &'a Bump,
    ) -> (std::vec::Vec<&'a (dyn Hittable + Sync)>, RealCamera) {
        let camera = self.camera.build(args);

        let materials = self
            .materials
            .into_iter()
            .map(|(k, v)| (k, v.build(arena)))
            .collect();

        (
            self.shapes
                .into_iter()
                .map(|s| -> &'a (dyn Hittable + Sync) {
                    s.build(&materials, arena)
                })
                .collect(),
            camera,
        )
    }
}
