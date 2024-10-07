use super::{text::TextElement, DashboardElement, ElementAlignment};

pub fn create_footer(content: &str, alignment: &str) -> Box<dyn DashboardElement> {
    let alignment_enum = match alignment {
        "left" => ElementAlignment::Left,
        "right" => ElementAlignment::Right,
        _ => ElementAlignment::Center,
    };

    Box::new(TextElement::new(content, alignment_enum, "HarbingerFooter"))
}
