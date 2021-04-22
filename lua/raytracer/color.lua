Color = {}

function Color.new(r, g, b)
	obj = { r = r, g = g, b = b }

	setmetatable(obj, Color)

	return obj
end

function Color:__tostring()
	return string.format(
		'( %g, %g, %g )',
		self.r,
		self.g,
		self.b
	)
end
