local utils = require("karasync.utils")
local event = require("karasync.event")
local evens = require("karasync.info").events

utils.command("StartKarasync", function()
	require("karasync.rpc"):listen()
end)

local M = {}

local message = {
	id = "",
	code = "",
	msg = "",
}

---@param task_conf message
function M.SendTask(task_conf)
	evens:emit_signal(evens.KarasyncSendTask, task_conf)
end

return M
