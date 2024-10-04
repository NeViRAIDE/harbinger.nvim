use std::any::Any;

use super::{DashboardElement, ElementAlignment};

pub struct EmptyLineElement;

impl DashboardElement for EmptyLineElement {
    fn render(&self) -> String {
        "\n".to_string()
    }

    fn alignment(&self) -> ElementAlignment {
        ElementAlignment::Left
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
