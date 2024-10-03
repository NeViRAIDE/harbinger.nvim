use crate::error::{handle_error, PluginError};
use nvim_oxi::{
    api::{get_current_win, opts::SetKeymapOpts, types::Mode, Buffer},
    Result as OxiResult,
};

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
            Self::move_up_cursor,
            first_button_index,
            last_button_index,
            raw_height,
        )?;
        Self::set_cursor_movement_keymap(
            buffer,
            "j",
            Self::move_down_cursor,
            first_button_index,
            last_button_index,
            raw_height,
        )?;
        Ok(())
    }

    fn set_cursor_movement_keymap<F>(
        buffer: &mut Buffer,
        key: &str,
        movement_fn: F,
        first_button_index: usize,
        last_button_index: usize,
        raw_height: usize,
    ) -> Result<(), PluginError>
    where
        F: Fn(usize, usize, usize, usize) -> usize + 'static,
    {
        handle_error(
            buffer.set_keymap(
                Mode::Normal,
                key,
                "",
                &SetKeymapOpts::builder()
                    .callback({
                        move |_| -> Result<(), PluginError> {
                            let mut win = get_current_win();
                            let (cur, _) =
                                handle_error(win.get_cursor(), "Failed to get cursor position")?;
                            let target_line =
                                movement_fn(cur, first_button_index, last_button_index, raw_height);
                            handle_error(
                                win.set_cursor(target_line, 0),
                                "Failed to set cursor position",
                            )?;
                            Ok(())
                        }
                    })
                    .build(),
            ),
            &format!("Failed to set keymap for '{}'", key),
        )
    }

    fn move_up_cursor(
        cur: usize,
        first_button_index: usize,
        last_button_index: usize,
        raw_height: usize,
    ) -> usize {
        if cur == first_button_index {
            last_button_index
        } else {
            cur - raw_height
        }
    }

    fn move_down_cursor(
        cur: usize,
        first_button_index: usize,
        last_button_index: usize,
        raw_height: usize,
    ) -> usize {
        if cur == last_button_index {
            first_button_index
        } else {
            cur + raw_height
        }
    }

    pub fn deactivate_keymaps(buffer: &mut Buffer) -> OxiResult<()> {
        let keys = ["gg", "G", "l", "h", "<Left>", "<Right>"];
        for key in &keys {
            Buffer::set_keymap(buffer, Mode::Normal, key, "", &Default::default())?;
        }

        Buffer::set_keymap(buffer, Mode::Normal, "<up>", "k", &Default::default())?;
        Buffer::set_keymap(buffer, Mode::Normal, "<down>", "j", &Default::default())?;

        Ok(())
    }
}
