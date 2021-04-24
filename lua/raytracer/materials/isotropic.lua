Isotropic = {}

function Isotropic.new(obj)
    assert(obj.albedo ~= nil)

    setmetatable(obj, Isotropic)

    return obj
end

function Isotropic:__tostring()
    return string.format('Isotropic ( albedo: %s )', self.albedo)
end
