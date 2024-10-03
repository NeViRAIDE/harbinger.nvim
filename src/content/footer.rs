use std::any::Any;

use super::{DashboardElement, ElementAlignment};
use crate::error::handle_error;
use crate::utils::get_window_size;

pub struct Footer {
    text: String,
    alignment: ElementAlignment,
}

impl Footer {
    pub fn new(text: &str, alignment: ElementAlignment) -> Self {
        Self {
            text: text.to_string(),
            alignment,
        }
    }
}

impl DashboardElement for Footer {
    fn render(&self) -> String {
        // Используем handle_error для обработки ошибки получения размеров окна
        let (win_width, _) =
            handle_error(get_window_size(), "Failed to get window size").unwrap_or((80, 0)); // В случае ошибки используем ширину по умолчанию 80

        // Выравнивание текста в зависимости от заданного положения
        let formatted_text = match self.alignment {
            ElementAlignment::Left => self.text.to_string(), // Слева без изменений
            ElementAlignment::Center => {
                let padding = (win_width.saturating_sub(self.text.len())) / 2;
                format!("{:width$}", self.text, width = padding + self.text.len())
                // Центрирование
            }
            ElementAlignment::Right => {
                let padding = win_width.saturating_sub(self.text.len());
                format!("{:width$}", self.text, width = padding + self.text.len())
                // Справа
            }
        };

        format!("\n{}", formatted_text) // Возвращаем результат с переносом строки
    }

    fn alignment(&self) -> ElementAlignment {
        self.alignment.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn create_footer(content: &str, alignment: &str) -> Box<dyn DashboardElement> {
    let alignment_enum = match alignment {
        "left" => ElementAlignment::Left,
        "right" => ElementAlignment::Right,
        _ => ElementAlignment::Center,
    };

    Box::new(Footer::new(content, alignment_enum))
}
