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
                    type = "text",
                    value = {
                      "Section 1.3: Part of left column, row 3"
                    },
                    alignment = { horizontal = "center", vertical = "middle" },
                  },
                  active = false,
                  highlights = {
                    fg = "#FF0000",
                    bg = "#2E2E2E",
                  }
                }
              }
            }
          },
          -- Вторая колонка (разделена на две строки)
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
                      "Section 2.1: Top part of middle column"
                    },
                    alignment = { horizontal = "center", vertical = "middle" },
                  },
                  active = false,
                  highlights = {
                    fg = "#00FFFF",
                    bg = "#333333",
                  }
                },
                {
                  size = 50,
                  content = {
                    type = "group",
                    value = {
                      -- Вторая строка разделена на две колонки
                      {
                        size = 50,
                        content = {
                          type = "text",
                          value = {
                            "Subsection 2.2.1: Left part of second row"
                          },
                          alignment = { horizontal = "center", vertical = "middle" },
                        },
                        active = false,
                        highlights = {
                          fg = "#FF00FF",
                          bg = "#444444",
                        }
                      },
                      {
                        size = 50,
                        content = {
                          type = "group",
                          value = {
                            {
                              size = 50,
                              content = {
                                type = "text",
                                value = {
                                  "Subsection 2.2.2.1: Top part of right half"
                                },
                                alignment = { horizontal = "center", vertical = "middle" },
                              },
                              active = false,
                              highlights = {
                                fg = "#FF9900",
                                bg = "#555555",
                              }
                            },
                            {
                              size = 50,
                              content = {
                                type = "text",
                                value = {
                                  "Subsection 2.2.2.2: Bottom part of right half"
                                },
                                alignment = { horizontal = "center", vertical = "middle" },
                              },
                              active = false,
                              highlights = {
                                fg = "#99FF00",
                                bg = "#666666",
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          },
          -- Третья колонка (во всю высоту)
          {
            size = 34,
            content = {
              type = "text",
              value = {
                "Section 3: Right column, full height"
              },
              alignment = { horizontal = "center", vertical = "middle" },
            },
            active = false,
            highlights = {
              fg = "#00FF00",
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
