local info = {
	plugin = "karasync.nvim",
	events = {
		karasyncStart = "karasyncStart",
		karasyncStartError = "karasyncStartError",
		KarasyncSendTask = "KarasyncSendTask",
	},
	tasks = {
		AsyncProjected = "AsyncProjected",
		CloneProjected = "CloneProjected",
		ExitKarasync = "ExitServer",
	},
}

return info
