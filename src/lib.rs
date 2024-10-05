use std::{cell::RefCell, path::PathBuf, rc::Rc};

use nvim_oxi::{
    api::{
        create_autocmd, create_user_command, err_writeln, get_vvar, list_bufs,
        opts::{CreateAutocmdOpts, CreateCommandOpts},
        types::{CommandArgs, CommandNArgs},
        Buffer,
    },
    Dictionary, Function, Result as OxiResult,
};

use config::Config;
use core::Dashboard;

mod buffer;
mod config;
mod content;
mod core;
mod defaults;
mod error;
mod utils;

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
            err_writeln(&format!("Failed to toggle dashboard: {}", e));
        }
    };

    create_user_command("Harbinger", open_or_close_dashboard, &opts)?;

    let exports: Dictionary =
        Dictionary::from_iter::<[(&str, Function<Dictionary, OxiResult<()>>); 1]>([(
            "setup",
            Function::from_fn({
                let app_setup = Rc::clone(&app);
                move |dict: Dictionary| -> OxiResult<()> {
                    app_setup.borrow_mut().setup(dict)?;

                    if app_setup.borrow().config.open_on_start {
                        let app_handle = Rc::clone(&app_setup);

                        // Set up autocommand on UIEnter
                        let autocmd_opts = CreateAutocmdOpts::builder()
                            .callback(Function::from_fn(move |_| -> OxiResult<bool> {
                                if should_open_dashboard() {
                                    if let Err(e) = app_handle.borrow_mut().toggle_dashboard() {
                                        err_writeln(&format!("Failed to toggle dashboard: {}", e));
                                    }
                                }
                                Ok(true) // Remove the autocommand after it runs
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

fn should_open_dashboard() -> bool {
    // Check if there are command-line arguments
    let argc: i64 = get_vvar("argc").unwrap_or(0);
    if argc > 0 {
        return false;
    }

    // Get the list of buffers and collect into a Vec
    let buffers: Vec<Buffer> = list_bufs().collect();

    if buffers.len() == 1 {
        let buffer = &buffers[0];
        // Check if the buffer is unnamed (no file associated)
        if buffer.get_name().unwrap_or_default() == PathBuf::new() {
            return true;
        }
    }

    false
}
