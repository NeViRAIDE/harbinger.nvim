use nvim_oxi::{conversion::FromObject, Dictionary};

use super::parse_string_option;

#[derive(Debug, Default)]
pub struct Highlights {
    pub fg: String,
    pub bg: String,
}

pub fn parse_highlights(dict: &Dictionary) -> Option<Highlights> {
    dict.get("highlights")
        .and_then(|obj| Dictionary::from_object(obj.clone()).ok())
        .map(|highlights_dict| Highlights {
            fg: parse_string_option(&highlights_dict, "fg", "#FFFFFF"),
            bg: parse_string_option(&highlights_dict, "bg", "#000000"),
        })
}
