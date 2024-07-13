local wk = require('which-key')
local i = require('stigmata.utils').icon

wk.add({
  {
    '<a-d>',
    '<cmd>NeViRAIDEDashboard<CR>',
    desc = 'Toggle dashboard',
    icon = i('󰕮', 'tmux'),
  },
})
