function material_tostring(material)
	if type(material) == 'string' then
		return string.format('Ref("%s")', material)
	else
		return string.format('Value(%s)', material)
	end
end
