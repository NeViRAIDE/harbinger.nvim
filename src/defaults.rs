pub const DEFAULT_AUTO_OPEN: bool = true;

pub const DEFAULT_KEYMAP: &str = "<M-d>";

pub const DEFAULT_HEADER_TEXT: &str = "Welcome to Neovim!";
pub const DEFAULT_HEADER_POSITION: &str = "center";

pub const DEFAULT_SUB_HEADER_TEXT: &str = "==================";
pub const DEFAULT_SUB_HEADER_POSITION: &str = "center";

pub const DEFAULT_FOOTER_TEXT: &str = "created with Rust by RAprogramm";
pub const DEFAULT_FOOTER_POSITION: &str = "center";

pub const DEFAULT_BUTTONS_ITEMS: &[(&str, &str, &str)] = &[
    ("Create new file", "", "edit new_file.txt"),
    ("Find file", "", "Telescope find_files"),
    ("Recent files", "", "Telescope oldfiles"),
    ("Exit", "X", "qall"),
];

pub const DEFAULT_BUTTONS_POSITION: &str = "center";
