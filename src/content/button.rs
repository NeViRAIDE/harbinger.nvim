use std::any::Any;

use unicode_width::UnicodeWidthStr;

use super::{DashboardElement, ElementAlignment};
use crate::utils::get_window_size;

pub struct Button {
    pub title: String,
    pub icon: String,
    pub command: String,
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

        // Find the maximum lengths of titles and icons, accounting for Unicode widths
        let max_title_length = self
            .buttons
            .iter()
            .map(|button| UnicodeWidthStr::width(button.title.as_str()))
            .max()
            .unwrap_or(0);

        let max_icon_length = self
            .buttons
            .iter()
            .map(|button| UnicodeWidthStr::width(button.icon.as_str()))
            .max()
            .unwrap_or(0);

        // Define the spacing between title and icon
        let spacing = 5;

        // Calculate the maximum total length of a button line
        let max_total_length = 1 + 1 + max_title_length + spacing + max_icon_length;

        let mut rendered_buttons = String::new();

        for button in &self.buttons {
            // Format the title and icon with fixed widths
            let button_text = format!(
                " {:<title_width$}{spacing}{:<icon_width$}",
                button.title,
                button.icon,
                title_width = max_title_length,
                icon_width = max_icon_length,
                spacing = " ".repeat(spacing)
            );

            // Apply alignment to the entire button text
            let aligned_button = match self.alignment {
                ElementAlignment::Left => button_text.clone(),
                ElementAlignment::Center => {
                    let total_padding = win_width.saturating_sub(max_total_length);
                    let left_padding = total_padding / 2;
                    format!("{:space$}{}", "", button_text, space = left_padding)
                }
                ElementAlignment::Right => {
                    let total_padding = win_width.saturating_sub(max_total_length);
                    format!("{:space$}{}", "", button_text, space = total_padding)
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
