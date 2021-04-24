Scene = {}

function Scene.new(obj)
    assert(obj.camera ~= nil)
    assert(obj.materials ~= nil)
    assert(obj.shapes ~= nil)

    setmetatable(obj, Scene)

    return obj
end

function Scene:__tostring()
    local materials = ''
    local shapes = ''

    for name, m in pairs(self.materials) do
        materials = string.format('%s\n"%s": %s,', materials, name, m)
    end

    for _, s in pairs(self.shapes) do
        shapes = string.format('%s\n%s,', shapes, s)
    end

    return string.format([[
Scene (
	camera: %s,
	materials: {%s
	},
	shapes: [%s
	],
)]], self.camera, materials, shapes)
end
