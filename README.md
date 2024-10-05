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
    open_on_start = true,
    keymap = "<M-d>",
    header = {
        text = "Welcome to Neovim!",
        position = "center",
    },
    sub_header = {
        text = "==================",
        position = "center",
    },
    footer = {
        text = "Created with Rust by RAprogramm",
        position = "center",
    },
    buttons = {
        items = {
            { "Create new file", "", "edit new_file.txt" },
            { "Find file", "", "Telescope find_files" },
            { "Recent files", "", "Telescope oldfiles" },
            { "Exit", "X", "qall" },
        },
        position = "center",
    },
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
