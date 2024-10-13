use std::{cell::RefCell, rc::Rc};

use nvim_oxi::{
    api::{create_autocmd, err_writeln, opts::CreateAutocmdOpts},
    Function, Result as OxiResult,
};

use crate::{
    config::{
        layout::{Column, Row},
        Config,
    },
    utils::should_open_dashboard,
    widget::WidgetManager,
};

#[derive(Debug)]
pub struct Dashboard {
    config: Config,
    widget_manager: WidgetManager,
}

impl Dashboard {
    pub fn new(config: Config) -> OxiResult<Self> {
        let w_man = WidgetManager::new()?;

        Ok(Dashboard {
            config,
            widget_manager: w_man,
        })
    }

    pub fn setup(self: Rc<Self>) -> OxiResult<()> {
        // if self.config.open_on_start {
        //     let dashboard_handle = Rc::clone(&self);
        //
        //     let autocmd_opts = CreateAutocmdOpts::builder()
        //         .callback(Function::from_fn(move |_| -> OxiResult<bool> {
        //             if should_open_dashboard() {
        //                 if let Err(e) = dashboard_handle.open_dashboard() {
        //                     err_writeln(&format!("Failed to open dashboard: {}", e));
        //                 }
        //             }
        //             Ok(true)
        //         }))
        //         .build();
        //
        //     create_autocmd(["UIEnter"], &autocmd_opts)?;
        // }

        let dashboard_handle = Rc::clone(&self);
        nvim_oxi::api::set_keymap(
            nvim_oxi::api::types::Mode::Normal,
            &dashboard_handle.config.keymaps.toggle_dashboard,
            "",
            &nvim_oxi::api::opts::SetKeymapOpts::builder()
                .callback(Function::from_fn(move |_| -> OxiResult<()> {
                    self.toggle_dashboard()
                }))
                .build(),
        )?;

        Ok(())
    }

    // fn open_on_start(&self) {
    //
    // }

    fn toggle_dashboard(&self) -> OxiResult<()> {
        self.open_dashboard();
        // i tyt budet esli otrkit to zakrit' a esli ne otrkit to oitkrit'

        Ok(())
    }

    fn open_dashboard(&self) -> Result<(), String> {
        nvim_oxi::print!("{:?}", self.config);
        // Печать текущего конфига
        // nvim_oxi::print!("{:?}", self.config);

        // Подсчет количества строк
        let count = self.count_widgets();

        nvim_oxi::print!("Widgets: {:?}", count);

        Ok(())
    }

    pub fn count_widgets(&self) -> usize {
        let mut total_widgets = 0;

        fn count_widgets_in_cols(column: &Rc<RefCell<Column>>, widgets_acc: &mut usize) {
            if column.borrow().content.is_some() {
                *widgets_acc += 1;
            }

            for row in &column.borrow().rows {
                count_columns_and_widgets_in_row(row, widgets_acc);
            }

            for sub_column in &column.borrow().columns {
                count_widgets_in_cols(sub_column, widgets_acc);
            }
        }

        fn count_columns_and_widgets_in_row(row: &Rc<RefCell<Row>>, widgets_acc: &mut usize) {
            if row.borrow().content.is_some() {
                *widgets_acc += 1;
            }

            for column in &row.borrow().columns {
                count_widgets_in_cols(column, widgets_acc);
            }
        }

        for row in &self.config.layout.rows {
            count_columns_and_widgets_in_row(row, &mut total_widgets);
        }

        total_widgets
    }

    fn handle_resize(&mut self) -> OxiResult<()> {
        // Пересчитать размеры и позиции всех виджетов
        self.widget_manager.update_widgets(&self.config)
    }
}
