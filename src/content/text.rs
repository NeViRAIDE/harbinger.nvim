use std::any::Any;

use crate::{error::handle_error, utils::get_window_size};

use super::{DashboardElement, ElementAlignment};

pub struct TextElement {
    text: String,
    alignment: ElementAlignment,
    highlight: &'static str,
}

impl TextElement {
    pub fn new(text: &str, alignment: ElementAlignment, highlight: &'static str) -> Self {
        Self {
            text: text.to_string(),
            alignment,
            highlight,
        }
    }
}

impl DashboardElement for TextElement {
    fn render(&self) -> String {
        let (win_width, _) =
            handle_error(get_window_size(), "Failed to get window size").unwrap_or((80, 0));

        let formatted_text = match self.alignment {
            ElementAlignment::Left => self.text.clone(),
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

    fn highlight_group(&self) -> &'static str {
        self.highlight
    }
}
