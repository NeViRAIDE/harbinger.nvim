use nvim_oxi::{
    api::{
        opts::{BufDeleteOpts, OptionOpts, OptionScope},
        set_option_value, Buffer,
    },
    Result as OxiResult,
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

    pub fn configure_buffer() -> OxiResult<()> {
        let buf_opts = OptionOpts::builder().scope(OptionScope::Local).build();

        handle_error(
            set_option_value("number", false, &buf_opts),
            "Failed to set 'number' option",
        )?;

        handle_error(
            set_option_value("relativenumber", false, &buf_opts),
            "Failed to set 'relativenumber' option",
        )?;

        handle_error(
            set_option_value("filetype", "harbinger", &buf_opts),
            "Failed to set 'filetype'",
        )?;

        Ok(())
    }

    pub fn delete_buffer(buf: &Buffer) -> OxiResult<()> {
        let buf_del_opts = BufDeleteOpts::builder().force(true).build();
        handle_error(buf.clone().delete(&buf_del_opts), "Failed to delete buffer")?;
        Ok(())
    }
}
