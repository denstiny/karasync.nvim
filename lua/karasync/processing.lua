local M = {}
local rpc = require("karasync.rpc")
local tasks = require("karasync.info").tasks
local store = require("karasync.store")
local utils = require("karasync.utils")

local TaskMap = {
	-- 连接服务器成功
	ConnectedOk = function(arg)
		utils.notify(arg.msg.body)
	end,
	--- 同步项目
	AsyncProjected = function(arg)
		print("同步项目")
	end,
	-- 克隆项目到本地
	CloneProjected = function(arg)
		local body = arg.msg.body
		local code = arg.msg.code
		local process = arg.msg.process
		utils.notify(string.format("[%s](%s): %s", code, process, body))
	end,

	[tasks.AsyncProjected] = function(arg)
		vim.notify(vim.inspect(arg))
	end,
}

M.buf = ""
--- 解析tcp数据获取等待获取到完整的数据包
function M.analyze_data(data, callback)
	M.buf = M.buf .. data
	while true do
		local msg_end = M.buf:find("\n", 1, true)
		if not msg_end then
			break
		end
		local message = M.buf:sub(1, msg_end - 1)
		M.buf = M.buf:sub(msg_end + 1)
		callback(message)
	end
end

function M.procession(data)
	if data == nil then
		vim.notify("close connection")
		return
	end
	M.analyze_data(data, function(arg)
		local e, value = pcall(vim.json.decode, arg)
		if e then
			if TaskMap[value.msg.code] ~= nil then
				TaskMap[value.msg.code](value)
			else
				vim.notify("nod found code callback " .. vim.inspect(value), vim.log.levels.ERROR)
			end
		else
			vim.notify("解析错误: " .. arg)
		end
	end)
end

return M
