use nvim_oxi::{conversion::FromObject, Dictionary, Object};

use crate::defaults::{
    DEFAULT_BUTTONS, DEFAULT_BUTTONS_POS, DEFAULT_FOOTER, DEFAULT_FOOTER_POS, DEFAULT_HEADER,
    DEFAULT_HEADER_POS, DEFAULT_KEYMAP, DEFAULT_SUB_HEADER, DEFAULT_SUB_HEADER_POS,
};

#[derive(Debug, Default)]
pub struct Config {
    pub keymap: String,
    pub header: String,
    pub header_pos: String,
    pub sub_header: String,
    pub sub_header_pos: String,
    pub footer: String,
    pub footer_pos: String,
    pub buttons: Vec<(String, String, String)>,
    pub buttons_pos: String,
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
                .and_then(Self::parse_string_or_array)
                .unwrap_or_else(|| DEFAULT_HEADER.to_string()),
            header_pos: options
                .get("header_pos")
                .and_then(|header_pos_obj| String::from_object(header_pos_obj.clone()).ok())
                .unwrap_or_else(|| DEFAULT_HEADER_POS.to_string()),

            sub_header: options
                .get("sub_header")
                .and_then(Self::parse_string_or_array)
                .unwrap_or_else(|| DEFAULT_SUB_HEADER.to_string()),
            sub_header_pos: options
                .get("sub_header_pos")
                .and_then(|sub_header_pos_obj| String::from_object(sub_header_pos_obj.clone()).ok())
                .unwrap_or_else(|| DEFAULT_SUB_HEADER_POS.to_string()),

            footer: options
                .get("footer")
                .and_then(Self::parse_string_or_array)
                .unwrap_or_else(|| DEFAULT_FOOTER.to_string()),
            footer_pos: options
                .get("footer_pos")
                .and_then(|footer_pos_obj| String::from_object(footer_pos_obj.clone()).ok())
                .unwrap_or_else(|| DEFAULT_FOOTER_POS.to_string()),

            buttons: options
                .get("buttons")
                .and_then(Self::parse_buttons)
                .unwrap_or_else(|| {
                    DEFAULT_BUTTONS
                        .iter()
                        .map(|(title, icon, command)| {
                            (title.to_string(), icon.to_string(), command.to_string())
                        })
                        .collect()
                }),
            buttons_pos: options
                .get("buttons_pos")
                .and_then(|buttons_pos_obj| String::from_object(buttons_pos_obj.clone()).ok())
                .unwrap_or_else(|| DEFAULT_BUTTONS_POS.to_string()),
        }
    }

    fn parse_string_or_array(obj: &Object) -> Option<String> {
        if let Ok(string_value) = String::from_object(obj.clone()) {
            Some(string_value)
        } else if let Ok(array) = Vec::<Object>::from_object(obj.clone()) {
            let joined = array
                .into_iter()
                .filter_map(|item| String::from_object(item).ok())
                .collect::<Vec<String>>()
                .join("\n");
            Some(joined)
        } else {
            None
        }
    }

    fn parse_buttons(obj: &Object) -> Option<Vec<(String, String, String)>> {
        if let Ok(buttons_array) = Vec::<Object>::from_object(obj.clone()) {
            Some(
                buttons_array
                    .into_iter()
                    .filter_map(|button_obj| {
                        if let Ok(button) = Vec::<Object>::from_object(button_obj) {
                            if button.len() == 3 {
                                let title = String::from_object(button[0].clone()).ok()?;
                                let icon = String::from_object(button[1].clone()).ok()?;
                                let command = String::from_object(button[2].clone()).ok()?;
                                return Some((title, icon, command));
                            }
                        }
                        None
                    })
                    .collect(),
            )
        } else {
            None
        }
    }
}
