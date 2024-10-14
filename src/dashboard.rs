use std::{cell::RefCell, rc::Rc};

use nvim_oxi::Result as OxiResult;

use crate::{
    config::{
        layout::{Column, Row},
        Config,
    },
    widget::WidgetManager,
};

pub struct Dashboard {
    pub config: Config,
    widget_manager: WidgetManager,
}

impl Dashboard {
    pub fn new(config: Config) -> OxiResult<Rc<RefCell<Self>>> {
        let w_man = WidgetManager::new()?;

        Ok(Rc::new(RefCell::new(Dashboard {
            config,
            widget_manager: w_man,
        })))
    }

    pub fn toggle_dashboard(&mut self) -> OxiResult<()> {
        if self.config.open_on_start {
            self.close_dashboard()?;
        } else {
            self.open_dashboard()?;
        }
        self.config.open_on_start = !self.config.open_on_start;
        Ok(())
    }

    fn open_dashboard(&mut self) -> OxiResult<()> {
        // nvim_oxi::print!("{:?}", self.config);
        self.widget_manager.render_dashboard(&self.config)?;
        // let count = self.count_widgets();
        // nvim_oxi::print!("Widgets: {:?}", count);
        Ok(())
    }

    fn close_dashboard(&mut self) -> OxiResult<()> {
        self.widget_manager.close_all_widgets()?;
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
        self.widget_manager.update_widgets()
    }
}
