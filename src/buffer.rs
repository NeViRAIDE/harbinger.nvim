use nvim_oxi::{
    api::{
        err_writeln,
        opts::{BufDeleteOpts, OptionOpts, OptionScope},
        set_option_value, Buffer,
    },
    Result as OxiResult,
};

pub struct BufferManager;

impl BufferManager {
    pub fn set_buffer_content(buf: &mut Buffer, content: &str) -> OxiResult<()> {
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        buf.set_lines(0.., true, lines)?;
        Ok(())
    }

    pub fn configure_buffer() -> OxiResult<()> {
        let buf_opts = OptionOpts::builder().scope(OptionScope::Local).build();

        if let Err(e) = set_option_value("number", false, &buf_opts) {
            err_writeln(&format!("Failed to set 'number' option: {}", e));
        }

        if let Err(e) = set_option_value("relativenumber", false, &buf_opts) {
            err_writeln(&format!("Failed to set 'relativenumber' option: {}", e));
        }

        if let Err(e) = set_option_value("filetype", "harbinger", &buf_opts) {
            err_writeln(&format!("Failed to set 'filetype': {}", e));
        }

        Ok(())
    }

    pub fn delete_buffer(buf: &Buffer) -> OxiResult<()> {
        let buf_del_opts = BufDeleteOpts::builder().force(true).build();
        buf.clone().delete(&buf_del_opts)?;
        Ok(())
    }
}
