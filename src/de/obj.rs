use crate::{
    hittable::Hittable,
    materials::Material,
    point3::Point3,
    shapes::{BvhNode, Triangle},
};
use bumpalo::Bump;
use std::{
    convert::TryInto,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn parse_obj_file<'a, P>(
    path: P,
    material: &'a (dyn Material + Sync),
    arena: &'a Bump,
) -> &'a (dyn Hittable + Sync)
where
    P: AsRef<Path>,
{
    let file = File::open(path).unwrap();
    let file = BufReader::new(file);

    let mut vertices = Vec::new();
    let mut triangles: Vec<&(dyn Hittable + Sync)> = Vec::new();

    for line in file.lines() {
        let line = line.unwrap();

        let parts = line.split_whitespace().collect::<Vec<_>>();
        match parts.split_first() {
            Some((&"v", coords)) => {
                let [x, y, z]: [&str; 3] = coords.try_into().unwrap();
                let x = x.parse().unwrap();
                let y = y.parse().unwrap();
                let z = z.parse().unwrap();

                vertices.push(Point3 { x, y, z });
            }

            Some((&"f", verts)) => {
                let verts = verts
                    .iter()
                    .map(|s| match s.find('/') {
                        Some(first_slash) => &s[..first_slash],
                        None => s,
                    })
                    .map(|s| s.parse::<usize>().unwrap() - 1)
                    .collect::<Vec<_>>();

                match verts[..] {
                    [p0, p1, p2] => {
                        // 3 points make a triangle

                        triangles.push(arena.alloc(Triangle {
                            points: [vertices[p0], vertices[p1], vertices[p2]],
                            material,
                        }));
                    }

                    [p0, p1, p2, p3] => {
                        // 4 points, subdivide a quad into 2 triangles

                        triangles.push(arena.alloc(Triangle {
                            points: [vertices[p0], vertices[p1], vertices[p2]],
                            material,
                        }));

                        triangles.push(arena.alloc(Triangle {
                            points: [vertices[p0], vertices[p2], vertices[p3]],
                            material,
                        }));
                    }

                    _ => {
                        unimplemented!()
                    }
                }
            }

            _ => {}
        }
    }

    BvhNode::subdivide_objects(&mut triangles, arena).unwrap()
}
