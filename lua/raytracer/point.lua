Point = {}
Point.__index = Point

function Point.new(obj)
	assert(obj.x ~= nil)
	assert(obj.y ~= nil)
	assert(obj.z ~= nil)

	setmetatable(obj, Point)

	return obj
end

function Point:__tostring()
	return string.format(
		'Point ( x: %g, y: %g, z: %g )',
		self.x,
		self.y,
		self.z
	)
end

function Point:__add(rhs)
	return Point.new {
		x = self.x + rhs.x,
		y = self.y + rhs.y,
		z = self.z + rhs.z,
	}
end

function Point:__sub(rhs)
	return Point.new {
		x = self.x - rhs.x,
		y = self.y - rhs.y,
		z = self.z - rhs.z,
	}
end

function Point:len_squared()
	return self.x * self.x + self.y * self.y + self.z * self.z
end

function Point:len()
	return math.sqrt(self:len_squared())
end
