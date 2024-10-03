use std::sync::OnceLock;

use crate::error::{handle_error, PluginError};

static CACHED_DIMENSIONS: OnceLock<(usize, usize)> = OnceLock::new();

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
