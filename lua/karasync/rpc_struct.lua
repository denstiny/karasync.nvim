local struct = {}
local tasks = require("karasync.info").tasks
local utils = require("karasync.utils")

function struct:cloneProject()
	local s = {
		msg = {
			host = "127.0.0.1:22",
			path = "/root/Public/colorful-winsep.nvim",
			save_dir = vim.fn.getcwd(),
			user = "root",
			login = {
				SSHPASSWORD = "asd",
			},
		},
		code = tasks.CloneProjected,
		id = os.time() .. "",
	}
	utils.input(s)
	--vim.notify("input: " .. vim.inspect(s))
	return s
end

function struct:pushProject()
	local s = {
		msg = {
			save_dir = vim.fn.getcwd(),
		},
		code = tasks.PushProjected,
		id = os.time() .. "",
	}
	return s
end

function struct:ExitServer()
	local s = {
		code = tasks.ExitKarasync,
		id = os.time() .. "",
	}
	return s
end

return struct
