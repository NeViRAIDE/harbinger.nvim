use super::DashboardElement;

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

impl DashboardElement for Button {
    fn render(&self) -> String {
        format!(" {:<width$} {}", self.title, self.icon, width = 20)
    }
}

// Функция для создания набора кнопок и возврата их в виде вектора элементов
pub fn create_buttons() -> Vec<Box<dyn DashboardElement>> {
    vec![
        Box::new(Button::new("Create new file", "", "new_file_command")),
        Box::new(Button::new("Find file", "", "Telescope find_files")),
        Box::new(Button::new("Recent files", "", "Telescope oldfiles")),
    ]
}
