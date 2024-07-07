local command = require("karasync.utils").command
local notify = require("karasync.ui.progressbar")
local M = {}
function M.setup(conf, client_id)
	notify:put("load project_unify")
	command("UploadProject", function()
		require("karasync.features.project_unify.src").upload(conf, client_id)
	end)

	command("DownloadProject", function()
		require("karasync.features.project_unify.src").clone(conf, client_id)
	end)
end

return M
