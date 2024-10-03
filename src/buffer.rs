use std::sync::OnceLock;

use keymap::KeymapManager;
use nvim_oxi::{
    api::{
        get_current_win, get_option_value,
        opts::{BufDeleteOpts, OptionOpts, OptionScope},
        set_option_value, Buffer,
    },
    Object, Result as OxiResult,
};

use crate::error::{handle_error, PluginError};

mod keymap;

pub struct BufferManager;

struct BufferOption<'a> {
    name: &'a str,
    value: Object,
}

static CACHED_DIMENSIONS: OnceLock<(usize, usize)> = OnceLock::new();

impl BufferManager {
    pub fn set_buffer_content(buf: &mut Buffer, content: &str) -> OxiResult<()> {
        let lines: Vec<String> = content.lines().map(String::from).collect();

        // Получаем текущее окно и его размеры
        let win = get_current_win();
        let win_height = handle_error(win.get_height(), "Failed to get window height")?;
        let win_width = handle_error(win.get_width(), "Failed to get window width")?;

        nvim_oxi::print!("Window size: {}x{}", win_width, win_height);

        // Определяем максимальное количество строк для буфера
        let max_height = std::cmp::max(win_height as usize, lines.len());
        let mut result = vec!["".to_string(); max_height]; // Инициализируем буфер пустыми строками

        // Получаем координаты для центрирования содержимого по вертикали
        let content_height = lines.len();
        let (row, _) = Self::get_centered_position(win_height, win_width, content_height, 0)?;
        nvim_oxi::print!(
            "Center position: row = {}, win_height = {}, win_width = {}",
            row,
            win_height,
            win_width
        );

        // Заполняем строки содержимым, добавляя пробелы для горизонтального центрирования
        for (i, line) in lines.iter().enumerate() {
            let col = if win_width as usize > line.len() {
                (win_width as usize - line.len()) / 2 // Вычисляем отступ для каждой строки отдельно
            } else {
                0
            };

            let padded_line = format!("{:width$}{}", "", line, width = col); // Горизонтальное центрирование
            result[row + i] = padded_line.clone();
            nvim_oxi::print!("Padded line (centered): '{}'", padded_line); // Логирование
        }

        // Логируем итоговое количество строк
        nvim_oxi::print!("Final number of lines: {}", result.len());

        // Устанавливаем строки в буфер
        handle_error(
            buf.set_lines(0.., true, result),
            "Failed to set buffer lines",
        )?;
        Ok(())
    }

    /// Конфигурирует буфер
    pub fn configure_buffer(
        buf: &mut Buffer,
        first_button_index: usize,
        last_button_index: usize,
    ) -> OxiResult<()> {
        let options = Self::get_buffer_options();
        Self::set_buffer_options(&options)?;
        KeymapManager::deactivate_keymaps(buf)?;
        KeymapManager::setup_keymaps(buf, first_button_index, last_button_index, 1)?;
        Ok(())
    }

    /// Возвращает набор опций для буфера
    fn get_buffer_options() -> [BufferOption<'static>; 9] {
        [
            BufferOption {
                name: "number",
                value: Object::from(false),
            },
            BufferOption {
                name: "relativenumber",
                value: Object::from(false),
            },
            BufferOption {
                name: "filetype",
                value: Object::from("harbinger"),
            },
            BufferOption {
                name: "modifiable",
                value: Object::from(false),
            },
            BufferOption {
                name: "wrap",
                value: Object::from(false),
            },
            BufferOption {
                name: "spell",
                value: Object::from(false),
            },
            BufferOption {
                name: "list",
                value: Object::from(false),
            },
            BufferOption {
                name: "cursorcolumn",
                value: Object::from(false),
            },
            BufferOption {
                name: "swapfile",
                value: Object::from(false),
            },
        ]
    }

    /// Устанавливает опции для буфера
    fn set_buffer_options(options: &[BufferOption]) -> OxiResult<()> {
        let buf_opts = OptionOpts::builder().scope(OptionScope::Local).build();
        for option in options {
            handle_error(
                set_option_value(option.name, option.value.clone(), &buf_opts),
                &format!("Failed to set '{}' option", option.name),
            )?;
        }
        Ok(())
    }

    pub fn get_centered_position(
        win_height: u32,
        win_width: u32,
        content_height: usize,
        max_line_length: usize,
    ) -> Result<(usize, usize), PluginError> {
        let dimensions = CACHED_DIMENSIONS.get_or_init(|| {
            let editor_height = get_option_value("lines", &OptionOpts::default()).unwrap_or(25);
            let editor_width = get_option_value("columns", &OptionOpts::default()).unwrap_or(80);

            nvim_oxi::print!("Editor height: {}, width: {}", editor_height, editor_width);
            (editor_height, editor_width)
        });

        nvim_oxi::print!(
            "Using editor dimensions: editor_height = {}, editor_width = {}",
            dimensions.0,
            dimensions.1
        );

        nvim_oxi::print!(
            "Window dimensions: win_height = {}, win_width = {}",
            win_height,
            win_width
        );

        // Центрирование по вертикали
        let row = if win_height as usize > content_height {
            (win_height as usize - content_height) / 2
        } else {
            0
        };

        // Центрирование по горизонтали (учитывая самую длинную строку)
        let col = if win_width as usize > max_line_length {
            (win_width as usize - max_line_length) / 2
        } else {
            0
        };

        nvim_oxi::print!("Calculated center: row = {}, col = {}", row, col);

        Ok((row, col))
    }

    /// Удаляет буфер с возможностью принудительного удаления
    pub fn delete_buffer(buf: &Buffer) -> OxiResult<()> {
        let buf_del_opts = BufDeleteOpts::builder().force(true).build();
        handle_error(buf.clone().delete(&buf_del_opts), "Failed to delete buffer")?;
        Ok(())
    }
}
