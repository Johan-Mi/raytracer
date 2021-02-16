#!/usr/bin/env python

from random import uniform
from math import sqrt


def vec_length(v):
    return sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2])


def main():
    materials = {}
    shapes = []

    ground_material = "Lambertian ( albedo: (0.5, 0.5, 0.5), )"
    shapes.append(
        'Sphere ( center: ( x: 0, y: -1000.0, z: 0.0), radius: 1000.0, material: "ground" )'
    )
    materials["ground"] = ground_material

    for a in range(-11, 11):
        for b in range(-11, 11):
            choose_mat = uniform(0.0, 1.0)
            center = (a + uniform(0.0, 0.9), 0.2, b + uniform(0.0, 0.9))

            if vec_length((center[0] - 4.0, center[1] - 0.2, center[2])) > 0.9:
                center_str = f"( x: {center[0]}, y: {center[1]}, z: {center[2]} )"

                if choose_mat < 0.8:
                    albedo = tuple(
                        uniform(0.0, 1.0) * uniform(0.0, 1.0)
                        for _ in range(3))
                    sphere_material = f'{{ "Lambertian": ( albedo: {albedo} ) }}'
                elif choose_mat < 0.95:
                    albedo = tuple(uniform(0.5, 1.0) for _ in range(3))
                    fuzz = uniform(0.0, 0.5)
                    sphere_material = f'{{ "Metal": ( albedo: {albedo}, fuzz: {fuzz} ) }}'
                else:
                    sphere_material = f'{{ "Dielectric": ( ir: 1.5 ) }}'

                shapes.append(
                    f"Sphere ( center: {center_str}, radius: 0.2, material: {sphere_material} )"
                )

    material1 = f'Dielectric ( ir: 1.5 )'
    shapes.append(
        'Sphere ( center: ( x: 0.0, y: 1.0, z: 0.0), radius: 1.0, material: "material1" )'
    )
    materials["material1"] = material1

    material2 = f'Lambertian ( albedo: ( 0.4, 0.2, 0.1 ) )'
    shapes.append(
        'Sphere ( center: ( x: -4.0, y: 1.0, z: 0.0), radius: 1.0, material: "material2" )'
    )
    materials["material2"] = material2

    material3 = f'Metal ( albedo: ( 0.7, 0.6, 0.5 ), fuzz: 0.0 )'
    shapes.append(
        'Sphere ( center: ( x: 4.0, y: 1.0, z: 0.0), radius: 1.0, material: "material3" )'
    )
    materials["material3"] = material3

    materials_str = ", ".join(f'"{k}": {v}' for k, v in materials.items())
    materials_str = f"{{ {materials_str} }}"

    shapes_str = ", ".join(shapes)
    shapes_str = f"[{shapes_str}]"

    print(f"""Scene (
    camera: (
        look_from: (
            x: 13.0,
            y: 2.0,
            z: 3.0,
        ),
        look_at: (
            x: 0.0,
            y: 0.0,
            z: 0.0,
        ),
        vup: (
            x: 0.0,
            y: 1.0,
            z: 0.0,
        ),
        vfov: 20.0,
        focus_dist: 10.0,
        aperture: 0.1,
    ),
    materials: {materials_str},
    shapes: {shapes_str},
)""")


if __name__ == "__main__":
    main()
