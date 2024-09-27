use super::DashboardElement;

pub struct Footer {
    text: String,
}

impl Footer {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

impl DashboardElement for Footer {
    fn render(&self) -> String {
        format!("\n{}", self.text)
    }
}

pub fn create_footer(content: &str) -> Box<dyn DashboardElement> {
    Box::new(Footer::new(content))
}
