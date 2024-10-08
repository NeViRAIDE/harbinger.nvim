use nvim_oxi::Object;
use nvim_oxi::{conversion::FromObject, Dictionary};
use std::cell::RefCell;
use std::rc::Rc;

use crate::error::{handle_error, PluginError};

use super::highlights::{parse_highlights, Highlights};
use super::parse_string_option;

#[derive(Debug)]
pub enum Content {
    Text(Rc<RefCell<TextContent>>),
    Buttons(Rc<RefCell<ButtonsContent>>),
}

impl Default for Content {
    fn default() -> Self {
        Content::Text(Rc::new(RefCell::new(TextContent::default())))
    }
}

#[derive(Debug, Default)]
pub struct TextContent {
    pub value: Vec<String>,
    pub alignment: Alignment,
}

#[derive(Debug, Default)]
pub struct ButtonsContent {
    pub items: Vec<Button>,
}

#[derive(Debug, Default)]
pub struct Button {
    pub label: String,
    pub command: Command,
    pub icon: Option<String>,
    pub highlights: Highlights,
}

#[derive(Debug)]
pub enum Command {
    VimCommand(String),
    LuaFunction(String),
}

impl Default for Command {
    fn default() -> Self {
        Command::VimCommand(String::new()) // Значение по умолчанию для Command
    }
}

#[derive(Debug, Default)]
pub struct Alignment {
    pub horizontal: HorizontalAlignment,
    pub vertical: VerticalAlignment,
}

#[derive(Debug, Default)]
pub enum HorizontalAlignment {
    Left,
    #[default]
    Center,
    Right,
}

#[derive(Debug, Default)]
pub enum VerticalAlignment {
    Top,
    #[default]
    Middle,
    Bottom,
}

impl Content {
    pub fn from_dict(dict: &Dictionary) -> Result<Self, PluginError> {
        let content_obj = handle_error(
            dict.get("content")
                .ok_or_else(|| PluginError::Custom("'content' field is missing".to_string())),
            "Failed to get 'content' from dictionary",
        )?;

        let content_dict = handle_error(
            Dictionary::from_object(content_obj.clone()),
            "Failed to convert 'content' to Dictionary",
        )?;

        let content_type_obj = handle_error(
            content_dict
                .get("type")
                .ok_or_else(|| PluginError::Custom("'type' field is missing".to_string())),
            "Failed to get 'type' field from content dictionary",
        )?;

        let content_type_str = handle_error(
            String::from_object(content_type_obj.clone()),
            "Failed to convert 'type' to string",
        )?;

        match content_type_str.as_str() {
            "text" => Ok(Content::Text(Rc::new(RefCell::new(TextContent {
                value: parse_text_value(&content_dict),
                alignment: parse_alignment(&content_dict),
            })))),
            "buttons" => Ok(Content::Buttons(Rc::new(RefCell::new(ButtonsContent {
                items: parse_buttons_items(&content_dict),
            })))),
            _ => Err(PluginError::Custom(format!(
                "Unknown content type: {}",
                content_type_str
            ))),
        }
    }
}

fn parse_text_value(dict: &Dictionary) -> Vec<String> {
    dict.get("value")
        .and_then(|obj| Vec::<Object>::from_object(obj.clone()).ok())
        .map(|values| {
            values
                .into_iter()
                .filter_map(|val| String::from_object(val).ok())
                .collect()
        })
        .unwrap_or_else(|| vec!["Default Text".to_string()])
}

fn parse_alignment(dict: &Dictionary) -> Alignment {
    let horizontal = dict
        .get("alignment")
        .and_then(|obj| Dictionary::from_object(obj.clone()).ok())
        .and_then(|alignment_dict| {
            alignment_dict
                .get("horizontal")
                .and_then(|obj| String::from_object(obj.clone()).ok())
        })
        .unwrap_or_else(|| "center".to_string());

    let vertical = dict
        .get("alignment")
        .and_then(|obj| Dictionary::from_object(obj.clone()).ok())
        .and_then(|alignment_dict| {
            alignment_dict
                .get("vertical")
                .and_then(|obj| String::from_object(obj.clone()).ok())
        })
        .unwrap_or_else(|| "middle".to_string());

    Alignment {
        horizontal: match horizontal.as_str() {
            "left" => HorizontalAlignment::Left,
            "right" => HorizontalAlignment::Right,
            _ => HorizontalAlignment::Center,
        },
        vertical: match vertical.as_str() {
            "top" => VerticalAlignment::Top,
            "bottom" => VerticalAlignment::Bottom,
            _ => VerticalAlignment::Middle,
        },
    }
}

fn parse_buttons_items(dict: &Dictionary) -> Vec<Button> {
    if let Some(obj) = dict.get("items") {
        if let Ok(items_array) = Vec::<Object>::from_object(obj.clone()) {
            items_array
                .into_iter()
                .filter_map(|item_obj| {
                    if let Ok(item_dict) = Dictionary::from_object(item_obj) {
                        Some(Button {
                            label: parse_string_option(&item_dict, "label", ""),
                            command: parse_command(&item_dict),
                            icon: item_dict
                                .get("icon")
                                .and_then(|obj| String::from_object(obj.clone()).ok()),
                            highlights: parse_highlights(&item_dict).unwrap_or(Highlights {
                                fg: "#FFFFFF".to_string(),
                                bg: "#000000".to_string(),
                            }),
                        })
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            vec![]
        }
    } else {
        vec![]
    }
}

fn parse_command(dict: &Dictionary) -> Command {
    if let Some(command_obj) = dict.get("command") {
        if let Ok(command_str) = String::from_object(command_obj.clone()) {
            if command_str.starts_with(":") {
                Command::VimCommand(command_str)
            } else {
                Command::LuaFunction(command_str)
            }
        } else {
            Command::VimCommand("".to_string())
        }
    } else {
        Command::VimCommand("".to_string())
    }
}
