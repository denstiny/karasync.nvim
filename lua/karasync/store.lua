local M = {
	core = {},
}

function M.append(key, value)
	M.core[key] = value
end

function M.remove(key)
	if M.core[key] ~= nil then
		M.core[key] = nil
	end
end

function M.get(key)
	return M.core[key]
end

return M
