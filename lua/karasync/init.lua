local M = {}
local events = require("karasync.info").events
local rpc = require("karasync.rpc_client.rpc")
local processing = require("karasync.rpc_client.processing")
local event = require("karasync.event")

function M.setup(conf)
	require("karasync.store").append("client_id", os.time() .. "")
	M.config = require("karasync.config"):merge_options(conf)
	M.init_bind_signal()
	vim.defer_fn(function()
		M.load_module()
	end, 0)

	if M.config.autostart then
		rpc:listen()
	end
end

--- 初始化信号的绑定
function M.init_bind_signal()
	event:bind_signal(events.karasyncStart, function(arg)
		rpc:readlisten(function(err, data)
			if not err then
				processing.procession(data)
			end
		end)
	end)

	event:bind_signal(events.karasyncStartError, function(arg) end)
	event:bind_signal(events.KarasyncSendTask, function(arg)
		local data = unpack(arg.data)
		rpc:send(data)
	end)
end

function M.load_module()
	for key, value in pairs(M.config.modules) do
		require("karasync.features." .. key).setup(value, require("karasync.store").get("client_id"))
	end
end

return M
