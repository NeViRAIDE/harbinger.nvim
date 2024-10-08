use nvim_oxi::{conversion::FromObject, Dictionary};

use super::defaults;

#[derive(Debug, Default)]
pub struct Keymaps {
    pub toggle_dashboard: String,
    pub navigate_sections: String,
    pub navigate_buttons: String,
    pub execute_button: String,
}

impl Keymaps {
    pub fn from_dict(options: &Dictionary) -> Self {
        Keymaps {
            toggle_dashboard: parse_string_option(
                options,
                "toggle_dashboard",
                defaults::DEFAULT_TOGGLE_DASHBOARD,
            ),
            navigate_sections: parse_string_option(
                options,
                "navigate_sections",
                defaults::DEFAULT_NAVIGATE_SECTIONS,
            ),
            navigate_buttons: parse_string_option(
                options,
                "navigate_buttons",
                defaults::DEFAULT_NAVIGATE_BUTTONS,
            ),
            execute_button: parse_string_option(
                options,
                "execute_button",
                defaults::DEFAULT_EXECUTE_BUTTON,
            ),
        }
    }
}

fn parse_string_option(options: &Dictionary, key: &str, default: &str) -> String {
    options
        .get(key)
        .and_then(|obj| String::from_object(obj.clone()).ok())
        .unwrap_or_else(|| default.to_string())
}
