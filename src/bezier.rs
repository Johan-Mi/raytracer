use crate::{
    hittable::Hittable,
    materials::Material,
    shapes::{BvhNode, Triangle},
    Point3,
};
use bumpalo::Bump;

fn eval_bezier_curve(points: &[Point3; 4], t: f32) -> Point3 {
    let b0 = (1.0 - t) * (1.0 - t) * (1.0 - t);
    let b1 = 3.0 * t * (1.0 - t) * (1.0 - t);
    let b2 = 3.0 * t * t * (1.0 - t);
    let b3 = t * t * t;

    points[0] * b0 + points[1] * b1 + points[2] * b2 + points[3] * b3
}

pub fn build_bezier_patch<'a>(
    points: &[[Point3; 4]; 4],
    material: &'a Material,
    arena: &'a Bump,
) -> &'a (dyn Hittable + Sync) {
    const DIVISIONS: usize = 24;

    let mut vertices = [[Point3::default(); DIVISIONS + 1]; DIVISIONS + 1];
    for (u, row) in vertices.iter_mut().enumerate() {
        let curve =
            points.map(|p| eval_bezier_curve(&p, u as f32 / DIVISIONS as f32));

        for (v, vert) in row.iter_mut().enumerate() {
            *vert = eval_bezier_curve(&curve, v as f32 / DIVISIONS as f32);
        }
    }

    let mut triangles: Vec<&(dyn Hittable + Sync)> =
        Vec::with_capacity(DIVISIONS * DIVISIONS * 2);

    for u in 0..DIVISIONS {
        for v in 0..DIVISIONS {
            triangles.push(arena.alloc(Triangle {
                points: [
                    vertices[u][v + 1],
                    vertices[u][v],
                    vertices[u + 1][v],
                ],
                material,
            }));
            triangles.push(arena.alloc(Triangle {
                points: [
                    vertices[u][v + 1],
                    vertices[u + 1][v],
                    vertices[u + 1][v + 1],
                ],
                material,
            }));
        }
    }

    BvhNode::subdivide_objects(&mut triangles, arena).unwrap()
}
