local struct = {}
local tasks = require("karasync.info").tasks
local utils = require("karasync.utils")

function struct:cloneProject()
	local s = {
		msg = {
			host = "127.0.0.1:22",
			path = "/root/Public",
			root = "Public",
			user = "root",
			password = "asd",
		},
		code = tasks.CloneProjected,
		id = os.time() .. "",
	}
	utils.input(s)
	--vim.notify("input: " .. vim.inspect(s))
	return s
end

return struct
