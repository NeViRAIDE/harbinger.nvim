local wk = require('which-key')
local i = require('stigmata.utils').icon

wk.register({
  ['<a-d>'] = {
    '<cmd>NeviraideUIDashboard<CR>',
    'Toggle dashboard' .. i('󰕮', 'tmux', 1),
  },
})
