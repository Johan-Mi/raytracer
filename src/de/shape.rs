use super::{
    material::Material,
    var::Var,
    vec::{Direction, Point},
};
use crate::{
    hittable::Hittable,
    materials::Material as DynMaterial,
    shapes::{
        Cuboid, Plane, RotateY, Sphere, Translate, XYRect, XZRect, YZRect,
    },
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
    XYRect {
        x0: f32,
        x1: f32,
        y0: f32,
        y1: f32,
        k: f32,
        material: Var<Material>,
    },
    XZRect {
        x0: f32,
        x1: f32,
        z0: f32,
        z1: f32,
        k: f32,
        material: Var<Material>,
    },
    YZRect {
        y0: f32,
        y1: f32,
        z0: f32,
        z1: f32,
        k: f32,
        material: Var<Material>,
    },
    Cuboid {
        minimum: Point,
        maximum: Point,
        material: Var<Material>,
    },
    RotateY {
        inner: Box<Shape>,
        angle: f32,
    },
    Translate {
        inner: Box<Shape>,
        offset: Direction,
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
            Shape::XYRect {
                x0,
                x1,
                y0,
                y1,
                k,
                material,
            } => Box::new(XYRect {
                x0,
                x1,
                y0,
                y1,
                k,
                material: material.map(|m| m.into()).resolve(materials),
            }),
            Shape::XZRect {
                x0,
                x1,
                z0,
                z1,
                k,
                material,
            } => Box::new(XZRect {
                x0,
                x1,
                z0,
                z1,
                k,
                material: material.map(|m| m.into()).resolve(materials),
            }),
            Shape::YZRect {
                y0,
                y1,
                z0,
                z1,
                k,
                material,
            } => Box::new(YZRect {
                y0,
                y1,
                z0,
                z1,
                k,
                material: material.map(|m| m.into()).resolve(materials),
            }),
            Shape::Cuboid {
                minimum,
                maximum,
                material,
            } => Box::new(Cuboid::new(
                minimum.into(),
                maximum.into(),
                material.map(|m| m.into()).resolve(materials),
            )),
            Shape::RotateY { inner, angle } => {
                let inner = inner.build(materials);
                Box::new(RotateY::new(inner, angle))
            }
            Shape::Translate { inner, offset } => {
                let inner = inner.build(materials);
                Box::new(Translate {
                    inner,
                    offset: offset.into(),
                })
            }
        }
    }
}
