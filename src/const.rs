pub static IGNORE_FILE: &'static str = include_str!("ignores.txt");

pub const RED: &str = "\x1b[33m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[36m";
pub const RESET: &str = "\x1b[39m";

pub const ADD: &str = "--add";
pub const ADD_S: &str = "-a";

pub const FORCE_ADD: &str = "--force-add";
pub const FORCE_ADD_S: &str = "-fa";

pub const REMOVE: &str = "--remove";
pub const REMOVE_S: &str = "-r";

pub const ADD_ITEM: &str = "--add-item";
pub const ADD_ITEM_S: &str = "-ai";

pub const REMOVE_ITEM: &str = "--remove-item";
pub const REMOVE_ITEM_S: &str = "-ri";

pub const CREATE: &str = "--create";
pub const CREATE_S: &str = "-c";

pub const HELP: &str = "--help";
pub const HELP_S: &str = "-h";

pub const VERSION: &str = "--version";
pub const VERSION_S: &str = "-v";

pub const LIST: &str = "--list";
pub const LIST_S: &str = "-l";

pub const LIST_EXHAUSTIVE: &str = "--list-exhaustive";
pub const LIST_EXHAUSTIVE_S: &str = "-le";

pub const CLEAR: &str = "--clear";
pub const CLEAR_S: &str = "-cl";

#[cfg(feature = "test")]
pub const GIT_IGNORE: &str = "test.gitignore";

#[cfg(not(feature = "test"))]
pub const GIT_IGNORE: &str = ".gitignore";

pub const PROGRAM_NAME: &str = "Git Ignore Builder";
pub const PROGRAM_VERSION: &str = "0.1.0";

pub const TITLE_WRAPPER_OPEN: &str = "#==========================================[";
pub const TITLE_WRAPPER_CLOSE: &str = "]==========================================#";
