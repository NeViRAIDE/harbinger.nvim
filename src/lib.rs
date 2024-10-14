use nvim_oxi::{Dictionary, Function, Result as OxiResult};

use config::Config;
use dashboard::Dashboard;
use setup::DashboardExt;

mod buffer;
mod config;
mod dashboard;
mod error;
mod setup;
mod utils;
mod widget;

#[nvim_oxi::plugin]
fn harbinger() -> OxiResult<Dictionary> {
    let exports: Dictionary =
        Dictionary::from_iter::<[(&str, Function<Dictionary, OxiResult<()>>); 1]>([(
            "setup",
            Function::from_fn({
                move |dict: Dictionary| -> OxiResult<()> {
                    let config = Config::from_dict(dict);
                    let dashboard_instance = Dashboard::new(config)?;
                    dashboard_instance.setup()?;
                    Ok(())
                }
            }),
        )]);

    Ok(exports)
}
