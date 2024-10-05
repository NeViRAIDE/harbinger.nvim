use std::{any::Any, collections::HashMap};

use button::ButtonGroup;

pub mod button;
pub mod empty_line;
pub mod footer;
pub mod header;
pub mod text;

pub trait DashboardElement {
    fn render(&self) -> String;
    fn alignment(&self) -> ElementAlignment;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone)]
pub enum ElementAlignment {
    Left,
    Center,
    Right,
}

pub struct Content {
    elements: Vec<Box<dyn DashboardElement>>,
}

impl Content {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, element: Box<dyn DashboardElement>) {
        self.elements.push(element);
    }

    pub fn render(&self) -> (Vec<String>, usize, usize, HashMap<usize, String>) {
        let mut lines = Vec::new();
        let mut button_count = 0;
        let mut first_button_line = None;
        let mut current_line_number = 1; // Start from 1
        let mut command_mapping = HashMap::new();

        for element in self.elements.iter() {
            let rendered = element.render();
            let rendered_lines: Vec<&str> = rendered.lines().collect();

            if let Some(button_group) = element.as_any().downcast_ref::<ButtonGroup>() {
                if first_button_line.is_none() {
                    first_button_line = Some(current_line_number);
                }
                button_count += button_group.buttons.len();

                for (i, line) in rendered_lines.iter().enumerate() {
                    lines.push(line.to_string());
                    // Map the line number to the command
                    command_mapping
                        .insert(current_line_number, button_group.buttons[i].command.clone());
                    current_line_number += 1;
                }
            } else {
                for line in rendered_lines {
                    lines.push(line.to_string());
                    current_line_number += 1;
                }
            }
        }

        (
            lines,
            button_count,
            first_button_line.unwrap_or(1),
            command_mapping,
        )
    }
}
