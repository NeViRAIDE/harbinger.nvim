use std::{collections::HashMap, sync::Arc};

use nvim_oxi::{
    api::{err_writeln, get_current_win, opts::SetKeymapOpts, types::Mode, Buffer},
    Function, Result as OxiResult,
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
                .callback(Function::from_fn({
                    let command_mapping = Arc::clone(&command_mapping);
                    move |_| -> Result<(), nvim_oxi::Error> {
                        let win = get_current_win();
                        let (cur_line, _) = win.get_cursor()?;
                        if let Some(command) = command_mapping.get(&cur_line) {
                            if let Err(e) = nvim_oxi::api::command(command) {
                                err_writeln(&format!(
                                    "Error executing command '{}': {}",
                                    command, e
                                ));
                            }
                        }
                        Ok(())
                    }
                }))
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
                .callback(Function::from_fn({
                    let buffer = buffer.clone();
                    move |_| -> Result<(), nvim_oxi::Error> {
                        let mut win = get_current_win();
                        let (cur_line, _) = win.get_cursor()?;
                        let target_line = Self::move_cursor(
                            cur_line,
                            first_button_line,
                            last_button_line,
                            direction,
                        );

                        // Adjust the cursor column
                        let line_iter = buffer.get_lines(target_line - 1..target_line, false)?;
                        let line_content: Vec<String> = line_iter.map(|s| s.to_string()).collect();

                        let col = if let Some(line) = line_content.first() {
                            line.chars().take_while(|c| c.is_whitespace()).count()
                        } else {
                            0
                        };

                        win.set_cursor(target_line, col)?;
                        Ok(())
                    }
                }))
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
        let keys = [
            "gg", "G", "L", "H", "l", "h", "<Left>", "<Right>", "<up>", "<down>",
        ];
        for key in &keys {
            buffer.set_keymap(Mode::Normal, key, "<Nop>", &Default::default())?;
        }
        Ok(())
    }
}
