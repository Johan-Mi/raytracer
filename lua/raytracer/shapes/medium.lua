require 'lua.raytracer.materials.utils'

ConstantMedium = {}

function ConstantMedium.new(obj)
	assert(obj.boundary ~= nil)
	assert(obj.density ~= nil)
	assert(obj.phase_function ~= nil)

	setmetatable(obj, ConstantMedium)

	return obj
end

function ConstantMedium:__tostring()
	return string.format(
		'ConstantMedium ( boundary: %s, density: %g, phase_function: %s )',
		self.boundary,
		self.density,
		material_tostring(self.phase_function)
	)
end
