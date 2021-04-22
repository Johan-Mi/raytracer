require 'lua.raytracer.materials.utils'

Cuboid = {}

function Cuboid.new(obj)
	assert(obj.minimum ~= nil)
	assert(obj.maximum ~= nil)
	assert(obj.material ~= nil)

	setmetatable(obj, Cuboid)

	return obj
end

function Cuboid:__tostring()
	return string.format(
		'Cuboid ( minimum: %s, maximum: %s, material: %s )',
		self.minimum,
		self.maximum,
		material_tostring(self.material)
	)
end
