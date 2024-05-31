local events = require("karasync.info").events
local tasks = require("karasync.info").tasks
local rpc = require("karasync.rpc")
local M = {}
local session = nil
local utils = require("karasync.utils")
local rpc = require("karasync.rpc")
local processing = require("karasync.processing")
local event = require("karasync.event")
local rpc_struct = require("karasync.rpc_struct")

function M.SendTask()
	event:emit_signal(events.KarasyncSendTask, rpc_struct:cloneProject())
end

function M.setup()
	event:bind_signal(events.karasyncStart, function(arg)
		rpc:readlisten(function(err, data)
			if not err then
				processing.procession(data)
			end
		end)
	end)

	event:bind_signal(events.karasyncStartError, function(arg) end)

	event:bind_signal(events.KarasyncSendTask, function(arg)
		--processing.procession(arg.data)
		local data = unpack(arg.data)
		rpc:send(data)
	end)

	rpc:listen()
	-- rpc:start_rpc_server()
end

vim.api.nvim_create_user_command("TestSendTask", function()
	require("karasync").SendTask()
end, {})

return M
