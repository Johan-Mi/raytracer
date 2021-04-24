require 'lua.raytracer.materials.utils'

Sphere = {}

function Sphere.new(obj)
    assert(obj.center ~= nil)
    assert(obj.radius ~= nil)
    assert(obj.material ~= nil)

    setmetatable(obj, Sphere)

    return obj
end

function Sphere:__tostring()
    return string.format('Sphere ( center: %s, radius: %s, material: %s )',
                         self.center, self.radius,
                         material_tostring(self.material))
end
