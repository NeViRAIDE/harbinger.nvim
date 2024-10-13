use std::{cell::RefCell, rc::Rc};

use nvim_oxi::{api::err_writeln, conversion::FromObject, Dictionary, Object};

use super::{
    content::Content,
    highlights::{parse_highlights, Highlights},
    parse_bool_option, parse_usize_option,
};

use column::ColumnBuilder;

mod column;

#[derive(Debug, Default)]
pub struct DashboardLayout {
    pub rows: Vec<Rc<RefCell<Row>>>,
}

#[derive(Debug, Default)]
pub struct Row {
    pub columns: Vec<Rc<RefCell<Column>>>,
    pub content: Option<Content>,
    pub width: String,
    pub height: String,
    pub priority: Option<usize>,
    pub hide_on_resize: Option<bool>,
}

#[derive(Debug, Default)]
pub struct Column {
    pub width: String,
    pub height: String,
    pub content: Option<Content>,
    pub rows: Vec<Rc<RefCell<Row>>>,
    pub columns: Vec<Rc<RefCell<Column>>>,
    pub active: bool,
    pub highlights: Option<Highlights>,
    pub priority: Option<usize>,
    pub hide_on_resize: Option<bool>,
}

impl DashboardLayout {
    pub fn from_dict(options: &Dictionary) -> Self {
        if let Some(obj) = options.get("layout") {
            if let Ok(dict) = Dictionary::from_object(obj.clone()) {
                DashboardLayout {
                    rows: parse_rows(&dict, "rows"),
                }
            } else {
                DashboardLayout { rows: vec![] }
            }
        } else {
            DashboardLayout { rows: vec![] }
        }
    }
}

fn parse_layout_items<T>(
    dict: &Dictionary,
    key: &str,
    parse_fn: fn(&Dictionary) -> T,
) -> Vec<Rc<RefCell<T>>> {
    if let Some(obj) = dict.get(key) {
        if let Ok(items_array) = Vec::<Object>::from_object(obj.clone()) {
            return items_array
                .into_iter()
                .filter_map(|item_obj| {
                    if let Ok(item_dict) = Dictionary::from_object(item_obj) {
                        Some(Rc::new(RefCell::new(parse_fn(&item_dict))))
                    } else {
                        err_writeln("Failed to parse item_dict");
                        None
                    }
                })
                .collect();
        } else {
            err_writeln(&format!("Failed to parse {}_array", key));
        }
    }
    vec![]
}

fn parse_string_option(options: &Dictionary, key: &str, default: String) -> String {
    options
        .get(key)
        .and_then(|obj| String::from_object(obj.clone()).ok())
        .unwrap_or(default)
}

fn parse_columns(dict: &Dictionary, key: &str) -> Vec<Rc<RefCell<Column>>> {
    parse_layout_items(dict, key, |column_dict| {
        ColumnBuilder::new()
            .width(parse_string_option(
                column_dict,
                "width",
                "100%".to_string(),
            ))
            .height(parse_string_option(
                column_dict,
                "height",
                "100%".to_string(),
            ))
            .content(Content::from_dict(column_dict).ok())
            .rows(parse_nested_rows(column_dict))
            .columns(parse_nested_columns(column_dict))
            .active(parse_bool_option(column_dict, "active", false))
            .highlights(parse_highlights(column_dict))
            .priority(parse_usize_option(column_dict, "priority", 1))
            .hide_on_resize(parse_bool_option(column_dict, "hide_on_resize", false))
            .build()
    })
}

fn parse_rows(dict: &Dictionary, key: &str) -> Vec<Rc<RefCell<Row>>> {
    parse_layout_items(dict, key, |row_dict| Row {
        content: Content::from_dict(row_dict).ok(),
        columns: parse_columns(row_dict, "columns"),
        width: parse_string_option(row_dict, "width", "100%".to_string()),
        height: parse_string_option(row_dict, "height", "100%".to_string()),
        priority: Some(parse_usize_option(row_dict, "priority", 1)),
        hide_on_resize: Some(parse_bool_option(row_dict, "hide_on_resize", false)),
    })
}

fn parse_nested_rows(dict: &Dictionary) -> Vec<Rc<RefCell<Row>>> {
    if let Some(obj) = dict.get("rows") {
        if let Ok(rows_array) = Vec::<Object>::from_object(obj.clone()) {
            rows_array
                .into_iter()
                .filter_map(|row_obj| {
                    if let Ok(row_dict) = Dictionary::from_object(row_obj) {
                        let content = Content::from_dict(&row_dict).ok();
                        let columns = parse_columns(&row_dict, "columns");

                        if content.is_some() || !columns.is_empty() {
                            return Some(Rc::new(RefCell::new(Row {
                                content,
                                columns,
                                width: parse_string_option(&row_dict, "width", "100%".to_string()),
                                height: parse_string_option(
                                    &row_dict,
                                    "height",
                                    "100%".to_string(),
                                ),
                                priority: Some(parse_usize_option(&row_dict, "priority", 1)),
                                hide_on_resize: Some(parse_bool_option(
                                    &row_dict,
                                    "hide_on_resize",
                                    false,
                                )),
                            })));
                        }
                    } else {
                        err_writeln("Failed to parse row_dict");
                    }
                    None
                })
                .collect()
        } else {
            err_writeln("Failed to parse rows_array");
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
                        let content = Content::from_dict(&column_dict).ok();
                        let rows = parse_nested_rows(&column_dict);
                        let columns = parse_nested_columns(&column_dict);

                        if content.is_some() || !rows.is_empty() || !columns.is_empty() {
                            return Some(Rc::new(RefCell::new(
                                ColumnBuilder::new()
                                    .width(parse_string_option(
                                        &column_dict,
                                        "width",
                                        "100%".to_string(),
                                    ))
                                    .height(parse_string_option(
                                        &column_dict,
                                        "height",
                                        "100%".to_string(),
                                    ))
                                    .content(content)
                                    .rows(rows)
                                    .columns(columns)
                                    .active(parse_bool_option(&column_dict, "active", false))
                                    .highlights(parse_highlights(&column_dict))
                                    .priority(parse_usize_option(&column_dict, "priority", 1))
                                    .hide_on_resize(parse_bool_option(
                                        &column_dict,
                                        "hide_on_resize",
                                        false,
                                    ))
                                    .build(),
                            )));
                        }
                    } else {
                        err_writeln("Failed to parse column_dict");
                    }
                    None
                })
                .collect()
        } else {
            err_writeln("Failed to parse columns_array");
            vec![]
        }
    } else {
        vec![]
    }
}
