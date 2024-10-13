use std::{cell::RefCell, rc::Rc};

use nvim_oxi::{
    api::{
        opts::OptionOpts,
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
    window: Window,
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

    pub fn add_widget_with_config(
        &mut self,
        config: &WidgetConfig,
        width: f64,
        height: f64,
        row: f64,
        col: f64,
    ) -> OxiResult<()> {
        let buffer = self.buffer_manager.get_buffer();

        let content = config.content.as_deref().unwrap_or("");
        let lines: Vec<String> = content.lines().map(String::from).collect();
        buffer.borrow_mut().set_lines(0.., true, lines)?;

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

        let window = nvim_oxi::api::open_win(&buffer.borrow(), true, &float_config)?;

        let window_opts = OptionOpts::builder().win(window.clone()).build();

        set_option_value("cursorline", true, &window_opts)?;
        set_option_value("wrap", false, &window_opts)?;

        self.widgets.push(Widget {
            window,
            config: config.clone(),
        });

        Ok(())
    }

    pub fn render_dashboard(&mut self, config: &Config) -> OxiResult<()> {
        let (term_width, term_height) = get_terminal_size()?;

        fn traverse_layout(
            layout: &DashboardLayout,
            manager: &mut WidgetManager,
            term_width: usize,
            term_height: usize,
        ) -> OxiResult<()> {
            for row_rc in &layout.rows {
                traverse_row(row_rc, manager, term_width, term_height)?;
            }
            Ok(())
        }

        fn traverse_row(
            row: &Rc<RefCell<Row>>,
            manager: &mut WidgetManager,
            term_width: usize,
            term_height: usize,
        ) -> OxiResult<()> {
            let row = row.borrow();

            if let Some(content) = &row.content {
                let widget_config = WidgetConfig::new(
                    "Row Widget".to_string(),
                    row.height.clone(),
                    row.width.clone(),
                    "0%".to_string(),
                    "0%".to_string(),
                )
                .with_content(Some(content.get_text()))
                .with_priority(row.priority.unwrap_or(1))
                .with_hide_on_resize(row.hide_on_resize.unwrap_or(false));

                let widget_width = percent_to_absolute(&widget_config.width, term_width);
                let widget_height = percent_to_absolute(&widget_config.height, term_height);
                let widget_row = percent_to_absolute(&widget_config.row, term_height);
                let widget_col = percent_to_absolute(&widget_config.col, term_width);

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

            for column_rc in &row.columns {
                traverse_column(column_rc, manager, term_width, term_height)?;
            }
            Ok(())
        }

        fn traverse_column(
            column: &Rc<RefCell<Column>>,
            manager: &mut WidgetManager,
            term_width: usize,
            term_height: usize,
        ) -> OxiResult<()> {
            let column = column.borrow();

            if let Some(content) = &column.content {
                let widget_config = WidgetConfig::new(
                    "Column Widget".to_string(),
                    column.height.clone(),
                    column.width.clone(),
                    "0%".to_string(),
                    "0%".to_string(),
                )
                .with_content(Some(content.get_text()))
                .with_priority(column.priority.unwrap_or(1))
                .with_hide_on_resize(column.hide_on_resize.unwrap_or(false));

                let widget_width = percent_to_absolute(&widget_config.width, term_width);
                let widget_height = percent_to_absolute(&widget_config.height, term_height);
                let widget_row = percent_to_absolute(&widget_config.row, term_height);
                let widget_col = percent_to_absolute(&widget_config.col, term_width);

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

            for nested_row_rc in &column.rows {
                traverse_row(nested_row_rc, manager, term_width, term_height)?;
            }

            for nested_column_rc in &column.columns {
                traverse_column(nested_column_rc, manager, term_width, term_height)?;
            }
            Ok(())
        }

        traverse_layout(&config.layout, self, term_width, term_height)
    }

    pub fn navigate_to_next_widget(&self) {
        if let Some(current_widget) = self.widgets.first() {}
    }

    pub fn navigate_to_previous_widget(&self) {
        // Логика переключения на предыдущее окно
    }

    pub fn update_widgets(&mut self, config: &Config) -> OxiResult<()> {
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
                    &widget.window.clone().close(false)?;
                    continue;
                }
            }

            widget.window.set_config(
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
        }

        Ok(())
    }
}

fn get_terminal_size() -> OxiResult<(usize, usize)> {
    let opts = OptionOpts::builder().build();
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
