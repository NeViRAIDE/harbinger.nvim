use super::DashboardElement;

pub struct Header {
    text: String,
}

impl Header {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

impl DashboardElement for Header {
    fn render(&self) -> String {
        format!("{}", self.text)
    }
}

pub fn create_header(content: &str) -> Box<dyn DashboardElement> {
    Box::new(Header::new(content))
}

pub fn create_subheader(content: &str) -> Box<dyn DashboardElement> {
    Box::new(Header::new(content))
}
