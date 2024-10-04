use std::{collections::HashMap, sync::Arc};

use nvim_oxi::{
    api::{get_current_win, opts::SetKeymapOpts, types::Mode, Buffer},
    Result as OxiResult,
};

use crate::error::PluginError;

pub struct KeymapManager;

impl KeymapManager {
    pub fn setup_keymaps(
        buffer: &mut Buffer,
        first_button_line: usize,
        last_button_line: usize,
        command_mapping: Arc<HashMap<usize, String>>,
    ) -> Result<(), PluginError> {
        Self::set_cursor_movement_keymap(buffer, "k", -1, first_button_line, last_button_line)?;
        Self::set_cursor_movement_keymap(buffer, "j", 1, first_button_line, last_button_line)?;

        // Enter keymap
        buffer.set_keymap(
            Mode::Normal,
            "<CR>",
            "",
            &SetKeymapOpts::builder()
                .callback({
                    let command_mapping = Arc::clone(&command_mapping);
                    move |_| -> Result<(), PluginError> {
                        let win = get_current_win();
                        let (cur_line, _) = win.get_cursor()?;
                        if let Some(command) = command_mapping.get(&cur_line) {
                            nvim_oxi::api::command(command)?;
                        }
                        Ok(())
                    }
                })
                .build(),
        )?;

        Ok(())
    }

    fn set_cursor_movement_keymap(
        buffer: &mut Buffer,
        key: &str,
        direction: isize,
        first_button_line: usize,
        last_button_line: usize,
    ) -> Result<(), PluginError> {
        buffer.set_keymap(
            Mode::Normal,
            key,
            "",
            &SetKeymapOpts::builder()
                .callback({
                    move |_| -> Result<(), PluginError> {
                        let mut win = get_current_win();
                        let (cur_line, _) = win.get_cursor()?;
                        let target_line = Self::move_cursor(
                            cur_line,
                            first_button_line,
                            last_button_line,
                            direction,
                        );
                        win.set_cursor(target_line, 0)?;
                        Ok(())
                    }
                })
                .build(),
        )?;
        Ok(())
    }

    fn move_cursor(
        cur_line: usize,
        first_button_line: usize,
        last_button_line: usize,
        direction: isize,
    ) -> usize {
        let new_line = cur_line as isize + direction;
        if new_line < first_button_line as isize {
            last_button_line
        } else if new_line > last_button_line as isize {
            first_button_line
        } else {
            new_line as usize
        }
    }

    pub fn deactivate_keymaps(buffer: &mut Buffer) -> OxiResult<()> {
        let keys = ["gg", "G", "l", "h", "<Left>", "<Right>", "<up>", "<down>"];
        for key in &keys {
            buffer.set_keymap(Mode::Normal, key, "<Nop>", &Default::default())?;
        }
        Ok(())
    }
}
