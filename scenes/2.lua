#!/usr/bin/lua

require 'lua.raytracer.scene'
require 'lua.raytracer.camera'
require 'lua.raytracer.point'
require 'lua.raytracer.direction'
require 'lua.raytracer.color'
require 'lua.raytracer.materials.lambertian'
require 'lua.raytracer.materials.metal'
require 'lua.raytracer.materials.dielectric'
require 'lua.raytracer.materials.light'
require 'lua.raytracer.shapes.sphere'

math.randomseed(os.time())

local function randfloat(min, max)
	return math.random() * (max - min) + min
end

local camera = Camera.new {
	look_from = Point.new {
		x = 13.0,
		y = 2.0,
		z = 3.0,
	},
	look_at = Point.new {
		x = 0.0,
		y = 0.0,
		z = 0.0,
	},
	vup = Direction.new {
		x = 0.0,
		y = 1.0,
		z = 0.0,
	},
	vfov = 20.0,
	focus_dist = 10.0,
	aperture = 0.1,
}

local materials = {}
local shapes = {}

local ground_material = Lambertian.new {
	albedo = Color.new(0.5, 0.5, 0.5),
}
materials['ground'] = ground_material
table.insert(shapes, Sphere.new {
	center = Point.new {
		x = 0.0,
		y = -1000.0,
		z = 0.0,
	},
	radius = 1000.0,
	material = 'ground',
})

local material1 = Dielectric.new {
	ir = 1.5,
}
materials['material1'] = material1
table.insert(shapes, Sphere.new {
	center = Point.new {
		x = 0.0,
		y = 1.0,
		z = 0.0,
	},
	radius = 1.0,
	material = 'material1',
})

local material2 = Lambertian.new {
	albedo = Color.new(0.4, 0.2, 0.1),
}
materials['material2'] = material2
table.insert(shapes, Sphere.new {
	center = Point.new {
		x = -4.0,
		y = 1.0,
		z = 0.0,
	},
	radius = 1.0,
	material = 'material2',
})

local material3 = Metal.new {
	albedo = Color.new(0.7, 0.6, 0.5),
	fuzz = 0.0,
}
materials['material3'] = material3
table.insert(shapes, Sphere.new {
	center = Point.new {
		x = 4.0,
		y = 1.0,
		z = 0.0,
	},
	radius = 1.0,
	material = 'material3',
})

for a = -11, 10 do
	for b = -11, 10 do
		local choose_mat = randfloat(0.0, 1.0)
		local center = Point.new {
			x = a + randfloat(0.0, 0.9),
			y = 0.2,
			z = b + randfloat(0.0, 0.9),
		}

		if (center - Point.new { x = 4.0, y = 0.2, z = 0.0 }):len() > 0.9 then
			local sphere_material

			if choose_mat < 0.8 then
				local albedo = Color.new(
					randfloat(0.0, 1.0) * randfloat(0.0, 2.0),
					randfloat(0.0, 1.0) * randfloat(0.0, 2.0),
					randfloat(0.0, 1.0) * randfloat(0.0, 2.0)
				)

				sphere_material = DiffuseLight.new {
					color = albedo,
				}
			elseif choose_mat < 0.95 then
				local albedo = Color.new(
					randfloat(0.5, 1.0),
					randfloat(0.5, 1.0),
					randfloat(0.5, 1.0)
				)
				local fuzz = randfloat(0.0, 0.5)

				sphere_material = Metal.new {
					albedo = albedo,
					fuzz = fuzz,
				}
			else
				sphere_material = Dielectric.new {
					ir = 1.5,
				}
			end

			table.insert(shapes, Sphere.new {
				center = center,
				radius = 0.2,
				material = sphere_material,
			})
		end
	end
end

local scene = Scene.new {
	camera = camera,
	materials = materials,
	shapes = shapes,
}

print(scene)
