pub mod button;
pub mod footer;
pub mod header;

pub trait DashboardElement {
    fn render(&self) -> String;
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

    pub fn render(&self) -> Vec<String> {
        self.elements.iter().map(|el| el.render()).collect()
    }
}
