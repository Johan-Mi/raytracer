use super::{camera::Camera, material::Material, shape::Shape};
use crate::{
    args::Args, camera::Camera as RealCamera, hittable::Hittable,
    shapes::HittableList,
};
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
    pub fn build(self, args: &Args) -> (Box<dyn Hittable + Sync>, RealCamera) {
        let camera = self.camera.build(args);

        let materials = self
            .materials
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect();

        (
            Box::new(HittableList::new(
                self.shapes
                    .into_iter()
                    .map(|s| s.build(&materials))
                    .collect(),
            )),
            camera,
        )
    }
}
