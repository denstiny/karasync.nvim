local M = {}
local rpc = require("karasync.rpc")
local tasks = require("karasync.info").tasks
local store = require("karasync.store")

local TaskMap = {
	-- 连接服务器成功
	ConnectedOk = function(arg)
		print(arg.msg.body)
	end,
	--- 同步项目
	AsyncProjected = function(arg)
		print("同步项目")
	end,
	-- 克隆项目到本地
	CloneProjected = function(arg)
		vim.notify(arg.msg.body)
	end,

	[tasks.AsyncProjected] = function(arg)
		vim.notify(vim.inspect(arg))
	end,
}

function M.procession(data)
	if data == nil then
		vim.notify("断开连接")
	end
	local e, data = pcall(vim.json.decode, data)
	if e then
		if TaskMap[data.msg.code] ~= nil then
			TaskMap[data.msg.code](data)
		else
			vim.notify("nod found code callback " .. vim.inspect(data), vim.log.levels.ERROR)
		end
	else
		vim.notify(data)
	end
end

return M
