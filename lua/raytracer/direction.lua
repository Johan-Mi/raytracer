Direction = {}

function Direction.new(obj)
	assert(obj.x ~= nil)
	assert(obj.y ~= nil)
	assert(obj.z ~= nil)

	setmetatable(obj, Direction)

	return obj
end

function Direction:__tostring()
	return string.format(
		'Direction ( x: %g, y: %g, z: %g )',
		self.x,
		self.y,
		self.z
	)
end
