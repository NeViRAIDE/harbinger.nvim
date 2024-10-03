use keymap::KeymapManager;
use nvim_oxi::{
    api::{
        get_current_win,
        opts::{BufDeleteOpts, OptionOpts, OptionScope},
        set_option_value, Buffer,
    },
    Object, Result as OxiResult,
};

use crate::error::{handle_error, PluginError};

mod keymap;

pub struct BufferManager;

struct BufferOption<'a> {
    name: &'a str,
    value: Object,
}

impl BufferManager {
    pub fn set_buffer_content(buf: &mut Buffer, content: &str) -> OxiResult<()> {
        let lines: Vec<String> = content.lines().map(String::from).collect();

        let win = get_current_win();
        let win_height = handle_error(win.get_height(), "Failed to get window height")?;

        let max_height = std::cmp::max(win_height as usize, lines.len());
        let mut result = vec!["".to_string(); max_height];

        let content_height = lines.len();
        let (row, _) = Self::get_centered_position(win_height, content_height, 0)?;

        for (i, line) in lines.iter().enumerate() {
            result[row + i] = line.clone();
        }

        handle_error(
            buf.set_lines(0.., true, result),
            "Failed to set buffer lines",
        )?;
        Ok(())
    }

    pub fn configure_buffer(
        buf: &mut Buffer,
        first_button_index: usize,
        last_button_index: usize,
    ) -> OxiResult<()> {
        let options = Self::get_buffer_options();
        Self::set_buffer_options(&options)?;
        KeymapManager::deactivate_keymaps(buf)?;
        KeymapManager::setup_keymaps(buf, first_button_index, last_button_index, 1)?;
        Ok(())
    }

    fn get_buffer_options() -> [BufferOption<'static>; 9] {
        [
            BufferOption {
                name: "number",
                value: Object::from(false),
            },
            BufferOption {
                name: "relativenumber",
                value: Object::from(false),
            },
            BufferOption {
                name: "filetype",
                value: Object::from("harbinger"),
            },
            BufferOption {
                name: "modifiable",
                value: Object::from(false),
            },
            BufferOption {
                name: "wrap",
                value: Object::from(false),
            },
            BufferOption {
                name: "spell",
                value: Object::from(false),
            },
            BufferOption {
                name: "list",
                value: Object::from(false),
            },
            BufferOption {
                name: "cursorcolumn",
                value: Object::from(false),
            },
            BufferOption {
                name: "swapfile",
                value: Object::from(false),
            },
        ]
    }

    fn set_buffer_options(options: &[BufferOption]) -> OxiResult<()> {
        let buf_opts = OptionOpts::builder().scope(OptionScope::Local).build();
        for option in options {
            handle_error(
                set_option_value(option.name, option.value.clone(), &buf_opts),
                &format!("Failed to set '{}' option", option.name),
            )?;
        }
        Ok(())
    }

    pub fn get_centered_position(
        win_height: u32,

        content_height: usize,
        _max_line_length: usize,
    ) -> Result<(usize, usize), PluginError> {
        let row = if win_height as usize > content_height {
            (win_height as usize - content_height) / 2
        } else {
            0
        };

        Ok((row, 0))
    }

    pub fn delete_buffer(buf: &Buffer) -> OxiResult<()> {
        let buf_del_opts = BufDeleteOpts::builder().force(true).build();
        handle_error(buf.clone().delete(&buf_del_opts), "Failed to delete buffer")?;
        Ok(())
    }
}
