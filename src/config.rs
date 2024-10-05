use nvim_oxi::{conversion::FromObject, Dictionary, Object};

use crate::defaults::{
    DEFAULT_AUTO_OPEN, DEFAULT_BUTTONS_ITEMS, DEFAULT_BUTTONS_POSITION, DEFAULT_FOOTER_POSITION,
    DEFAULT_FOOTER_TEXT, DEFAULT_HEADER_POSITION, DEFAULT_HEADER_TEXT, DEFAULT_KEYMAP,
    DEFAULT_SUB_HEADER_POSITION, DEFAULT_SUB_HEADER_TEXT,
};

#[derive(Debug, Default)]
pub struct Config {
    pub open_on_start: bool,
    pub keymap: String,
    pub header: TextPosition,
    pub sub_header: TextPosition,
    pub footer: TextPosition,
    pub buttons: ButtonsConfig,
}

#[derive(Debug, Default)]
pub struct TextPosition {
    pub text: String,
    pub position: String,
}

#[derive(Debug, Default)]
pub struct ButtonsConfig {
    pub items: Vec<(String, String, String)>,
    pub position: String,
}

impl Config {
    pub fn from_dict(options: Dictionary) -> Self {
        Self {
            open_on_start: parse_bool_option(&options, "open_on_start", DEFAULT_AUTO_OPEN),
            keymap: parse_string_option(&options, "keymap", DEFAULT_KEYMAP),
            header: parse_text_position_option(
                &options,
                "header",
                DEFAULT_HEADER_TEXT,
                DEFAULT_HEADER_POSITION,
            ),
            sub_header: parse_text_position_option(
                &options,
                "sub_header",
                DEFAULT_SUB_HEADER_TEXT,
                DEFAULT_SUB_HEADER_POSITION,
            ),
            footer: parse_text_position_option(
                &options,
                "footer",
                DEFAULT_FOOTER_TEXT,
                DEFAULT_FOOTER_POSITION,
            ),
            buttons: parse_buttons_config(&options),
        }
    }
}

fn parse_string_option(options: &Dictionary, key: &str, default: &str) -> String {
    options
        .get(key)
        .and_then(|obj| String::from_object(obj.clone()).ok())
        .unwrap_or_else(|| default.to_string())
}

fn parse_text_position_option(
    options: &Dictionary,
    key: &str,
    default_text: &str,
    default_position: &str,
) -> TextPosition {
    if let Some(obj) = options.get(key) {
        if let Ok(dict) = Dictionary::from_object(obj.clone()) {
            let text = parse_text_in_dict(&dict, "text", default_text);
            let position = parse_string_in_dict(&dict, "position", default_position);
            TextPosition { text, position }
        } else {
            TextPosition {
                text: default_text.to_string(),
                position: default_position.to_string(),
            }
        }
    } else {
        TextPosition {
            text: default_text.to_string(),
            position: default_position.to_string(),
        }
    }
}

fn parse_string_in_dict(dict: &Dictionary, key: &str, default: &str) -> String {
    dict.get(key)
        .and_then(|obj| String::from_object(obj.clone()).ok())
        .unwrap_or_else(|| default.to_string())
}

fn parse_text_in_dict(dict: &Dictionary, key: &str, default: &str) -> String {
    if let Some(obj) = dict.get(key) {
        // Try to parse as a string
        if let Ok(string_value) = String::from_object(obj.clone()) {
            string_value
        }
        // Try to parse as an array of strings
        else if let Ok(array) = Vec::<Object>::from_object(obj.clone()) {
            array
                .into_iter()
                .filter_map(|item| String::from_object(item).ok())
                .collect::<Vec<String>>()
                .join("\n")
        }
        // Fallback to default
        else {
            default.to_string()
        }
    } else {
        default.to_string()
    }
}

fn parse_buttons_config(options: &Dictionary) -> ButtonsConfig {
    if let Some(obj) = options.get("buttons") {
        if let Ok(dict) = Dictionary::from_object(obj.clone()) {
            let items = parse_buttons_items(&dict);
            let position = parse_string_in_dict(&dict, "position", DEFAULT_BUTTONS_POSITION);
            ButtonsConfig { items, position }
        } else {
            ButtonsConfig {
                items: DEFAULT_BUTTONS_ITEMS
                    .iter()
                    .map(|(t, i, c)| (t.to_string(), i.to_string(), c.to_string()))
                    .collect(),
                position: DEFAULT_BUTTONS_POSITION.to_string(),
            }
        }
    } else {
        ButtonsConfig {
            items: DEFAULT_BUTTONS_ITEMS
                .iter()
                .map(|(t, i, c)| (t.to_string(), i.to_string(), c.to_string()))
                .collect(),
            position: DEFAULT_BUTTONS_POSITION.to_string(),
        }
    }
}

fn parse_buttons_items(dict: &Dictionary) -> Vec<(String, String, String)> {
    if let Some(obj) = dict.get("items") {
        if let Ok(buttons_array) = Vec::<Object>::from_object(obj.clone()) {
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
                .collect()
        } else {
            DEFAULT_BUTTONS_ITEMS
                .iter()
                .map(|(t, i, c)| (t.to_string(), i.to_string(), c.to_string()))
                .collect()
        }
    } else {
        DEFAULT_BUTTONS_ITEMS
            .iter()
            .map(|(t, i, c)| (t.to_string(), i.to_string(), c.to_string()))
            .collect()
    }
}

fn parse_bool_option(options: &Dictionary, key: &str, default: bool) -> bool {
    options
        .get(key)
        .and_then(|obj| bool::from_object(obj.clone()).ok())
        .unwrap_or(default)
}
