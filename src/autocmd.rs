use nvim_oxi::{
    api::{
        create_autocmd, get_current_buf, get_option_value,
        opts::{CreateAutocmdOpts, OptionOpts, OptionScope},
        Buffer,
    },
    Function, Result as OxiResult,
};

use crate::buffer::BufferManager;

pub fn buffer_delete(dashboard_buf: Buffer) -> OxiResult<()> {
    let option_opts = OptionOpts::builder().scope(OptionScope::Global).build();

    let autocmd_opts = CreateAutocmdOpts::builder()
        .callback(Function::from_fn(move |_| -> OxiResult<bool> {
            // Get the current buffer
            let current_buf = get_current_buf();

            // Get the 'buftype' of the current buffer
            let buf_type: String = get_option_value("buftype", &option_opts)?;

            // Check if the current buffer is a normal file buffer and not the dashboard buffer
            if buf_type.is_empty() && current_buf != dashboard_buf {
                // Check if the dashboard buffer is valid
                if dashboard_buf.is_valid() {
                    // Delete the dashboard buffer
                    BufferManager::delete_buffer(&dashboard_buf)?;
                }
            }

            Ok(true)
        }))
        .once(false)
        .build();

    create_autocmd(["BufEnter"], &autocmd_opts)?;

    Ok(())
}
