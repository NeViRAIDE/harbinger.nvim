use nvim_oxi::api::{opts::SetHighlightOpts, set_hl};

use crate::error::{handle_error, PluginError};

pub fn setup_highlight_groups() -> Result<(), PluginError> {
    handle_error(
        set_hl(
            0,
            "HarbingerHeader",
            &SetHighlightOpts::builder()
                .foreground("#FF0000")
                .bold(true)
                .build(),
        ),
        "Failed to set highlight for HarbingerHeader",
    )?;

    handle_error(
        set_hl(
            0,
            "HarbingerSubHeader",
            &SetHighlightOpts::builder()
                .link("Comment")
                .italic(true)
                .build(),
        ),
        "Failed to set highlight for HarbingerHeader",
    )?;

    handle_error(
        set_hl(
            0,
            "HarbingerButton",
            &SetHighlightOpts::builder()
                .foreground("#00FF00")
                .italic(true)
                .build(),
        ),
        "Failed to set highlight for HarbingerButton",
    )?;

    handle_error(
        set_hl(
            0,
            "HarbingerFooter",
            &SetHighlightOpts::builder()
                .link("Comment")
                .bold(true) // Жирный шрифт
                .build(),
        ),
        "Failed to set highlight for HarbingerFooter",
    )?;

    Ok(())
}
