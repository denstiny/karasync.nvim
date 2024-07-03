local M = {}

--- 格式化消息
---@param schedule integer
---@param msg string
---@return string
function M.format(schedule, msg)
	return string.format("(%d) %s", schedule, msg)
end

return M
