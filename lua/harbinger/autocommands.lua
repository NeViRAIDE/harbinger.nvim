local api = vim.api
local autocmd = require('neviraide.utils').autocmd

autocmd('NEVIRAIDE_dashresized', 'VimResized', {
  callback = function()
    if vim.bo.filetype == 'neviraideDashboard' then
      vim.opt_local.modifiable = true
      api.nvim_buf_set_lines(0, 0, -1, false, { '' })
      require('harbinger.open').open()
    end
  end,
})

autocmd('NeviraideResetCursorLinehl', 'FileType', {
  pattern = '*',
  callback = function()
    vim.api.nvim_set_option_value(
      'winhighlight',
      'CursorLine:CursorLine',
      { win = vim.api.nvim_get_current_win() }
    )
  end,
})
autocmd('NeviraideDashActiveButton', 'FileType', {
  pattern = 'neviraideDashboard',
  callback = function()
    vim.api.nvim_set_option_value(
      'winhighlight',
      'CursorLine:DashboardCursorLine',
      { win = vim.api.nvim_get_current_win() }
    )
  end,
})
