```lua
{
  open_on_start = true,
  layout = {
    rows = {
      {
        columns = {
          -- Первая колонка (разделена на три строки)
          {
            size = 33,
            content = {
              type = "group",
              value = {
                {
                  size = 33,
                  content = {
                    type = "text",
                    value = {
                      "Section 1.1: Part of left column, row 1"
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
                  size = 33,
                  content = {
                    type = "text",
                    value = {
                      "Section 1.2: Part of left column, row 2"
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
                  size = 34,
                  content = {
                    type = "group",
                    value = {
                      {
                        size = 50,
                        content = {
                          type = "text",
                          value = {
                            "Subsection 1.3.1: Left half of third row"
                          },
                          alignment = { horizontal = "center", vertical = "middle" },
                        },
                        active = false,
                        highlights = {
                          fg = "#FF0000",
                          bg = "#2E2E2E",
                        }
                      },
                      {
                        size = 50,
                        content = {
                          type = "text",
                          value = {
                            "Subsection 1.3.2: Right half of third row"
                          },
                          alignment = { horizontal = "center", vertical = "middle" },
                        },
                        active = false,
                        highlights = {
                          fg = "#00FF00",
                          bg = "#2E2E2E",
                        }
                      }
                    }
                  }
                }
              }
            }
          },
          -- Вторая колонка (во всю высоту)
          {
            size = 33,
            content = {
              type = "text",
              value = {
                "Section 2: Middle column, full height"
              },
              alignment = { horizontal = "center", vertical = "middle" },
            },
            active = false,
            highlights = {
              fg = "#00FFFF",
              bg = "#000000",
            }
          },
          -- Третья колонка (во всю высоту)
          {
            size = 33,
            content = {
              type = "text",
              value = {
                "Section 3: Right column, full height"
              },
              alignment = { horizontal = "center", vertical = "middle" },
            },
            active = false,
            highlights = {
              fg = "#FF00FF",
              bg = "#333333",
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
