local api = require("karasync.api")
local pros = require("karasync.rpc_client.processing")

local M = {}
function M.upload(conf, client_id)
	local code = "UploadProject"
	api.SendTask({
		code = code,
		msgid = client_id,
		msg = conf,
	})

	pros.resign(code, function(arg) end)
end

function M.clone(conf, client_id) end

return M
