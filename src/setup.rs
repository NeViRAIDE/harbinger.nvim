use nvim_oxi::{conversion::FromObject, Dictionary};

#[derive(Debug, Default)]
pub struct Config {
    pub keymap: String,
}

impl Config {
    pub fn from_dict(options: Dictionary) -> Self {
        Config {
            keymap: options
                .get("keymap")
                .and_then(|keymap_obj| String::from_object(keymap_obj.clone()).ok())
                .unwrap_or_else(|| "<M-d>".to_string()),
        }
    }
}
