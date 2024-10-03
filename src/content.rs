use std::any::Any;

use button::Button;

pub mod button;
pub mod footer;
pub mod header;

pub trait DashboardElement {
    fn render(&self) -> String;
    fn as_any(&self) -> &dyn Any;
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

            if element.as_any().is::<Button>() {
                button_count += 1;
                if first_button_line_index.is_none() {
                    first_button_line_index = Some(current_line_index);
                }
            }
        }

        (lines, button_count, first_button_line_index.unwrap_or(0))
    }
}
