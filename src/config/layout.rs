use nvim_oxi::{conversion::FromObject, Dictionary, Object};
use std::cell::RefCell;
use std::rc::Rc;

use super::content::Content;
use super::highlights::{parse_highlights, Highlights};
use super::parse_bool_option;

#[derive(Debug, Default)]
pub struct Layout {
    pub rows: Vec<Rc<RefCell<Row>>>,
}

#[derive(Debug, Default)]
pub struct Row {
    pub columns: Vec<Rc<RefCell<Column>>>,
}

#[derive(Debug, Default)]
pub struct Column {
    pub size: usize,
    pub content: Option<Content>,
    pub rows: Vec<Rc<RefCell<Row>>>,       // Вложенные строки
    pub columns: Vec<Rc<RefCell<Column>>>, // Вложенные колонки
    pub active: bool,
    pub highlights: Option<Highlights>,
}

impl Layout {
    pub fn from_dict(options: &Dictionary) -> Self {
        if let Some(obj) = options.get("layout") {
            if let Ok(dict) = Dictionary::from_object(obj.clone()) {
                Layout {
                    rows: parse_rows(&dict, "rows"),
                }
            } else {
                Layout { rows: vec![] }
            }
        } else {
            Layout { rows: vec![] }
        }
    }
}

fn parse_rows(dict: &Dictionary, key: &str) -> Vec<Rc<RefCell<Row>>> {
    if let Some(obj) = dict.get(key) {
        if let Ok(rows_array) = Vec::<Object>::from_object(obj.clone()) {
            rows_array
                .into_iter()
                .filter_map(|row_obj| {
                    if let Ok(row_dict) = Dictionary::from_object(row_obj) {
                        Some(Rc::new(RefCell::new(Row {
                            columns: parse_columns(&row_dict, "columns"),
                        })))
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

fn parse_columns(dict: &Dictionary, key: &str) -> Vec<Rc<RefCell<Column>>> {
    if let Some(obj) = dict.get(key) {
        if let Ok(columns_array) = Vec::<Object>::from_object(obj.clone()) {
            columns_array
                .into_iter()
                .filter_map(|column_obj| {
                    if let Ok(column_dict) = Dictionary::from_object(column_obj) {
                        Some(Rc::new(RefCell::new(Column {
                            size: parse_usize_option(&column_dict, "size", 100),
                            content: Content::from_dict(&column_dict).ok(), // Преобразование Result в Option
                            rows: parse_nested_rows(&column_dict),
                            columns: parse_nested_columns(&column_dict),
                            active: parse_bool_option(&column_dict, "active", false),
                            highlights: parse_highlights(&column_dict),
                        })))
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

fn parse_usize_option(options: &Dictionary, key: &str, default: usize) -> usize {
    options
        .get(key)
        .and_then(|obj| usize::from_object(obj.clone()).ok())
        .unwrap_or(default)
}

fn parse_nested_rows(dict: &Dictionary) -> Vec<Rc<RefCell<Row>>> {
    if let Some(obj) = dict.get("rows") {
        if let Ok(rows_array) = Vec::<Object>::from_object(obj.clone()) {
            rows_array
                .into_iter()
                .filter_map(|row_obj| {
                    if let Ok(row_dict) = Dictionary::from_object(row_obj) {
                        Some(Rc::new(RefCell::new(Row {
                            columns: parse_columns(&row_dict, "columns"),
                        })))
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

fn parse_nested_columns(dict: &Dictionary) -> Vec<Rc<RefCell<Column>>> {
    if let Some(obj) = dict.get("columns") {
        if let Ok(columns_array) = Vec::<Object>::from_object(obj.clone()) {
            columns_array
                .into_iter()
                .filter_map(|column_obj| {
                    if let Ok(column_dict) = Dictionary::from_object(column_obj) {
                        Some(Rc::new(RefCell::new(Column {
                            size: parse_usize_option(&column_dict, "size", 100),
                            content: Content::from_dict(&column_dict).ok(), // Преобразование Result в Option
                            rows: parse_nested_rows(&column_dict),
                            columns: parse_nested_columns(&column_dict),
                            active: parse_bool_option(&column_dict, "active", false),
                            highlights: parse_highlights(&column_dict),
                        })))
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
