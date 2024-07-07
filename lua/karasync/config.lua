local M = {
	config = {},
	default = {
		ip = "127.0.0.1",
		port = "5555",
		data_dir = vim.fn.stdpath("data") .. "/karasync",
		autostart = false,
		modules = {
			project_unify = {},
		},
		author = {
			name = "denstiny",
			email = "2254228017@qq.com",
		},
	},
}

---@param opts
---@return
function M:merge_options(opts)
	if type(opts) == "table" and opts ~= {} then
		self.config = vim.tbl_deep_extend("force", self.default, opts)
	else
		self.config = self.default
	end
	return self.config
end

return M
