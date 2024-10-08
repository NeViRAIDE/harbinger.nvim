use std::any::Any;

use super::DashboardElement;

pub struct EmptyLineElement;

impl DashboardElement for EmptyLineElement {
    fn render(&self) -> String {
        "\n".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn highlight_group(&self) -> &'static str {
        "HarbingerEmptyLine"
    }
}
