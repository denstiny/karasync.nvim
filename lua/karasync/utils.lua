local utils = {}

--- @return string
function utils.get_root_dir()
	vim.fs.root(0, ".git")
end

--- 返回当前脚本的位置
---@return string
function utils.get_script_path()
	local str = debug.getinfo(2, "S").source:sub(2)
	return str:match("(.*/)")
end

function utils.get_plugin_root(plugin_name)
	local path = utils.get_script_path()
	local nvim_end, _ = string.find(path, plugin_name)
	if not nvim_end then
		utils.notify("no find plugin root", vim.log.levels.ERROR)
		return nil
	end
	local root_path = string.sub(path, 1, nvim_end + string.len(plugin_name))
	return root_path
end

function utils.notify(msg, level)
	--require("fidget").notify(msg, level, { title = "karasync", skip_history = true })
	--	vim.schedule(function()
	--		--require("fidget").notify(msg, level, { title = "karasync", skip_history = true })
	--		vim.notify(msg, level, { title = "karasync", skip_history = true })
	--	end)
end

--- input
---@param tb table
function utils.input(tb)
	for key, v in pairs(tb) do
		if type(v) == "table" then
			utils.input(tb[key])
		else
			if v == "" then
				vim.ui.input({ prompt = string.format("Enter %s: ", key) }, function(input)
					tb[key] = input
				end)
			end
		end
	end
end

--- 创建用户指令
---@param cmd string
---@param fun function
function utils.command(cmd, fun)
	vim.api.nvim_create_user_command(cmd, fun, {})
end

return utils
