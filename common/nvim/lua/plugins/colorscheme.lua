return {
  {
    'loctvl842/monokai-pro.nvim', 
    priority = 1000,
    lazy = false,
    init = function()
      vim.cmd.colorscheme 'monokai-pro-classic'

      -- Override line number background (after colorscheme loads)
      vim.api.nvim_set_hl(0, 'LineNr', { fg = '#919288', bg = '#161613' }) -- Default line numbers
      vim.api.nvim_set_hl(0, 'CursorLineNr', { fg = '#fdfff1', bg = '#161613' }) -- Current line number
      vim.api.nvim_set_hl(0, 'SignColumn', { bg = '#161613' })
      vim.api.nvim_set_hl(0, 'MsgArea', { bg = '#1d1e19' })
      
      -- You can configure highlights by doing something like:
      vim.cmd.hi 'Comment gui=none'
      vim.opt.signcolumn = 'auto'
    end
  },
  {
    "ellisonleao/gruvbox.nvim",
    priority = 1000,
    init = function()
      --vim.cmd.colorscheme 'gruvbox'
    end
  },
}
