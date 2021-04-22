RotateY = {}

function RotateY.new(obj)
	assert(obj.inner ~= nil)
	assert(obj.angle ~= nil)

	setmetatable(obj, RotateY)

	return obj
end

function RotateY:__tostring()
	return string.format(
		'RotateY ( inner: %s, angle: %g )',
		self.inner,
		self.angle
	)
end
