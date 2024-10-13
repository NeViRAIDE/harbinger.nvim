// widget/config.rs
#[derive(Clone, Debug)]
pub struct WidgetConfig {
    pub title: String,
    pub width: String,                // Например, "50%"
    pub height: String,               // Например, "30%"
    pub row: String,                  // Например, "10%"
    pub col: String,                  // Например, "20%"
    pub content: Option<String>,      // Контент виджета
    pub priority: Option<usize>,      // Приоритет для скрытия
    pub hide_on_resize: Option<bool>, // Флаг для скрытия при изменении размера
}

impl WidgetConfig {
    pub fn new(title: String, height: String, width: String, row: String, col: String) -> Self {
        Self {
            title,
            height,
            width,
            row,
            col,
            content: None,
            priority: None,
            hide_on_resize: None,
        }
    }

    pub fn with_content(mut self, content: Option<String>) -> Self {
        self.content = content;
        self
    }

    pub fn with_priority(mut self, priority: usize) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn with_hide_on_resize(mut self, hide_on_resize: bool) -> Self {
        self.hide_on_resize = Some(hide_on_resize);
        self
    }
}
