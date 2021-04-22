HittableList = {}

function HittableList.new(list)
	obj = { shapes = list }

	setmetatable(obj, HittableList)

	return obj
end

function HittableList:__tostring()
	local shapes = ''
	for _, shape in pairs(self.shapes) do
		shapes = string.format('%s\n%s,', shapes, shape)
	end

	return string.format('HittableList ( shapes: [%s] )', shapes)
end
