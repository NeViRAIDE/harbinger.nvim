# Harbinger
*Startup neovim dashboard*

## Installation

With [lazy.nvim](https://github.com/folke/lazy.nvim):
```lua
{
    'NeViRAIE/harbinger.nvim',
    build = 'chmod +x ./install.sh && ./install.sh'
    event = 'VimEnter',
    config = true       -- if you want to use default options
                        -- otherwise use `opts = { ...your options... }` 
}
```

## Default options

> [!NOTE]
>
> `<M-d>` = `Alt + d`
> 
> available positions are: `left`, `right`, `center`

```lua
{
  "RAprogramm/harbinger",
  config = function()
    require('harbinger').setup(
{
  open_on_start = true,
  layout = {
    rows = {
      -- Первая строка на всю ширину
      {
        columns = {
          {
            size = 100,
            content = {
              type = "text",
              value = {
                "Welcome to the Harbinger Dashboard!",
                "This is a fully customizable interface."
              },
              alignment = { horizontal = "center", vertical = "top" },
            },
            active = false,
            highlights = {
              fg = "#FFFFFF",
              bg = "#000000",
            }
          }
        }
      },
      -- Вторая строка с разделением на три колонки
      {
        columns = {
          {
            size = 33,
            rows = {
              {
                size = 33,
                content = {
                  type = "text",
                  value = {
                    "Left Column - Row 1"
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
                size = 33,
                content = {
                  type = "text",
                  value = {
                    "Left Column - Row 2"
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
                size = 34,
                columns = {
                  {
                    size = 50,
                    content = {
                      type = "text",
                      value = {
                        "Subsection - Left Part of Third Row"
                      },
                      alignment = { horizontal = "center", vertical = "middle" },
                    },
                    active = false,
                    highlights = {
                      fg = "#00FF00",
                      bg = "#444444",
                    }
                  },
                  {
                    size = 50,
                    content = {
                      type = "text",
                      value = {
                        "Subsection - Right Part of Third Row"
                      },
                      alignment = { horizontal = "center", vertical = "middle" },
                    },
                    active = false,
                    highlights = {
                      fg = "#00FFFF",
                      bg = "#555555",
                    }
                  }
                }
              }
            }
          },
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
                    fg = "#FF00FF",
                    bg = "#666666",
                  }
                },
                {
                  label = "Find File",
                  command = ":Telescope find_files",
                  icon = "🔍",
                  highlights = {
                    fg = "#FFFF00",
                    bg = "#777777",
                  }
                }
              }
            },
            active = true,
            highlights = {
              fg = "#00FFFF",
              bg = "#000000",
            }
          },
          {
            size = 34,
            rows = {
              {
                size = 50,
                content = {
                  type = "text",
                  value = {
                    "Right Column - Top Row"
                  },
                  alignment = { horizontal = "center", vertical = "middle" },
                },
                active = false,
                highlights = {
                  fg = "#FF9900",
                  bg = "#888888",
                }
              },
              {
                size = 50,
                content = {
                  type = "text",
                  value = {
                    "Right Column - Bottom Row"
                  },
                  alignment = { horizontal = "center", vertical = "middle" },
                },
                active = false,
                highlights = {
                  fg = "#99FF00",
                  bg = "#999999",
                }
              }
            }
          }
        }
      }
    }
  },
  borders = "rounded",  -- Настройка границ для активных секций
  keymaps = {
    toggle_dashboard = "<leader>db",  -- Клавиша для переключения дашборда
    navigate_sections = "<Tab>",      -- Клавиша для перехода между секциями
    navigate_buttons = "<Down>",      -- Клавиша для перехода по кнопкам
    execute_button = "<Enter>",       -- Клавиша для выполнения команды кнопки
  }
}
)
  end,
}
```

## Dashboard elements configurations

### Header/Subheader/Footer

```lua
opts = {
    header = {
        text = 'Your own header'
    }
}
```
or
```lua
opts = {
    header = {
        text = 'Your\nown\nheader'
    }
}
```
or
```lua
opts = {
    header = {
        text = {
            'Your',
            'Own',
            'Header'
        }
    }
}
```
