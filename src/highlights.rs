// use nvim_oxi::api::create_namespace;
use nvim_oxi::api::{opts::SetHighlightOpts, set_hl};

use crate::error::{handle_error, PluginError};

pub fn setup_highlight_groups() -> Result<(), PluginError> {
    nvim_oxi::print!("Set up highlightings");
    // let ns_id = create_namespace("harbinger");
    // Создаем группу подсветки для заголовков
    let header_highlight = SetHighlightOpts::builder()
        .foreground("#FF0000")
        .bold(true)
        .build();

    handle_error(
        set_hl(0, "HarbingerHeader", &header_highlight),
        "Failed to set highlight for HarbingerHeader",
    )?;

    nvim_oxi::api::command("highlight HarbingerHeader guifg=#FF00FF")?;

    // Создаем группу подсветки для кнопок
    let button_highlight = SetHighlightOpts::builder()
        .foreground("#00FF00")
        .background("#000000")
        .italic(true)
        .build();

    handle_error(
        set_hl(0, "HarbingerButton", &button_highlight),
        "Failed to set highlight for HarbingerButton",
    )?;

    let footer_highlight = SetHighlightOpts::builder()
        .foreground("#FF0000") // Цвет текста
        .background("#000000") // Цвет фона
        .bold(true) // Жирный шрифт
        .build();

    handle_error(
        set_hl(0, "HarbingerFooter", &footer_highlight),
        "Failed to set highlight for HarbingerFooter",
    )?;

    Ok(())
}
