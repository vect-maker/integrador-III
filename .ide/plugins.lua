vim.g.clipboard = {
	name = "OSC 52",
	copy = {
		["+"] = require("vim.ui.clipboard.osc52").copy("+"),
		["*"] = require("vim.ui.clipboard.osc52").copy("*"),
	},
	paste = {
		["+"] = require("vim.ui.clipboard.osc52").paste("+"),
		["*"] = require("vim.ui.clipboard.osc52").paste("*"),
	},
}

vim.filetype.add({
	extension = { Containerfile = "dockerfile" },
	filename = { ["Containerfile"] = "dockerfile" },
	pattern = {
		[".*%.Containerfile"] = "dockerfile",
		[".*%.containerfile"] = "dockerfile",
	},
})

return {
	{
		"nvim-tree/nvim-tree.lua",
		opts = {
			filesystem_watchers = {
				enable = true,
				ignore_dirs = { "data", "target" },
			},
		},
	},
	{
		"NoahTheDuke/vim-just",
		ft = { "just", "justfile" },
	},
	{
		"stevearc/conform.nvim",
		event = "BufWritePre",
		opts = {
			formatters_by_ft = {
				sh = { "shfmt" },
				rust = { "rustfmt" },
			},
		},
	},
	{
		"neovim/nvim-lspconfig",
		config = function()
			local lspconfig = require("lspconfig")
			local servers = { "bashls", "yamlls", "dockerls" }
			for _, lsp in ipairs(servers) do
				lspconfig[lsp].setup({
					on_attach = require("nvchad.configs.lspconfig").on_attach,
					capabilities = require("nvchad.configs.lspconfig").capabilities,
				})
			end
		end,
	},
	{
		"mrcjkb/rustaceanvim",
		version = "^8",
		ft = { "rust" },
		init = function()
			vim.g.rustaceanvim = function()
				local ok, blink = pcall(require, "blink.cmp")
				local capabilities = ok and blink.get_lsp_capabilities() or vim.lsp.protocol.make_client_capabilities()

				return {
					server = {
						capabilities = capabilities,
						on_attach = function(client, bufnr)
							client.server_capabilities.semanticTokensProvider =
								client.server_capabilities.semanticTokensProvider

							if client.server_capabilities.inlayHintProvider then
								vim.lsp.inlay_hint.enable(true, { bufnr = bufnr })
							end

							local map = function(mode, keys, func, desc)
								vim.keymap.set(mode, keys, func, { buffer = bufnr, desc = "Rust: " .. desc })
							end

							map("n", "<leader>ca", function()
								vim.cmd.RustLsp("codeAction")
							end, "Code Action")
							map("n", "<leader>dr", function()
								vim.cmd.RustLsp("debuggables")
							end, "Debuggables")
							map("n", "<leader>rr", function()
								vim.cmd.RustLsp("runnables")
							end, "Runnables")
							map("n", "<leader>em", function()
								vim.cmd.RustLsp("expandMacro")
							end, "Expand Macro")
							map("n", "K", function()
								vim.cmd.RustLsp({ "hover", "actions" })
							end, "Hover Actions")
						end,
						default_settings = {
							["rust-analyzer"] = {
								check = { command = "clippy" },
							},
						},
					},
					dap = {
						adapter = require("rustaceanvim.config").get_lldb_adapter("/usr/bin/lldb-dap"),
					},
				}
			end
		end,
	},
	{
		"folke/trouble.nvim",
		opts = {},
		cmd = "Trouble",
		keys = {
			{ "<leader>xx", "<cmd>Trouble diagnostics toggle<cr>", desc = "Diagnostics (Trouble)" },
			{ "<leader>xX", "<cmd>Trouble diagnostics toggle filter.buf=0<cr>", desc = "Buffer Diagnostics (Trouble)" },
			{ "<leader>cs", "<cmd>Trouble symbols toggle focus=false<cr>", desc = "Symbols (Trouble)" },
			{
				"<leader>cl",
				"<cmd>Trouble lsp toggle focus=false win.position=right<cr>",
				desc = "LSP Definitions / references / ... (Trouble)",
			},
			{ "<leader>xL", "<cmd>Trouble loclist toggle<cr>", desc = "Location List (Trouble)" },
			{ "<leader>xQ", "<cmd>Trouble qflist toggle<cr>", desc = "Quickfix List (Trouble)" },
		},
	},
	{
		"mfussenegger/nvim-dap",
		config = function()
			local dap, dapui = require("dap"), require("dapui")
			dap.listeners.before.attach.dapui_config = function()
				dapui.open()
			end
			dap.listeners.before.launch.dapui_config = function()
				dapui.open()
			end
			dap.listeners.before.event_terminated.dapui_config = function()
				dapui.close()
			end
			dap.listeners.before.event_exited.dapui_config = function()
				dapui.close()
			end
		end,
	},
	{
		"rcarriga/nvim-dap-ui",
		dependencies = { "mfussenegger/nvim-dap", "nvim-neotest/nvim-nio" },
		config = function()
			require("dapui").setup()
		end,
	},
	{
		"saecki/crates.nvim",
		event = { "BufRead Cargo.toml" },
		config = function()
			require("crates").setup()
		end,
	},
	{ import = "nvchad.blink.lazyspec" },
	{
		"nvim-treesitter/nvim-treesitter",
		opts = {
			ensure_installed = {
				"vim",
				"vimdoc",
				"rust",
				"ron",
				"toml",
				"just",
				"bash",
				"dockerfile",
				"yaml",
			},
		},
	},
	{
		"stevearc/aerial.nvim",
		opts = {},
		dependencies = {
			"nvim-treesitter/nvim-treesitter",
			"nvim-tree/nvim-web-devicons",
		},
		keys = {
			{ "<leader>a", "<cmd>AerialToggle!<CR>", desc = "Toggle Aerial (Code Outline)" },
		},
	},
}
