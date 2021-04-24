#!/usr/bin/lua

require 'lua.raytracer.scene'
require 'lua.raytracer.camera'
require 'lua.raytracer.point'
require 'lua.raytracer.direction'
require 'lua.raytracer.color'
require 'lua.raytracer.shapes.sphere'
require 'lua.raytracer.shapes.cuboid'
require 'lua.raytracer.shapes.rect'
require 'lua.raytracer.shapes.medium'
require 'lua.raytracer.shapes.list'
require 'lua.raytracer.shapes.translate'
require 'lua.raytracer.shapes.rotate'
require 'lua.raytracer.materials.lambertian'
require 'lua.raytracer.materials.metal'
require 'lua.raytracer.materials.dielectric'
require 'lua.raytracer.materials.isotropic'
require 'lua.raytracer.materials.light'

math.randomseed(os.time())

local function randfloat(min, max)
    return math.random() * (max - min) + min
end

local camera = Camera.new {
    look_from = Point.new { x = 478.0, y = 278.0, z = -600.0 },
    look_at = Point.new { x = 278.0, y = 278.0, z = 0.0 },
    vup = Direction.new { x = 0.0, y = 1.0, z = 0.0 },
    vfov = 40.0,
    focus_dist = 1.0,
    aperture = 0.0,
}

local materials = {
    ground = Lambertian.new { albedo = Color.new(0.48, 0.83, 0.53) },

    light = DiffuseLight.new { color = Color.new(7.0, 7.0, 7.0) },

    white = Lambertian.new { albedo = Color.new(0.73, 0.73, 0.73) },

    metal = Metal.new { albedo = Color.new(0.8, 0.8, 0.9), fuzz = 1.0 },
}

local shapes = {
    XzRect.new {
        x0 = 123.0,
        x1 = 423.0,
        z0 = 147.0,
        z1 = 412.0,
        k = 554.0,
        material = 'light',
    },
    Sphere.new {
        center = Point.new { x = 400.0, y = 400.0, z = 200.0 },
        radius = 50.0,
        material = Lambertian.new { albedo = Color.new(0.7, 0.3, 0.1) },
    },
    Sphere.new {
        center = Point.new { x = 260.0, y = 150.0, z = 45.0 },
        radius = 50.0,
        material = Dielectric.new { ir = 1.5 },
    },
    Sphere.new {
        center = Point.new { x = 0.0, y = 150.0, z = 145.0 },
        radius = 50.0,
        material = 'metal',
    },
    Sphere.new {
        center = Point.new { x = 220.0, y = 280.0, z = 300.0 },
        radius = 80.0,
        material = 'metal',
    },
    Sphere.new {
        center = Point.new { x = 400.0, y = 200.0, z = 400.0 },
        radius = 100.0,
        material = Lambertian.new { albedo = Color.new(0.2, 0.4, 0.9) },
    },
}

local boundary = Sphere.new {
    center = Point.new { x = 360.0, y = 150.0, z = 145.0 },
    radius = 70.0,
    material = Dielectric.new { ir = 1.5 },
}
table.insert(shapes, boundary)
table.insert(shapes, ConstantMedium.new {
    boundary = boundary,
    density = 0.2,
    phase_function = Isotropic.new { albedo = Color.new(0.2, 0.4, 0.9) },
})

boundary = Sphere.new {
    center = Point.new { x = 0.0, y = 0.0, z = 0.0 },
    radius = 5000.0,
    material = Dielectric.new { ir = 1.5 },
}
table.insert(shapes, ConstantMedium.new {
    boundary = boundary,
    density = 0.0001,
    phase_function = Isotropic.new { albedo = Color.new(1.0, 1.0, 1.0) },
})

local boxes_per_side = 20
for i = 0, boxes_per_side - 1 do
    for j = 0, boxes_per_side - 1 do
        local w = 100.0
        local x0 = -1000.0 + i * w
        local z0 = -1000.0 + j * w
        local y0 = 0.0
        local x1 = x0 + w
        local y1 = randfloat(1.0, 101.0)
        local z1 = z0 + w

        table.insert(shapes, Cuboid.new {
            minimum = Point.new { x = x0, y = y0, z = z0 },
            maximum = Point.new { x = x1, y = y1, z = z1 },
            material = 'ground',
        })
    end
end

local boxes2 = {}
local ns = 200
for j = 0, ns do
    table.insert(boxes2, Sphere.new {
        center = Point.new {
            x = randfloat(0.0, 165.0),
            y = randfloat(0.0, 165.0),
            z = randfloat(0.0, 165.0),
        },
        radius = 10.0,
        material = 'white',
    })
end

table.insert(shapes, Translate.new {
    inner = RotateY.new { inner = HittableList.new(boxes2), angle = 15.0 },
    offset = Direction.new { x = -100.0, y = 270.0, z = 395.0 },
})

local scene = Scene.new {
    camera = camera,
    materials = materials,
    shapes = shapes,
}

print(scene)
