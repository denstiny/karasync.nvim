local command = require("karasync.utils").command
local notify = require("karasync.ui.progressbar")
local M = {}
function M.setup(conf, client_id)
	notify:put("load project_unify")
	command("UploadProject", function()
		local c = {
			path = vim.fn.getcwd() .. "./.project",
		}
		require("karasync.features.project_unify.src").upload(c, client_id)
	end)

	command("DownloadProject", function()
		local c = {}
		require("karasync.features.project_unify.src").clone(c, client_id)
	end)
end

return M
