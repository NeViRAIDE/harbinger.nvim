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
    pub buttons: Vec<Button>,
    alignment: ElementAlignment,
}

impl ButtonGroup {
    pub fn new(buttons: Vec<Button>, alignment: ElementAlignment) -> Self {
        Self { buttons, alignment }
    }
}

impl DashboardElement for ButtonGroup {
    fn render(&self) -> String {
        let (win_width, _) = get_window_size().unwrap_or((80, 0));

        // Находим максимальную длину текста и максимальную длину иконок
        let max_title_length = self
            .buttons
            .iter()
            .map(|button| button.title.len())
            .max()
            .unwrap_or(20);

        let max_icon_length = self
            .buttons
            .iter()
            .map(|button| button.icon.len())
            .max()
            .unwrap_or(1);

        // Считаем максимальную длину строки (текст + иконка + отступ)
        let max_total_length = max_title_length + 5 + max_icon_length; // 5 пробелов между текстом и иконкой

        let mut rendered_buttons = String::new();

        for button in &self.buttons {
            // Рассчитываем отступ для текста
            let title_padding = max_title_length.saturating_sub(button.title.len());

            // Формируем строку с фиксированными отступами для текста и иконки
            let button_text = format!(
                " {}{:title_padding$}     {}",
                button.title,
                "",
                button.icon,
                title_padding = title_padding
            );

            // Применяем выравнивание кнопок по центру
            let aligned_button = match self.alignment {
                ElementAlignment::Left => button_text,
                ElementAlignment::Center => {
                    let space_padding = (win_width.saturating_sub(max_total_length)) / 2;
                    format!(
                        "{:space_padding$}{}",
                        "",
                        button_text,
                        space_padding = space_padding
                    )
                }
                ElementAlignment::Right => {
                    let space_padding = win_width.saturating_sub(max_total_length);
                    format!(
                        "{:space_padding$}{}",
                        "",
                        button_text,
                        space_padding = space_padding
                    )
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

pub fn create_buttons(
    content: &[(String, String, String)],
    alignment: &str,
) -> Vec<Box<dyn DashboardElement>> {
    let alignment_enum = match alignment {
        "left" => ElementAlignment::Left,
        "right" => ElementAlignment::Right,
        _ => ElementAlignment::Center,
    };

    let buttons: Vec<Button> = content
        .iter()
        .map(|(title, icon, command)| Button::new(title, icon, command))
        .collect();

    vec![Box::new(ButtonGroup::new(buttons, alignment_enum))]
}
