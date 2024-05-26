local events = require("karasync.info").events
local rpc = require("karasync.rpc")
local M = {}
local session = nil
local utils = require("karasync.utils")
local rpc = require("karasync.rpc")
local processing = require("karasync.processing")

function M.setup()
	local event = require("karasync.event")
	event:bind_signal(events.karasyncStart, function(arg)
		rpc:readlisten(function(err, data)
			if not err then
				processing.procession(data)
			end
		end)
	end)

	event:bind_signal(events.karasyncStartError, function(arg) end)

	rpc:listen()
	-- rpc:start_rpc_server()
end

return M
