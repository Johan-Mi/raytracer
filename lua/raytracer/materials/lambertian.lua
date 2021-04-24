Lambertian = {}

function Lambertian.new(obj)
    assert(obj.albedo ~= nil)

    setmetatable(obj, Lambertian)

    return obj
end

function Lambertian:__tostring()
    return string.format('Lambertian ( albedo: %s )', self.albedo)
end
