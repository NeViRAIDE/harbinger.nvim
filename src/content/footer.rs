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
        format!("{}", self.text)
    }
}

// Функция для создания футера
pub fn create_footer() -> Box<dyn DashboardElement> {
    Box::new(Footer::new("Neovim 0.5.1"))
}
