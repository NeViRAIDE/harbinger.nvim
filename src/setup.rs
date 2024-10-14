use std::{cell::RefCell, rc::Rc};

use nvim_oxi::{
    api::{opts::SetKeymapOpts, set_keymap, types::Mode},
    Function, Result as OxiResult,
};

use crate::dashboard::Dashboard;

pub trait DashboardExt {
    fn setup(&self) -> OxiResult<()>;
}

impl DashboardExt for Rc<RefCell<Dashboard>> {
    fn setup(&self) -> OxiResult<()> {
        let dashboard_handle = Rc::clone(self);

        set_keymap(
            Mode::Normal,
            &dashboard_handle
                .clone()
                .borrow()
                .config
                .keymaps
                .toggle_dashboard,
            "",
            &SetKeymapOpts::builder()
                .callback(Function::from_fn(move |_| -> OxiResult<()> {
                    dashboard_handle.borrow_mut().toggle_dashboard()
                }))
                .build(),
        )?;

        if self.borrow().config.open_on_start {
            self.borrow_mut().toggle_dashboard()?;
        }

        Ok(())
    }
}
