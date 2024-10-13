use std::{cell::RefCell, rc::Rc};

use crate::config::{content::Content, highlights::Highlights};

use super::{Column, Row};

pub struct ColumnBuilder {
    width: String,
    height: String,
    content: Option<Content>,
    rows: Vec<Rc<RefCell<Row>>>,
    columns: Vec<Rc<RefCell<Column>>>,
    active: bool,
    highlights: Option<Highlights>,
    priority: Option<usize>,
    hide_on_resize: Option<bool>,
}

impl ColumnBuilder {
    pub fn new() -> Self {
        Self {
            width: "100%".to_string(),
            height: "100%".to_string(),
            content: None,
            rows: vec![],
            columns: vec![],
            active: false,
            highlights: None,
            priority: Some(1),
            hide_on_resize: Some(false),
        }
    }

    pub fn width(mut self, width: String) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: String) -> Self {
        self.height = height;
        self
    }

    pub fn content(mut self, content: Option<Content>) -> Self {
        self.content = content;
        self
    }

    pub fn rows(mut self, rows: Vec<Rc<RefCell<Row>>>) -> Self {
        self.rows = rows;
        self
    }

    pub fn columns(mut self, columns: Vec<Rc<RefCell<Column>>>) -> Self {
        self.columns = columns;
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn highlights(mut self, highlights: Option<Highlights>) -> Self {
        self.highlights = highlights;
        self
    }

    pub fn priority(mut self, priority: usize) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn hide_on_resize(mut self, hide_on_resize: bool) -> Self {
        self.hide_on_resize = Some(hide_on_resize);
        self
    }

    pub fn build(self) -> Column {
        Column {
            width: self.width,
            height: self.height,
            content: self.content,
            rows: self.rows,
            columns: self.columns,
            active: self.active,
            highlights: self.highlights,
            priority: self.priority,
            hide_on_resize: self.hide_on_resize,
        }
    }
}
