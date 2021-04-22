use super::{
    material::Material,
    var::Var,
    vec::{Direction, Point},
};
use crate::{
    hittable::Hittable,
    materials::Material as DynMaterial,
    shapes::{
        BvhNode, ConstantMedium, Cuboid, Plane, RotateY, Sphere, Translate,
        XyRect, XzRect, YzRect,
    },
};
use bumpalo::Bump;
use serde::Deserialize;
use std::collections::HashMap;

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
    XyRect {
        x0: f32,
        x1: f32,
        y0: f32,
        y1: f32,
        k: f32,
        material: Var<Material>,
    },
    XzRect {
        x0: f32,
        x1: f32,
        z0: f32,
        z1: f32,
        k: f32,
        material: Var<Material>,
    },
    YzRect {
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
    ConstantMedium {
        boundry: Box<Shape>,
        phase_function: Var<Material>,
        density: f32,
    },
    HittableList {
        shapes: Vec<Shape>,
    },
}

impl Shape {
    pub fn build<'a>(
        self,
        materials: &HashMap<String, &'a (dyn DynMaterial + Sync)>,
        arena: &'a Bump,
    ) -> &'a (dyn Hittable + Sync) {
        match self {
            Shape::Sphere {
                center,
                radius,
                material,
            } => arena.alloc(Sphere {
                center: center.into(),
                radius,
                material: material.map(|m| m.build(arena)).resolve(materials),
            }),
            Shape::Plane {
                position,
                normal,
                material,
            } => arena.alloc(Plane {
                pos: position.into(),
                normal: normal.into(),
                material: material.map(|m| m.build(arena)).resolve(materials),
            }),
            Shape::XyRect {
                x0,
                x1,
                y0,
                y1,
                k,
                material,
            } => arena.alloc(XyRect {
                x0,
                x1,
                y0,
                y1,
                k,
                material: material.map(|m| m.build(arena)).resolve(materials),
            }),
            Shape::XzRect {
                x0,
                x1,
                z0,
                z1,
                k,
                material,
            } => arena.alloc(XzRect {
                x0,
                x1,
                z0,
                z1,
                k,
                material: material.map(|m| m.build(arena)).resolve(materials),
            }),
            Shape::YzRect {
                y0,
                y1,
                z0,
                z1,
                k,
                material,
            } => arena.alloc(YzRect {
                y0,
                y1,
                z0,
                z1,
                k,
                material: material.map(|m| m.build(arena)).resolve(materials),
            }),
            Shape::Cuboid {
                minimum,
                maximum,
                material,
            } => arena.alloc(Cuboid::new(
                minimum.into(),
                maximum.into(),
                material.map(|m| m.build(arena)).resolve(materials),
                arena,
            )),
            Shape::RotateY { inner, angle } => {
                let inner = inner.build(materials, arena);
                arena.alloc(RotateY::new(inner, angle))
            }
            Shape::Translate { inner, offset } => {
                let inner = inner.build(materials, arena);
                arena.alloc(Translate {
                    inner,
                    offset: offset.into(),
                })
            }
            Shape::ConstantMedium {
                boundry,
                phase_function,
                density,
            } => arena.alloc(ConstantMedium {
                boundry: boundry.build(materials, arena),
                phase_function: phase_function
                    .map(|m| m.build(arena))
                    .resolve(materials),
                neg_inv_density: -1.0 / density,
            }),
            Shape::HittableList { shapes } => {
                let mut shapes = shapes
                    .into_iter()
                    .map(|s| s.build(materials, arena))
                    .collect::<Vec<_>>();
                BvhNode::subdivide_objects(&mut shapes, arena).unwrap()
            }
        }
    }
}
