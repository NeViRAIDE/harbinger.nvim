use std::{cell::RefCell, rc::Rc};

use nvim_oxi::{
    api::{
        create_user_command, err_writeln,
        opts::CreateCommandOpts,
        types::{CommandArgs, CommandNArgs},
    },
    Dictionary, Function, Result as OxiResult,
};

use core::Dashboard;
use setup::Config;

mod buffer;
mod content;
mod core;
mod defaults;
mod error;
mod setup;

#[nvim_oxi::plugin]
fn harbinger() -> OxiResult<Dictionary> {
    let config = Config::default();

    let app = Rc::new(RefCell::new(Dashboard::new(config)));

    let opts = CreateCommandOpts::builder()
        .bang(true)
        .desc("Opens or closes the dashboard")
        .nargs(CommandNArgs::Zero)
        .build();

    let app_handle = Rc::clone(&app);
    let open_or_close_dashboard = move |_: CommandArgs| {
        if let Err(e) = app_handle.borrow_mut().toggle_dashboard() {
            err_writeln(&format!("Error toggling dashboard: {}", e));
        }
    };

    create_user_command("Harbinger", open_or_close_dashboard, &opts)?;

    let app_setup = Rc::clone(&app);
    let exports: Dictionary =
        Dictionary::from_iter::<[(&str, Function<Dictionary, OxiResult<()>>); 1]>([(
            "setup",
            Function::from_fn(move |dict: Dictionary| -> OxiResult<()> {
                app_setup.borrow_mut().setup(dict)?;
                Ok(())
            }),
        )]);

    Ok(exports)
}
