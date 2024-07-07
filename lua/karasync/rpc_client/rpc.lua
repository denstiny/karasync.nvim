local utils = require("karasync.utils")
local event = require("karasync.event")
local info = require("karasync.info")
local level = vim.log.levels
local progressbar = require("karasync.ui.progressbar")
local events = require("karasync.info").events
local rpc = {
	socket = nil,
	jobs = {},
}

function rpc:check_server_start(timer)
	self.socket = vim.uv.new_tcp()
	local ip = require("karasync.config").config.ip
	local port = require("karasync.config").config.port

	self.socket:connect(ip, port, function(err)
		if err then
			vim.schedule(function()
				self:start_rpc_server()
			end)
			return
		else
			timer:stop()
			if not timer:is_closing() then
				timer:close()
			end
			vim.schedule(function()
				event:emit_signal(events.karasyncStart)
			end)
		end
	end)
end

---@param mes table
function rpc:send(mes)
	local msg = vim.json.encode(mes)
	self.socket:write(msg, function(err)
		if err then
			progressbar:put("failed: send message " .. err)
		end
	end)
end
--- 关闭服务器
function rpc:close()
	self.socket:close()
end
--- 读取服务器发送的消息
---@return
function rpc:readlisten(callback)
	self.socket:read_start(callback)
end

function rpc:is_conn()
	return self.socket ~= nil
end

function rpc:listen()
	local timer = vim.uv.new_timer()
	local i = 0
	timer:start(
		100,
		1000,
		vim.schedule_wrap(function()
			i = i + 1
			local status = self:check_server_start(timer)
			if status then
			elseif i > 3 then
				timer:stop()
				if not timer:is_closing() then
					timer:close()
				end
				event:emit_signal(events.karasyncStartError, { msg = "connection out time" })
			else
				--vim.notify("failed: connection error", level.ERROR, { title = "karasync" })
			end
		end)
	)
end

function rpc:start_rpc_server()
	--utils.notify("Try to start the server", vim.log.levels.INFO)
	progressbar:put("Try to start the server")
	local dir = utils.get_plugin_root(info.plugin)
	dir = dir .. "core/karasync"
	local config = require("karasync.config").config
	local cmd = string.format("cd %s && cargo run %s %s %s", dir, config.data_dir, config.ip, config.port)
	progressbar:put(cmd)

	vim.fn.jobstart(cmd, {
		on_stdout = function(job_id, data, event)
			for _, item in pairs(data) do
				--utils.notify(item)
			end
		end,
		on_stderr = function(job_id, data, event)
			for _, item in pairs(data) do
				--utils.notify(item, event)
			end
		end,
		on_exit = function(job_id, data, event) end,
	})
end

return rpc
