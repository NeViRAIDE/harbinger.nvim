use nvim_oxi::{
    api::{get_current_win, opts::SetKeymapOpts, types::Mode, Buffer},
    Result as OxiResult,
};

use crate::error::PluginError;

pub struct KeymapManager;

impl KeymapManager {
    pub fn setup_keymaps(
        buffer: &mut Buffer,
        first_button_index: usize,
        last_button_index: usize,
        raw_height: usize,
    ) -> Result<(), PluginError> {
        Self::set_cursor_movement_keymap(
            buffer,
            "k",
            -1,
            first_button_index,
            last_button_index,
            raw_height,
        )?;
        Self::set_cursor_movement_keymap(
            buffer,
            "j",
            1,
            first_button_index,
            last_button_index,
            raw_height,
        )?;
        Ok(())
    }

    fn set_cursor_movement_keymap(
        buffer: &mut Buffer,
        key: &str,
        direction: isize,
        first_button_index: usize,
        last_button_index: usize,
        raw_height: usize,
    ) -> Result<(), PluginError> {
        buffer.set_keymap(
            Mode::Normal,
            key,
            "",
            &SetKeymapOpts::builder()
                .callback({
                    move |_| -> Result<(), PluginError> {
                        let mut win = get_current_win();
                        let (cur, _) = win.get_cursor()?;
                        let target_line = Self::move_cursor(
                            cur,
                            first_button_index,
                            last_button_index,
                            raw_height,
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
        cur: usize,
        first_button_index: usize,
        last_button_index: usize,
        raw_height: usize,
        direction: isize,
    ) -> usize {
        let new_line = cur as isize + direction * raw_height as isize;
        if new_line < first_button_index as isize {
            last_button_index
        } else if new_line > last_button_index as isize {
            first_button_index
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
