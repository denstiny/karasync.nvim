local api = vim.api
local M = {
	context = {},
}

---@return {apply: function,only_config: function}
function M:builder_window()
	local width = vim.api.nvim_win_get_width(0)
	local height = vim.api.nvim_win_get_height(0)
	local count = vim.api.nvim_buf_line_count(self.context.buf_id)
	local config = {
		relative = "win",
		row = height - count - 2,
		col = width,
		width = self.context._maxline,
		height = count,
		style = "minimal",
		border = "solid",
		footer = "karasync",
		footer_pos = "right",
	}
	local opt = {
		apply = function()
			self.context.win_id = vim.api.nvim_open_win(self.context.buf_id, false, config)
			vim.api.nvim_win_set_option(self.context.win_id, "winblend", 100)
		end,
		only_config = function()
			return config
		end,
	}
	return opt
end

---@return self
function M:builder()
	local buf_id = vim.api.nvim_create_buf(false, true)
	api.nvim_buf_set_option(buf_id, "buftype", "nofile")
	api.nvim_buf_set_option(buf_id, "filetype", "karasync")

	self.context = {
		buf_id = buf_id,
		win_id = nil,
		schedule = 0,
		msg = {},
		_maxline = 0,
		timer = vim.uv.new_timer(),
	}

	return vim.tbl_extend("force", {}, self)
end

--- 删除消息，当消息全部删除时关闭窗口
function M:del_msg(id)
	self.context.msg[id] = nil
	local count = 0
	for _, _ in pairs(self.context.msg) do
		count = count + 1
	end
	if count == 0 then
		M:close()
	else
		self:show_msg()
	end
end

---  关闭窗口
function M:close()
	pcall(vim.api.nvim_win_close, self.context.win_id, false)
	self.context.win_id = nil
end

function M:generte_id()
	local currentTime = os.time()
	local randomNum = math.random(1000, 9999)
	return currentTime .. tostring(randomNum)
end

--- 压入新的消息
---@param msg string
function M:put(msg)
	if msg:len() > self.context._maxline then
		self.context._maxline = msg:len()
	end
	local id = M:generte_id()
	self.context.msg[id] = msg
	M:show_msg()
	self:sleep_remove(5000, id)
	return id
end

--- 推迟清理消息
---@param time integer
---@param id string
function M:sleep_remove(time, id)
	vim.defer_fn(function()
		self:del_msg(id)
	end, time)
end

--- 更新窗口配置,每次添加新消息之后需要对窗口大小和位置做相应的调整
function M:update_window()
	vim.api.nvim_win_set_config(self.context.win_id, self:builder_window().only_config())
end

function M:align_right_msg()
	local msgs = {}
	for _, msg in pairs(self.context.msg) do
		table.insert(msgs, string.rep(" ", self.context._maxline - msg:len()) .. msg)
	end
	return msgs
end

--- 显示消息
function M:show_msg()
	vim.schedule(function()
		vim.api.nvim_buf_set_lines(self.context.buf_id, 0, -1, true, self:align_right_msg())
		if self.context.win_id == nil then
			self:builder_window().apply()
		else
			self:update_window()
		end
	end)
end

---@param id integer
---@param msg string
function M:update_msg(id, msg)
	if self.context.msg[id] then
		self.context.msg[id] = msg
	end
end

return M
