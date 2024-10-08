```lua
{
  open_on_start = true,
  layout = {
    rows = {
      -- Первая строка: одна колонка на всю ширину
      {
        columns = {
          {
            size = 100,
            content = {
              type = "text",
              value = {
                "Header: Welcome to Harbinger!",
                "Customize your dashboard as you wish."
              },
              alignment = { horizontal = "center", vertical = "middle" },
            },
            active = false,
            highlights = {
              fg = "#FFFFFF",
              bg = "#1E1E1E",
            }
          }
        }
      },
      -- Вторая строка: три колонки
      {
        columns = {
          {
            size = 33,
            content = {
              type = "buttons",
              items = {
                { 
                  label = "New File", 
                  command = ":ene",  
                  icon = "📄",
                  highlights = {
                    fg = "#FF0000",
                    bg = "#1E1E1E",
                  }
                },
                { 
                  label = "Find File", 
                  command = ":Telescope find_files",
                  icon = "🔍",
                  highlights = {
                    fg = "#00FF00",
                    bg = "#1E1E1E",
                  }
                }
              }
            },
            alignment = { horizontal = "left", vertical = "middle" },
            active = true,
            highlights = {
              fg = "#FFFF00",
              bg = "#2E2E2E",
            }
          },
          {
            size = 34,
            content = {
              type = "group",  -- Группа из двух строк
              value = {
                {
                  size = 50,
                  content = {
                    type = "text",
                    value = {
                      "Subsection 1: Part of middle section, row 1"
                    },
                    alignment = { horizontal = "center", vertical = "middle" },
                  },
                  active = false,
                  highlights = {
                    fg = "#AAAAAA",
                    bg = "#1A1A1A",
                  }
                },
                {
                  size = 50,
                  content = {
                    type = "text",
                    value = {
                      "Subsection 2: Part of middle section, row 2"
                    },
                    alignment = { horizontal = "center", vertical = "middle" },
                  },
                  active = false,
                  highlights = {
                    fg = "#AAAAAA",
                    bg = "#1A1A1A",
                  }
                }
              }
            }
          },
          {
            size = 33,
            content = {
              type = "text",
              value = {
                "Helpful tips:",
                "Press `Ctrl-p` to open recent files",
                "Enjoy coding!"
              },
              alignment = { horizontal = "right", vertical = "middle" },
            },
            active = false,
            highlights = {
              fg = "#00FFFF",
              bg = "#000000",
            }
          }
        }
      }
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
