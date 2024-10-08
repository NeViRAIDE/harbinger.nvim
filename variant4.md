```lua
{
  open_on_start = true,
  layout = {
    rows = {
      -- Первая строка с тремя колонками: левая разделена на 2 строки, центральная пустая, правая полная
      {
        columns = {
          {
            size = 33,
            content = {
              type = "group",
              value = {
                {
                  size = 50,
                  content = {
                    type = "text",
                    value = {
                      "Line 1: Part of left section, row 1",
                    },
                    alignment = { horizontal = "center", vertical = "middle" },
                  },
                  active = false,
                  highlights = {
                    fg = "#FFFFFF",
                    bg = "#000000",
                  }
                },
                {
                  size = 50,
                  content = {
                    type = "text",
                    value = {
                      "Line 2: Part of left section, row 2",
                    },
                    alignment = { horizontal = "center", vertical = "middle" },
                  },
                  active = false,
                  highlights = {
                    fg = "#FFFFFF",
                    bg = "#000000",
                  }
                }
              }
            }
          },
          {
            size = 33,
            content = {
              type = "none"
            },
            active = false,
          },
          {
            size = 33,
            content = {
              type = "text",
              value = {
                "Right Full Section",
              },
              alignment = { horizontal = "center", vertical = "middle" },
            },
            active = false,
            highlights = {
              fg = "#00FF00",
              bg = "#222222",
            }
          }
        }
      },
    }
  },
  borders = "rounded",  -- Set border style for active sections, e.g., 'rounded', 'single', 'double', etc.
  keymaps = {
    toggle_dashboard = "<leader>db",  -- Keymap to toggle the dashboard visibility.
    navigate_sections = "<Tab>",      -- Keymap to move between active sections.
    navigate_buttons = "<Down>",      -- Keymap to move between buttons within a section.
    execute_button = "<Enter>",       -- Keymap to execute the command associated with a button.
  }
}
```
