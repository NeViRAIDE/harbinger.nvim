use nvim_oxi::{Dictionary, Result as OxiResult};

use crate::setup::Config;

#[derive(Debug)]
pub struct App {
    config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        App { config }
    }

    pub fn setup(&mut self, dict: Dictionary) -> OxiResult<()> {
        let config = Config::from_dict(dict);
        self.config = config;
        Ok(())
    }
}
