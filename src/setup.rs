use nvim_oxi::{conversion::FromObject, Dictionary};

use crate::defaults::{DEFAULT_FOOTER, DEFAULT_HEADER, DEFAULT_KEYMAP, DEFAULT_SUB_HEADER};

#[derive(Debug, Default)]
pub struct Config {
    pub keymap: String,
    pub header: String,
    pub sub_header: String,
    pub footer: String,
}

impl Config {
    pub fn from_dict(options: Dictionary) -> Self {
        Config {
            keymap: options
                .get("keymap")
                .and_then(|keymap_obj| String::from_object(keymap_obj.clone()).ok())
                .unwrap_or_else(|| DEFAULT_KEYMAP.to_string()),
            header: options
                .get("header")
                .and_then(|header_obj| String::from_object(header_obj.clone()).ok())
                .unwrap_or_else(|| DEFAULT_HEADER.to_string()),
            sub_header: options
                .get("sub_header")
                .and_then(|sub_header_obj| String::from_object(sub_header_obj.clone()).ok())
                .unwrap_or_else(|| DEFAULT_SUB_HEADER.to_string()),
            footer: options
                .get("footer")
                .and_then(|footer_obj| String::from_object(footer_obj.clone()).ok())
                .unwrap_or_else(|| DEFAULT_FOOTER.to_string()),
        }
    }
}
