local commands = {}

local M = {}

---@type table<string, fun()>
M.commands = {}

function M.cmd(cmd)
  if M.commands[cmd] then
    M.commands[cmd]()
  else
    M.commands.history()
  end
end

function M.setup()
  M.commands = {
    dashboard = function()
      if vim.g.neviraide_dashboard_displayed then
        require('nevitabs').close_buffer()
      else
        require('harbinger.open').open()
      end
    end,
  }

  for name, command in pairs(commands) do
    M.commands[name] = M.command(command)
  end

  vim.api.nvim_create_user_command('NeViRAIDE', function(args)
    local cmd = vim.trim(args.args or '')
    M.cmd(cmd)
  end, {
    nargs = '?',
    desc = 'NeViRAIDE',
    complete = function(_, line)
      if line:match('^%s*NeViRAIDE %w+ ') then return {} end
      local prefix = line:match('^%s*NeViRAIDE (%w*)') or ''
      return vim.tbl_filter(
        function(key) return key:find(prefix) == 1 end,
        vim.tbl_keys(M.commands)
      )
    end,
  })

  for name in pairs(M.commands) do
    local cmd = 'NeViRAIDE' .. name:sub(1, 1):upper() .. name:sub(2)
    vim.api.nvim_create_user_command(
      cmd,
      function() M.cmd(name) end,
      { desc = 'NeViRAIDE ' .. name }
    )
  end
end

return M
