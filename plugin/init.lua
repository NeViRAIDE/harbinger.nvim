vim.defer_fn(function()
  local bufs = vim.api.nvim_list_bufs()

  if #vim.fn.argv() == 0 and (#bufs == 1 and bufs[1] == 1) then
    require('harbinger').open()
    vim.api.nvim_exec2(':bd#', { output = true })
  end
end, 0)

require('harbinger.whichkey')
