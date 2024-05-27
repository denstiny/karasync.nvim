local M = {}
local rpc = require("karasync.rpc")

local TaskMap = {
	ConnectedOk = function(data)
		print("ConnectedOk")
	end,
}

function M.procession(data)
	local e, data = pcall(vim.json.decode, data)
	if e then
		if TaskMap[data.code] ~= nil then
			TaskMap[data.code](data)
		end
	else
		vim.notify(data)
	end
end

return M
