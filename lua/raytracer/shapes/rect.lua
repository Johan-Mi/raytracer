require 'lua.raytracer.materials.utils'

XyRect = {}

function XyRect.new(obj)
	assert(obj.x0 ~= nil)
	assert(obj.x1 ~= nil)
	assert(obj.y0 ~= nil)
	assert(obj.y1 ~= nil)
	assert(obj.k ~= nil)
	assert(obj.material ~= nil)

	setmetatable(obj, XyRect)

	return obj
end

function XyRect:__tostring()
	return string.format(
		'XyRect ( x0: %g, x1: %g, y0: %g, y1: %g, k: %g, material: %s)',
		self.x0,
		self.x1,
		self.y0,
		self.y1,
		self.k,
		material_tostring(self.material)
	)
end

XzRect = {}

function XzRect.new(obj)
	assert(obj.x0 ~= nil)
	assert(obj.x1 ~= nil)
	assert(obj.z0 ~= nil)
	assert(obj.z1 ~= nil)
	assert(obj.k ~= nil)
	assert(obj.material ~= nil)

	setmetatable(obj, XzRect)

	return obj
end

function XzRect:__tostring()
	return string.format(
		'XzRect ( x0: %g, x1: %g, z0: %g, z1: %g, k: %g, material: %s)',
		self.x0,
		self.x1,
		self.z0,
		self.z1,
		self.k,
		material_tostring(self.material)
	)
end

YzRect = {}

function YzRect.new(obj)
	assert(obj.y0 ~= nil)
	assert(obj.y1 ~= nil)
	assert(obj.z0 ~= nil)
	assert(obj.z1 ~= nil)
	assert(obj.k ~= nil)
	assert(obj.material ~= nil)

	setmetatable(obj, YzRect)

	return obj
end

function YzRect:__tostring()
	return string.format(
		'YzRect ( y0: %g, y1: %g, z0: %g, z1: %g, k: %g, material: %s)',
		self.y0,
		self.y1,
		self.z0,
		self.z1,
		self.k,
		material_tostring(self.material)
	)
end
