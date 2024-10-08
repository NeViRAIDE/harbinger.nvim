use std::{cell::RefCell, rc::Rc};

use nvim_oxi::{
    api::{create_autocmd, err_writeln, opts::CreateAutocmdOpts},
    Dictionary, Function, Result as OxiResult,
};

use config::Config;
use utils::should_open_dashboard;

mod config;
mod error;
mod utils;

#[nvim_oxi::plugin]
fn harbinger() -> OxiResult<Dictionary> {
    let app = Rc::new(Dashboard::new(Config::default()));

    let exports: Dictionary =
        Dictionary::from_iter::<[(&str, Function<Dictionary, OxiResult<()>>); 1]>([(
            "setup",
            Function::from_fn({
                let app_setup = Rc::clone(&app);
                move |dict: Dictionary| -> OxiResult<()> {
                    app_setup.borrow_mut().setup(dict, Rc::clone(&app_setup))?;

                    if app_setup.borrow().config.open_on_start {
                        let app_handle = Rc::clone(&app_setup);

                        let autocmd_opts = CreateAutocmdOpts::builder()
                            .callback(Function::from_fn(move |_| -> OxiResult<bool> {
                                if should_open_dashboard() {
                                    if let Err(e) = app_handle.borrow_mut().toggle_dashboard() {
                                        err_writeln(&format!("Failed to toggle dashboard: {}", e));
                                    }
                                }
                                Ok(true)
                            }))
                            .build();

                        create_autocmd(["UIEnter"], &autocmd_opts)?;
                    }

                    Ok(())
                }
            }),
        )]);

    Ok(exports)
}
