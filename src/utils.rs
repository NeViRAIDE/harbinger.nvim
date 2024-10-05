use std::{path::PathBuf, sync::OnceLock};

use nvim_oxi::api::{get_option_value, get_vvar, list_bufs, opts::OptionOpts, Buffer};

use crate::error::{handle_error, PluginError};

static CACHED_DIMENSIONS: OnceLock<(usize, usize)> = OnceLock::new();

pub fn should_open_dashboard() -> bool {
    // Check if there are command-line arguments
    let argc: i64 = get_vvar("argc").unwrap_or(0);
    if argc > 0 {
        return false;
    }

    // Get the list of buffers and collect into a Vec
    let buffers: Vec<Buffer> = list_bufs().collect();

    if buffers.len() == 1 {
        let buffer = &buffers[0];
        // Check if the buffer is unnamed (no file associated)
        if buffer.get_name().unwrap_or_default() == PathBuf::new() {
            return true;
        }
    }

    false
}

pub fn get_window_size() -> Result<(usize, usize), PluginError> {
    if let Some(&(cached_width, cached_height)) = CACHED_DIMENSIONS.get() {
        return Ok((cached_width, cached_height));
    }

    let win = nvim_oxi::api::get_current_win();

    let win_height: usize = handle_error(
        win.get_height().map_err(PluginError::Api)?.try_into(),
        "Failed to convert window height to usize",
    )?;

    let win_width: usize = handle_error(
        win.get_width().map_err(PluginError::Api)?.try_into(),
        "Failed to convert window width to usize",
    )?;

    CACHED_DIMENSIONS.set((win_width, win_height)).ok();

    Ok((win_width, win_height))
}

pub fn count_file_buffers() -> usize {
    nvim_oxi::api::list_bufs()
        .filter(|buf| {
            let buf_type: String = get_option_value(
                "buftype",
                &OptionOpts::builder().buffer(buf.clone()).build(),
            )
            .unwrap_or_default();

            let is_valid = buf.is_valid();
            let is_normal = buf_type.is_empty();

            is_valid && is_normal
        })
        .count()
}
