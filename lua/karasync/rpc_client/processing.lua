local api = require("karasync.api")

local M = {
	TaskProcessor = {
		ConnectedOk = function(arg)
			vim.schedule(function()
				api.LoginServer({
					id = "testMain",
					path = "/home/message",
				})
			end)
		end,
	},
}

local processbar = require("karasync.ui").processbar

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
		processbar:put("close connection")
		return
	end
	M.analyze_data(data, function(arg)
		local e, value = pcall(vim.json.decode, arg)
		if e then
			if M.TaskProcessor[value.code] ~= nil then
				M.TaskProcessor[value.code](value)
			else
				processbar:put("no found code callback " .. value.code)
			end
		else
			processbar:put(arg)
		end
	end)
end

return M
