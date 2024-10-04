pub const DEFAULT_KEYMAP: &str = "<M-d>";

pub const DEFAULT_HEADER: &str = "Welcome to Neovim!";
pub const DEFAULT_HEADER_POS: &str = "center";

pub const DEFAULT_SUB_HEADER: &str = "==================";
pub const DEFAULT_SUB_HEADER_POS: &str = "center";

pub const DEFAULT_FOOTER: &str = "created with Rust by RAprogramm";
pub const DEFAULT_FOOTER_POS: &str = "center";

pub const DEFAULT_BUTTONS: &[(&str, &str, &str)] = &[
    ("Create new file", "", "new_file_command"),
    ("Find file", "", "Telescope find_files"),
    ("Recent files", "", "Telescope oldfiles"),
    ("Exit", "X", "qall"),
    ("test1", "Xyi", "qall"),
];
pub const DEFAULT_BUTTONS_POS: &str = "center";
