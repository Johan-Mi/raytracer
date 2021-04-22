require 'lua.raytracer.materials.utils'

Plane = {}

function Plane.new(obj)
	assert(obj.position ~= nil)
	assert(obj.normal ~= nil)
	assert(obj.material ~= nil)

	setmetatable(obj, Plane)

	return obj
end

function Plane:__tostring()
	return string.format(
		'Plane ( position: %s, normal: %g, material: %s )',
		self.position,
		self.normal,
		material_tostring(self.material)
	)
end
