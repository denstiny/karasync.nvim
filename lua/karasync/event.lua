local event = {}
local group = vim.api.nvim_create_augroup("Karasync-nvim", { clear = true })

---
---@param signal string
function event:emit_signal(signal, ...)
	vim.api.nvim_exec_autocmds("User", {
		group = group,
		pattern = signal,
		data = { ... },
	})
end

---@param signal string
---@param callback function
function event:bind_signal(signal, callback)
	vim.api.nvim_create_autocmd({ "User" }, {
		pattern = signal,
		group = group,
		callback = callback,
	})
end

return event
