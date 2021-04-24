Translate = {}

function Translate.new(obj)
    assert(obj.inner ~= nil)
    assert(obj.offset ~= nil)

    setmetatable(obj, Translate)

    return obj
end

function Translate:__tostring()
    return string.format('Translate ( inner: %s, offset: %s )', self.inner,
                         self.offset)
end
