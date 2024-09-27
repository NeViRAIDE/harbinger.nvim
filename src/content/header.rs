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

// Функции для создания заголовков
pub fn create_header() -> Box<dyn DashboardElement> {
    Box::new(Header::new("Welcome to Neovim!"))
}

pub fn create_subheader() -> Box<dyn DashboardElement> {
    Box::new(Header::new("==================="))
}
