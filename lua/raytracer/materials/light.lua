DiffuseLight = {}

function DiffuseLight.new(obj)
	assert(obj.color ~= nil)

	setmetatable(obj, DiffuseLight)

	return obj
end

function DiffuseLight:__tostring()
	return string.format('DiffuseLight ( color: %s )', self.color)
end
