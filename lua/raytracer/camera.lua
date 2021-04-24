Camera = {}

function Camera.new(obj)
    assert(obj.look_from ~= nil)
    assert(obj.look_at ~= nil)
    assert(obj.vup ~= nil)
    assert(obj.vfov ~= nil)
    assert(obj.aperture ~= nil)
    assert(obj.focus_dist ~= nil)

    setmetatable(obj, Camera)

    return obj
end

function Camera:__tostring()
    return string.format([[
Camera (
	look_from: %s,
	look_at: %s,
	vup: %s,
	vfov: %s,
	aperture: %s,
	focus_dist: %s,
)]], self.look_from, self.look_at, self.vup, self.vfov, self.aperture,
                         self.focus_dist)
end
