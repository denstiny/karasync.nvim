local api = require("karasync.api")
local pros = require("karasync.rpc_client.processing")

local M = {}
function M.upload(conf, client_id)
	local code = "UploadProject"
	api.SendTask({
		code = code,
		id = client_id,
		msg = {},
	})

	pros.resign(code, function(arg) end)
end

function M.clone(conf, client_id) end

return M
