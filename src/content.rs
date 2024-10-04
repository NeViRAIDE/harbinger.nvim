use std::any::Any;

use button::ButtonGroup;

pub mod button;
pub mod footer;
pub mod header;
pub mod text_element;

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

    pub fn render(&self) -> (Vec<String>, usize, usize) {
        let mut lines = Vec::new();
        let mut button_count = 0;
        let mut first_button_line_index = None;
        let mut current_line_index = 0;

        for element in self.elements.iter() {
            let rendered = element.render();
            let rendered_lines: Vec<&str> = rendered.split('\n').collect();

            for line in &rendered_lines {
                lines.push(line.to_string());
                current_line_index += 1;
            }

            if let Some(button_group) = element.as_any().downcast_ref::<ButtonGroup>() {
                button_count += button_group.buttons.len();
                if first_button_line_index.is_none() {
                    first_button_line_index = Some(current_line_index);
                }
            }
        }

        (lines, button_count, first_button_line_index.unwrap_or(0))
    }
}
