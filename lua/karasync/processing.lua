local M = {}
local rpc = require("karasync.rpc")

local TaskMap = {
	ConnectedOk = function(data)
		print("ConnectedOk")
	end,
}

function M.procession(data)
	data = vim.json.decode(data)
	if TaskMap[data.code] ~= nil then
		TaskMap[data.code](data)
	end
end

return M
