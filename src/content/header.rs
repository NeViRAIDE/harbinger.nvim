use std::any::Any;

use crate::error::handle_error;
use crate::utils::get_window_size;

use super::{DashboardElement, ElementAlignment};

pub struct Header {
    text: String,
    alignment: ElementAlignment,
}

impl Header {
    pub fn new(text: &str, alignment: ElementAlignment) -> Self {
        Self {
            text: text.to_string(),
            alignment,
        }
    }
}

impl DashboardElement for Header {
    fn render(&self) -> String {
        let (win_width, _) =
            handle_error(get_window_size(), "Failed to get window size").unwrap_or((80, 0));

        let formatted_text = match self.alignment {
            ElementAlignment::Left => self.text.to_string(),
            ElementAlignment::Center => {
                let padding = (win_width.saturating_sub(self.text.len())) / 2;
                format!("{:padding$}{}", "", self.text, padding = padding)
            }
            ElementAlignment::Right => {
                let padding = win_width.saturating_sub(self.text.len());
                format!("{:padding$}{}", "", self.text, padding = padding)
            }
        };
        format!("{}\n", formatted_text)
    }

    fn alignment(&self) -> ElementAlignment {
        self.alignment.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn create_header(content: &str, alignment: &str) -> Box<dyn DashboardElement> {
    let alignment_enum = match alignment {
        "left" => ElementAlignment::Left,
        "right" => ElementAlignment::Right,
        _ => ElementAlignment::Center,
    };

    Box::new(Header::new(content, alignment_enum))
}

pub fn create_subheader(content: &str, alignment: &str) -> Box<dyn DashboardElement> {
    let alignment_enum = match alignment {
        "left" => ElementAlignment::Left,
        "right" => ElementAlignment::Right,
        _ => ElementAlignment::Center,
    };

    Box::new(Header::new(content, alignment_enum))
}
