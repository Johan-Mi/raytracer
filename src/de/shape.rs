use super::{
    material::Material,
    var::Var,
    vec::{Direction, Point},
};
use crate::{
    hittable::Hittable,
    materials::Material as DynMaterial,
    shapes::{Plane, Sphere},
};
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Deserialize)]
pub enum Shape {
    Sphere {
        center: Point,
        radius: f32,
        material: Var<Material>,
    },
    Plane {
        position: Point,
        normal: Direction,
        material: Var<Material>,
    },
}

impl Shape {
    pub fn build(
        self,
        materials: &HashMap<String, Arc<dyn DynMaterial + Send + Sync>>,
    ) -> Box<dyn Hittable + Sync> {
        match self {
            Shape::Sphere {
                center,
                radius,
                material,
            } => Box::new(Sphere {
                center: center.into(),
                radius,
                material: material.map(|m| m.into()).resolve(materials),
            }),
            Shape::Plane {
                position,
                normal,
                material,
            } => Box::new(Plane {
                pos: position.into(),
                normal: normal.into(),
                material: material.map(|m| m.into()).resolve(materials),
            }),
        }
    }
}
