use nvim_oxi::{
    api::{
        opts::{BufDeleteOpts, OptionOpts, OptionScope},
        set_option_value,
        types::Mode,
        Buffer,
    },
    Object, Result as OxiResult,
};

use crate::error::handle_error;

pub struct BufferManager;

impl BufferManager {
    pub fn set_buffer_content(buf: &mut Buffer, content: &str) -> OxiResult<()> {
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        handle_error(
            buf.set_lines(0.., true, lines),
            "Failed to set buffer lines",
        )?;
        Ok(())
    }

    pub fn configure_buffer(buf: &mut Buffer) -> OxiResult<()> {
        let buf_opts = OptionOpts::builder().scope(OptionScope::Local).build();

        let options: Vec<(&str, Box<Object>)> = vec![
            ("number", Box::new(false.into())),
            ("relativenumber", Box::new(false.into())),
            ("filetype", Box::new("harbinger".into())),
            ("modifiable", Box::new(false.into())),
            ("wrap", Box::new(false.into())),
            ("spell", Box::new(false.into())),
            ("list", Box::new(false.into())),
            ("cursorcolumn", Box::new(false.into())),
            ("swapfile", Box::new(false.into())),
            ("matchpairs", Box::new("".into())),
        ];

        for (option, value) in options {
            handle_error(
                set_option_value(option, *value, &buf_opts),
                &format!("Failed to set '{}' option", option),
            )?;
        }

        Self::deactivate_keymaps(buf)?;

        Ok(())
    }

    fn deactivate_keymaps(buf: &mut Buffer) -> OxiResult<()> {
        let keys = vec!["gg", "G", "l", "h", "<Left>", "<Right>"];

        for key in keys {
            Buffer::set_keymap(buf, Mode::Normal, key, "", &Default::default())?;
        }

        Ok(())
    }

    pub fn delete_buffer(buf: &Buffer) -> OxiResult<()> {
        let buf_del_opts = BufDeleteOpts::builder().force(true).build();
        handle_error(buf.clone().delete(&buf_del_opts), "Failed to delete buffer")?;
        Ok(())
    }
}
