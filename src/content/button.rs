use std::any::Any;

use super::{DashboardElement, ElementAlignment};
use crate::utils::get_window_size;

pub struct Button {
    title: String,
    icon: String,
    command: String,
}

impl Button {
    pub fn new(title: &str, icon: &str, command: &str) -> Self {
        Self {
            title: title.to_string(),
            icon: icon.to_string(),
            command: format!("<cmd>{}<cr>", command),
        }
    }
}

pub struct ButtonGroup {
    buttons: Vec<Button>,
    alignment: ElementAlignment,
}

impl ButtonGroup {
    pub fn new(buttons: Vec<Button>, alignment: ElementAlignment) -> Self {
        Self { buttons, alignment }
    }
}

impl DashboardElement for ButtonGroup {
    fn render(&self) -> String {
        let (win_width, _) = get_window_size().unwrap_or((80, 0)); // Получаем размеры окна или используем стандартное значение

        let mut rendered_buttons = String::new();

        for button in &self.buttons {
            let button_text = format!(" {:<width$} {}", button.title, button.icon, width = 20);
            let aligned_button = match self.alignment {
                ElementAlignment::Left => button_text, // Слева
                ElementAlignment::Center => {
                    let padding = (win_width.saturating_sub(button_text.len())) / 2;
                    format!(
                        "{:width$}",
                        button_text,
                        width = padding + button_text.len()
                    ) // Центр
                }
                ElementAlignment::Right => {
                    let padding = win_width.saturating_sub(button_text.len());
                    format!(
                        "{:width$}",
                        button_text,
                        width = padding + button_text.len()
                    ) // Справа
                }
            };
            rendered_buttons.push_str(&aligned_button);
            rendered_buttons.push('\n');
        }

        rendered_buttons
    }

    fn alignment(&self) -> ElementAlignment {
        self.alignment.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn create_buttons() -> Vec<Box<dyn DashboardElement>> {
    let buttons = vec![
        Button::new("Create new file", "", "new_file_command"),
        Button::new("Find file", "", "Telescope find_files"),
        Button::new("Recent files", "", "Telescope oldfiles"),
    ];

    // Группируем кнопки с общим выравниванием по центру
    vec![Box::new(ButtonGroup::new(
        buttons,
        ElementAlignment::Center,
    ))]
}
