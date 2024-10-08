use nvim_oxi::{conversion::FromObject, Dictionary};

use self::keymaps::Keymaps;
use self::layout::Layout;

mod content;
mod defaults;
mod keymaps;
mod layout;
mod highlights;

#[derive(Debug, Default)]
pub struct Config {
    pub open_on_start: bool,
    pub layout: Layout,
    pub borders: String,
    pub keymaps: Keymaps,
}

impl Config {
    pub fn from_dict(options: Dictionary) -> Self {
        Self {
            open_on_start: parse_bool_option(
                &options,
                "open_on_start",
                defaults::DEFAULT_AUTO_OPEN,
            ),
            layout: Layout::from_dict(&options),
            borders: parse_string_option(&options, "borders", defaults::DEFAULT_BORDERS),
            keymaps: Keymaps::from_dict(&options),
        }
    }
}

fn parse_string_option(options: &Dictionary, key: &str, default: &str) -> String {
    options
        .get(key)
        .and_then(|obj| String::from_object(obj.clone()).ok())
        .unwrap_or_else(|| default.to_string())
}

fn parse_bool_option(options: &Dictionary, key: &str, default: bool) -> bool {
    options
        .get(key)
        .and_then(|obj| bool::from_object(obj.clone()).ok())
        .unwrap_or(default)
}
