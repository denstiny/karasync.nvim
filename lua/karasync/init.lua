local M = {}
local events = require("karasync.info").events
local rpc = require("karasync.rpc")
local processing = require("karasync.processing")
local event = require("karasync.event")

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
		local data = unpack(arg.data)
		rpc:send(data)
	end)

	rpc:listen()
	require("karasync.command")
end

return M
