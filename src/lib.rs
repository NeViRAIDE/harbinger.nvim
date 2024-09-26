use std::sync::{Arc, Mutex};

use nvim_oxi::{
    api::{create_user_command, err_writeln, opts::CreateCommandOpts, types::*},
    Dictionary, Error as OxiError, Function, Result as OxiResult,
};

use error::PluginError;
use setup::Config;

use self::core::App;

mod core;
mod error;
mod setup;

#[nvim_oxi::plugin]
fn harbinger() -> OxiResult<Dictionary> {
    let config = Config::default();

    let app = Arc::new(Mutex::new(App::new(config)));

    let opts = CreateCommandOpts::builder()
        .bang(true)
        .desc("shows a greetings message")
        .nargs(CommandNArgs::ZeroOrOne)
        .build();

    let greetings = |args: CommandArgs| {
        let who = args.args.unwrap_or("from Rust".to_owned());
        let bang = if args.bang { "!" } else { "" };
        print!("Hello {}{}", who, bang);
    };

    create_user_command("Harbinger", greetings, &opts)?;

    let app_setup = Arc::clone(&app);
    let exports: Dictionary =
        Dictionary::from_iter::<[(&str, Function<Dictionary, Result<(), OxiError>>); 1]>([(
            "setup",
            Function::from_fn(move |dict: Dictionary| -> OxiResult<()> {
                match app_setup.lock() {
                    Ok(mut app) => app.setup(dict),
                    Err(e) => {
                        err_writeln(&format!(
                            "Failed to acquire lock on app during setup: {}",
                            e
                        ));
                        Err(PluginError::Custom("Lock error during setup".into()).into())
                    }
                }
            }),
        )]);

    Ok(exports)
}
