local M = {}

M.config = {}

M.setup = function(config)
  M.config = vim.tbl_deep_extend('force', M.config, config or {})

  dofile(vim.g.ntc .. 'harbinger')

  require('harbinger.autocommands')
  require('harbinger.commands').setup()

  vim.defer_fn(function()
    local bufs = vim.api.nvim_list_bufs()

    if #vim.fn.argv() == 0 and (#bufs == 1 and bufs[1] == 1) then
      require('harbinger.open').open()
      vim.api.nvim_exec2(':bd#', { output = true })
    end
  end, 0)

  require('harbinger.whichkey')
end

function M.cmd(name) require('harbinger.commands').cmd(name) end

return M
