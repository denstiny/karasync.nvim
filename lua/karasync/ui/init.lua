local api = vim.api
local M = {
	context = {
		buf_id = nil,
		win_id = nil,
		schedule = 0,
		msg = {},
	},
}

function M:builder_window()
	local width = vim.api.nvim_win_get_width(0)
	local height = vim.api.nvim_win_get_height(0)
	local count = vim.api.nvim_buf_line_count(self.context.buf_id)
	local win_id = vim.api.nvim_open_win(
		self.context.buf_id,
		false,
		{ relative = "win", row = height - count, col = width, width = 12, height = 1, style = "minimal" }
	)
	return win_id
end

function M:builder()
	local buf_id = vim.api.nvim_create_buf(false, true)
	api.nvim_buf_set_option(buf_id, "buftype", "nofile")
	api.nvim_buf_set_option(buf_id, "filetype", "karasync")

	self.context = {
		buf_id = buf_id,
		win_id = nil,
		schedule = 0,
		msg = {},
	}

	return self
end

function M:del_first()
	local count = vim.api.nvim_buf_line_count(self.context.buf_id)
	if count == 1 then
		self.context.msg = {}
    vim.uv.sleep(1000)
		return false
	else
		self.context.msg = vim.api.nvim_buf_get_lines(self.context.buf_id, 1, -1, false)
    vim.uv.sleep(1000)
		return true
	end
end

function M:close()
	vim.api.nvim_win_close(self.context.win_id, false)
end

function M:put(vtr, msg)
	self.context.schedule = self.context.schedule + vtr
	table.insert(self.context.msg, msg)
	M:show_msg()
  local ctx = vim.uv.new_work(self.del_first,function (var)
    if !var then
      self:close()
    end
  end)
  vim.uv.queue_work(ctx)
end

function M:show_msg()
	vim.api.nvim_buf_set_lines(self.context.buf_id, 0, -1, false, self.context.msg)
	if self.context.win_id == nil then
		self:builder_window()
	end
end

return M
