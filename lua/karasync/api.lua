local utils = require("karasync.utils")
local event = require("karasync.event")
local evens = require("karasync.info").events

utils.command("StartKarasync", function()
	require("karasync.rpc_client.rpc"):listen()
end)

utils.command("StopKarsync", function()
	require("karasync.rpc_client.rpc"):close()
end)

local M = {}

--- @class task_conf
--- @field msgid string Task id
--- @field code string Task code
--- @field msg table Task msg body
--- @param task_conf task_conf
function M.SendTask(task_conf)
	event:emit_signal(evens.KarasyncSendTask, task_conf)
end

--- @class conf
--- @field id string
--- @field path string
---@param conf conf
function M.LoginServer(conf)
	event:emit_signal(evens.KarasyncSendTask, conf)
end

function M.Registration_processing() end

return M
