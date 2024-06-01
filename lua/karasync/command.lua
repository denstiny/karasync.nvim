local utils = require("karasync.utils")
local event = require("karasync.event")
local evens = require("karasync.info").events
local strcut = require("karasync.rpc_struct")

utils.command("TestSendTask", function()
	event:emit_signal(evens.KarasyncSendTask, strcut:cloneProject())
end)

utils.command("StopKarasync", function()
	event:emit_signal(evens.KarasyncSendTask, strcut:ExitServer())
end)

utils.command("StartKarasync", function()
	require("karasync.rpc"):listen()
end)
