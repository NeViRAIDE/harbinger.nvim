use std::{collections::HashMap, sync::Arc};

use keymap::KeymapManager;
use nvim_oxi::{
    api::{
        get_current_win, get_option_value,
        opts::{OptionOpts, OptionScope},
        set_option_value, Buffer,
    },
    Object, Result as OxiResult,
};

mod keymap;

pub struct BufferManager;

impl BufferManager {
    pub fn set_buffer_content(buf: &mut Buffer, content: &str) -> OxiResult<usize> {
        let lines: Vec<String> = content.lines().map(String::from).collect();

        let win = get_current_win();
        let win_height = win.get_height()? as usize;

        let content_height = lines.len();
        let row = Self::get_centered_position(win_height as u32, content_height)?;

        let mut result = vec!["".to_string(); row];
        result.extend(lines);
        result.resize(win_height, "".to_string());

        buf.set_lines(0.., true, result)?;

        Ok(row) // Return the number of top padding lines
    }

    pub fn configure_buffer(
        buf: &mut Buffer,
        first_button_line: usize,
        last_button_line: usize,
        command_mapping: Arc<HashMap<usize, String>>,
    ) -> OxiResult<()> {
        let options = Self::get_buffer_options();
        Self::set_buffer_options(&options)?;
        KeymapManager::deactivate_keymaps(buf)?;
        KeymapManager::setup_keymaps(buf, first_button_line, last_button_line, command_mapping)?;
        Ok(())
    }

    fn get_buffer_options() -> Vec<(&'static str, Object)> {
        vec![
            ("number", false.into()),
            ("relativenumber", false.into()),
            ("filetype", "harbinger".into()),
            ("modifiable", false.into()),
            ("wrap", false.into()),
            ("spell", false.into()),
            ("list", false.into()),
            ("cursorcolumn", false.into()),
            ("swapfile", false.into()),
            ("bufhidden", "wipe".into()),
            ("buftype", "nofile".into()),
            ("buflisted", false.into()), // Add this line
        ]
    }

    fn set_buffer_options(options: &[(&str, Object)]) -> OxiResult<()> {
        let buf_opts = OptionOpts::builder().scope(OptionScope::Local).build();
        for &(name, ref value) in options {
            set_option_value(name, value.clone(), &buf_opts)?;
        }
        Ok(())
    }

    pub fn get_centered_position(win_height: u32, content_height: usize) -> OxiResult<usize> {
        let row = if win_height as usize > content_height {
            (win_height as usize - content_height) / 2
        } else {
            0
        };
        Ok(row)
    }

    pub fn delete_buffer(buf: &Buffer) -> OxiResult<()> {
        let buf_ft: String = get_option_value(
            "filetype",
            &OptionOpts::builder()
                // .scope(OptionScope::Local)
                .buffer(buf.clone())
                .build(),
        )?;

        if buf_ft == "harbinger" {
            // Only delete the buffer if it's the dashboard
            buf.clone().delete(&Default::default())?;
        }

        Ok(())
    }
}
