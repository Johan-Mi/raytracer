Metal = {}

function Metal.new(obj)
    assert(obj.albedo ~= nil)
    assert(obj.fuzz ~= nil)

    setmetatable(obj, Metal)

    return obj
end

function Metal:__tostring()
    return string.format('Metal ( albedo: %s, fuzz: %g )', self.albedo,
                         self.fuzz)
end
