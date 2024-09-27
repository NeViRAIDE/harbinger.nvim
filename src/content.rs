mod button;
mod footer;
mod header;

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

// Генерация контента теперь происходит в других модулях.
pub fn generate_dashboard_content() -> Vec<String> {
    let mut dashboard = Content::new();

    // Заголовки и кнопки добавляются с помощью функций из соответствующих модулей
    dashboard.add_element(header::create_header());
    dashboard.add_element(header::create_subheader());

    for button in button::create_buttons() {
        dashboard.add_element(button);
    }

    dashboard.add_element(footer::create_footer());

    // Рендерим контент и возвращаем в виде вектора строк
    dashboard.render()
}
