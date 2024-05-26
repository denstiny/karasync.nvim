local progress = {}
local fidget_progress = require("fidget.progress")

function progress:new(title, message, name)
	local _progress = {}

	_progress.title = title
	_progress.prog = fidget_progress.handle.create({
		title = title,
		message = message,
		lsp_client = { name = name },
		percentage = 0,
	})

	function _progress:update(message, percentage)
		self.prog:report({
			title = self.title,
			message = message,
			progress = progress,
		})
	end

	function _progress:finish()
		self.prog:finish()
	end
	return _progress
end

return progress
