require 'lua.raytracer.materials.utils'

ConstantMedium = {}

function ConstantMedium.new(obj)
	assert(obj.boundry ~= nil)
	assert(obj.density ~= nil)
	assert(obj.phase_function ~= nil)

	setmetatable(obj, ConstantMedium)

	return obj
end

function ConstantMedium:__tostring()
	return string.format(
		'ConstantMedium ( boundry: %s, density: %g, phase_function: %s )',
		self.boundry,
		self.density,
		material_tostring(self.phase_function)
	)
end
