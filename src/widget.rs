// widget_manager.rs

use std::{cell::RefCell, rc::Rc};

use nvim_oxi::{
    api::{
        clear_autocmds, err_writeln,
        opts::{ClearAutocmdsOpts, OptionOpts},
        set_option_value,
        types::{WindowBorder, WindowConfig, WindowRelativeTo, WindowTitle, WindowTitlePosition},
        Window,
    },
    Result as OxiResult,
};

use crate::config::Config;
use crate::{
    buffer::BufferManager,
    config::layout::{Column, DashboardLayout, Row},
};

use self::config::WidgetConfig;

mod config;

#[derive(Debug)]
pub struct WidgetManager {
    widgets: Vec<Widget>,
    buffer_manager: BufferManager,
}

#[derive(Debug)]
pub struct Widget {
    window: Option<Window>,
    config: WidgetConfig,
}

impl WidgetManager {
    pub fn new() -> OxiResult<Self> {
        let buffer_man = BufferManager::new()?;
        Ok(Self {
            widgets: vec![],
            buffer_manager: buffer_man,
        })
    }

    fn create_widget(
        manager: &mut WidgetManager,
        content: Option<String>,
        row: f64,
        col: f64,
        width: f64,
        height: f64,
        title: String,
        hide_on_resize: bool,
    ) -> OxiResult<()> {
        if let Some(content) = content {
            let widget_config = WidgetConfig::new(
                title,
                height.to_string(),
                width.to_string(),
                format!("{}%", (row / get_terminal_size()?.1 as f64) * 100.0),
                format!("{}%", (col / get_terminal_size()?.0 as f64) * 100.0),
            )
            .with_content(Some(content))
            .with_hide_on_resize(hide_on_resize);

            manager.add_widget_with_config(&widget_config, width, height, row, col)?;
        }
        Ok(())
    }

    pub fn add_widget_with_config(
        &mut self,
        config: &WidgetConfig,
        width: f64,
        height: f64,
        row: f64,
        col: f64,
    ) -> OxiResult<()> {
        // Создаём новый буфер для виджета
        let buffer = self.buffer_manager.create_buffer()?;

        // Устанавливаем содержимое буфера
        self.buffer_manager
            .set_content(&buffer, config.content.as_deref().unwrap_or(""))?;

        // Настройка параметров плавающего окна
        let float_config = WindowConfig::builder()
            .relative(WindowRelativeTo::Editor)
            .width(width as u32)
            .height(height as u32)
            .row(row as u32)
            .col(col as u32)
            .border(WindowBorder::Single)
            .title(WindowTitle::SimpleString(config.title.clone().into()))
            .title_pos(WindowTitlePosition::Center)
            .build();

        // Создаём окно для виджета
        let window = nvim_oxi::api::open_win(&buffer.borrow(), true, &float_config)?;

        let window_opts = OptionOpts::builder().win(window.clone()).build();

        // Настраиваем дополнительные опции окна
        set_option_value("cursorline", true, &window_opts)?;
        set_option_value("wrap", false, &window_opts)?;

        // Добавляем виджет в список
        self.widgets.push(Widget {
            window: Some(window),
            config: config.clone(),
        });

        Ok(())
    }

    pub fn render_dashboard(&mut self, config: &Config) -> OxiResult<()> {
        let (term_width, term_height) = get_terminal_size()?;

        // Начальные позиции
        let mut current_row = 0.0;
        let mut current_col = 0.0;

        fn traverse_layout(
            layout: &DashboardLayout,
            manager: &mut WidgetManager,
            term_width: usize,
            term_height: usize,
            current_row: &mut f64,
            current_col: &mut f64,
        ) -> OxiResult<()> {
            for row_rc in &layout.rows {
                traverse_row(
                    row_rc,
                    manager,
                    term_width,
                    term_height,
                    current_row,
                    current_col,
                    term_width as f64, // Передаем ширину всего терминала как родительскую
                )?;
            }
            Ok(())
        }

        fn traverse_row(
            row: &Rc<RefCell<Row>>,
            manager: &mut WidgetManager,
            term_width: usize,
            term_height: usize,
            current_row: &mut f64,
            current_col: &mut f64,
            parent_width: f64, // Добавлено: ширина родительского элемента
        ) -> OxiResult<()> {
            let row = row.borrow();

            // Рассчитываем высоту строки
            let row_height = percent_to_absolute(&row.height, term_height);
            let row_start = *current_row;

            // Если у строки есть content, создаём виджет
            if let Some(content) = &row.content {
                let widget_config = WidgetConfig::new(
                    "Row Widget".to_string(),
                    row.height.clone(),
                    row.width.clone(),
                    format!("{}%", (row_start / term_height as f64) * 100.0),
                    "0%".to_string(), // Начинаем с 0 колонки для строки
                )
                .with_content(Some(content.get_text()))
                .with_priority(row.priority.unwrap_or(1))
                .with_hide_on_resize(row.hide_on_resize.unwrap_or(false));

                let widget_width = percent_to_absolute(&widget_config.width, parent_width as usize); // Используем родительскую ширину
                let widget_height = row_height;
                let widget_row = row_start;
                let widget_col = 0.0; // Строки занимают всю ширину, начинаем с 0

                if !(widget_config.hide_on_resize.unwrap_or(false) && term_width < 80) {
                    manager.add_widget_with_config(
                        &widget_config,
                        widget_width,
                        widget_height,
                        widget_row,
                        widget_col,
                    )?;
                }
            }

            // Обход колонок в строке
            let mut col_acc = 0.0;
            for column_rc in &row.columns {
                traverse_column(
                    column_rc,
                    manager,
                    term_width,
                    term_height,
                    current_row,
                    &mut col_acc,
                    parent_width, // Передаем ширину родителя
                )?;
            }

            // Обновляем текущую строку
            *current_row += row_height;

            Ok(())
        }

        fn traverse_column(
            column: &Rc<RefCell<Column>>,
            manager: &mut WidgetManager,
            term_width: usize,
            term_height: usize,
            current_row: &mut f64,
            current_col: &mut f64,
            parent_width: f64, // Добавлено: ширина родительского элемента
        ) -> OxiResult<()> {
            let column = column.borrow();

            // Рассчитываем ширину колонки
            let column_width = percent_to_absolute(&column.width, parent_width as usize); // Используем родительскую ширину
            let column_start = *current_col;

            // Если у колонки есть content, создаём виджет
            if let Some(content) = &column.content {
                let widget_config = WidgetConfig::new(
                    "Column Widget".to_string(),
                    column.height.clone(),
                    column.width.clone(),
                    format!("{}%", (*current_row / term_height as f64) * 100.0),
                    format!("{}%", (column_start / parent_width) * 100.0), // Используем ширину родителя
                )
                .with_content(Some(content.get_text()))
                .with_priority(column.priority.unwrap_or(1))
                .with_hide_on_resize(column.hide_on_resize.unwrap_or(false));

                let widget_width = column_width;
                let widget_height = percent_to_absolute(&column.height, term_height);
                let widget_row = *current_row;
                let widget_col = column_start;

                if !(widget_config.hide_on_resize.unwrap_or(false) && term_width < 80) {
                    manager.add_widget_with_config(
                        &widget_config,
                        widget_width,
                        widget_height,
                        widget_row,
                        widget_col,
                    )?;
                }
            }

            // Обход вложенных строк в колонке
            for nested_row_rc in &column.rows {
                traverse_row(
                    nested_row_rc,
                    manager,
                    term_width,
                    term_height,
                    current_row,
                    &mut 0.0,     // Для вложенных строк начальная колонка 0
                    column_width, // Передаем ширину колонки
                )?;
            }

            // Обход вложенных колонок
            for nested_column_rc in &column.columns {
                traverse_column(
                    nested_column_rc,
                    manager,
                    term_width,
                    term_height,
                    current_row,
                    &mut 0.0,     // Для вложенных колонок начальная колонка 0
                    parent_width, // Передаем ширину родителя
                )?;
            }

            // Обновляем текущую колонку
            *current_col += column_width;

            Ok(())
        }

        traverse_layout(
            &config.layout,
            self,
            term_width,
            term_height,
            &mut current_row,
            &mut current_col,
        )
    }

    pub fn navigate_to_next_widget(&self) {
        if let Some(current_widget) = self.widgets.first() {
            // Логика переключения на следующее окно
            // Например, фокусировка на следующее окно
        }
    }

    pub fn navigate_to_previous_widget(&self) {
        // Логика переключения на предыдущее окно
    }

    pub fn update_widgets(&mut self) -> OxiResult<()> {
        let (term_width, term_height) = get_terminal_size()?;

        self.widgets.sort_by_key(|w| w.config.priority.unwrap_or(1));

        for widget in &mut self.widgets {
            let widget_config = &widget.config;

            let widget_width = percent_to_absolute(&widget_config.width, term_width) as u32;
            let widget_height = percent_to_absolute(&widget_config.height, term_height) as u32;
            let widget_row = percent_to_absolute(&widget_config.row, term_height) as u32;
            let widget_col = percent_to_absolute(&widget_config.col, term_width) as u32;

            if let Some(hide_on_resize) = widget_config.hide_on_resize {
                if hide_on_resize && (term_width < 80 || term_height < 24) {
                    if let Some(win) = widget.window.take() {
                        win.close(false)?;
                    }
                    continue;
                }
            }

            if let Some(win) = widget.window.as_mut() {
                win.set_config(
                    &WindowConfig::builder()
                        .relative(WindowRelativeTo::Editor)
                        .width(widget_width)
                        .height(widget_height)
                        .row(widget_row)
                        .col(widget_col)
                        .border(WindowBorder::Single)
                        .title(WindowTitle::SimpleString(
                            widget_config.title.clone().into(),
                        ))
                        .title_pos(WindowTitlePosition::Center)
                        .build(),
                )?;
            } else {
                err_writeln("Window is already closed");
            }
        }

        Ok(())
    }

    pub fn close_all_widgets(&mut self) -> OxiResult<()> {
        for widget in &mut self.widgets {
            // Проверяем, если окно уже закрыто (None), продолжаем цикл, избегаем паники
            if widget.window.is_none() {
                err_writeln("Window is already closed");
                continue;
            }

            if let Some(win) = widget.window.take() {
                let buf = win.get_buf()?; // Получаем буфер из окна

                let clear_opts = ClearAutocmdsOpts::builder().buffer(buf.clone()).build();
                clear_autocmds(&clear_opts)?;

                // Закрываем окно безопасно, без panics
                win.close(false)?;
            }
        }
        self.widgets.clear(); // Очищаем список виджетов после закрытия всех окон
        Ok(())
    }
}

fn get_terminal_size() -> OxiResult<(usize, usize)> {
    let opts = nvim_oxi::api::opts::OptionOpts::builder().build();
    let width = nvim_oxi::api::get_option_value::<usize>("columns", &opts)?;
    let height = nvim_oxi::api::get_option_value::<usize>("lines", &opts)?;
    Ok((width, height))
}

fn percent_to_absolute(value: &str, total: usize) -> f64 {
    if let Some(stripped) = value.strip_suffix('%') {
        if let Ok(percent) = stripped.parse::<f64>() {
            return (percent / 100.0) * (total as f64);
        }
    }
    if let Ok(abs_val) = value.parse::<f64>() {
        return abs_val;
    }
    0.0
}
