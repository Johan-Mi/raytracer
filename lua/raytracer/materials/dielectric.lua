Dielectric = {}

function Dielectric.new(obj)
	assert(obj.ir ~= nil)

	setmetatable(obj, Dielectric)

	return obj
end

function Dielectric:__tostring()
	return string.format('Dielectric ( ir: %g )', self.ir)
end
