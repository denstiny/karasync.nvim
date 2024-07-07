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

return M
