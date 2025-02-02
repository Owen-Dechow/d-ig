use crate::{
    r#const::{
        ADD, ADD_ITEM, ADD_ITEM_S, ADD_S, CLEAR, CLEAR_S, CREATE, CREATE_S, FORCE_ADD, FORCE_ADD_S,
        HELP, HELP_S, LIST, LIST_EXHAUSTIVE, LIST_EXHAUSTIVE_S, LIST_S, REMOVE, REMOVE_ITEM,
        REMOVE_ITEM_S, REMOVE_S, VERSION, VERSION_S,
    },
    Error, PROGRAM_NAME, PROGRAM_VERSION,
};
use std::env;

enum ParserState {
    Add,
    AddF,
    AddItem,
    Remove,
    RemoveItem,
    List,
    None,
}

#[derive(Debug)]
pub enum Change {
    AddG(String),
    AddGF(String),
    AddI(String),
    RemoveG(String),
    RemoveI(String),
}

#[derive(Debug)]
pub struct Args {
    pub changes: Vec<Change>,
    pub create: bool,
    pub version: bool,
    pub help: bool,
    pub list: Option<Vec<String>>,
    pub list_exhaustive: bool,
    pub clear: bool,
}

impl Args {
    pub fn parse() -> Result<Args, Error> {
        let args: Vec<String> = env::args()
            .enumerate()
            .filter_map(|(idx, x)| if idx > 0 { Some(x) } else { None })
            .collect();

        let arg_count = args.len();

        let mut changes = Vec::new();
        let mut create = false;
        let mut version = false;
        let mut clear = false;
        let mut help = false;
        let mut state = ParserState::None;
        let mut list = None;
        let mut list_exhaustive = false;

        if arg_count == 0 {
            return Err(Error(format!("Must enter command --help, -h for help.",)));
        }

        for (idx, arg) in args.into_iter().enumerate() {
            match arg.as_str() {
                ADD | ADD_S => {
                    Args::set_state_or_list(&mut state, &mut list, ParserState::Add, arg);
                }
                ADD_ITEM | ADD_ITEM_S => {
                    Args::set_state_or_list(&mut state, &mut list, ParserState::AddItem, arg);
                }
                FORCE_ADD | FORCE_ADD_S => {
                    Args::set_state_or_list(&mut state, &mut list, ParserState::AddF, arg);
                }
                REMOVE | REMOVE_S => {
                    Args::set_state_or_list(&mut state, &mut list, ParserState::Remove, arg);
                }
                REMOVE_ITEM | REMOVE_ITEM_S => {
                    Args::set_state_or_list(&mut state, &mut list, ParserState::RemoveItem, arg);
                }
                CREATE | CREATE_S => {
                    if Args::set_state_or_list(&mut state, &mut list, ParserState::Add, arg) {
                        create = true;
                    }
                }
                VERSION | VERSION_S => {
                    if Args::set_state_or_list(&mut state, &mut list, ParserState::None, arg) {
                        state = ParserState::None;
                        version = true;

                        if arg_count > 1 {
                            return Err(Error(format!(
                                "{}, {} must be called without any other arguments.",
                                VERSION, VERSION_S
                            )));
                        }
                    }
                }
                CLEAR | CLEAR_S => {
                    if Args::set_state_or_list(&mut state, &mut list, ParserState::None, arg) {
                        state = ParserState::None;
                        clear = true;

                        if arg_count > 1 {
                            return Err(Error(format!(
                                "{}, {} must be called without any other arguments.",
                                CLEAR, CLEAR_S
                            )));
                        }
                    }
                }
                HELP | HELP_S => {
                    if Args::set_state_or_list(&mut state, &mut list, ParserState::None, arg) {
                        help = true;

                        if arg_count > 1 {
                            return Err(Error(format!(
                                "{}, {} must be called without any other arguments.",
                                HELP, HELP_S
                            )));
                        }
                    }
                }
                LIST | LIST_S => {
                    if Args::set_state_or_list(&mut state, &mut list, ParserState::List, arg) {
                        list = Some(Vec::new());

                        if idx > 0 {
                            return Err(Error(format!(
                                "{}, {} must be the first arguments",
                                LIST, LIST_S
                            )));
                        }
                    }
                }
                LIST_EXHAUSTIVE | LIST_EXHAUSTIVE_S => {
                    if Args::set_state_or_list(&mut state, &mut list, ParserState::List, arg) {
                        list = Some(Vec::new());
                        list_exhaustive = true;

                        if idx > 0 {
                            return Err(Error(format!(
                                "{}, {} must be the first arguments",
                                LIST_EXHAUSTIVE, LIST_EXHAUSTIVE_S
                            )));
                        }
                    }
                }
                _ => match state {
                    ParserState::Add => changes.push(Change::AddG(arg)),
                    ParserState::AddF => changes.push(Change::AddGF(arg)),
                    ParserState::Remove => changes.push(Change::RemoveG(arg)),
                    ParserState::AddItem => changes.push(Change::AddI(arg)),
                    ParserState::RemoveItem => changes.push(Change::RemoveI(arg)),
                    ParserState::List => {
                        if let Some(ref mut lst) = list {
                            lst.push(arg);
                        }
                    }
                    ParserState::None => {
                        return Err(Error(format!(
                            "Invalid command \"{}\". {} for help.",
                            arg, HELP
                        )));
                    }
                },
            }
        }

        return Ok(Args {
            create,
            version,
            help,
            list,
            list_exhaustive,
            changes,
            clear,
        });
    }

    fn set_state_or_list(
        state: &mut ParserState,
        list: &mut Option<Vec<String>>,
        new: ParserState,
        arg: String,
    ) -> bool {
        match list {
            Some(lst) => {
                lst.push(arg);
                return false;
            }
            None => {
                *state = new;
                return true;
            }
        }
    }

    pub fn add_command_to_string(
        string: &mut String,
        long: &str,
        short: &str,
        arguments: Option<&str>,
        info: &str,
        min_width: usize,
    ) {
        let mut cmd = format!("{long}, {short}");

        if let Some(arg) = arguments {
            cmd += &format!(" [{arg}]");
        }

        let left_pad = String::from(" ").repeat(4);
        let width = cmd.chars().count();

        let right_pad = if min_width > width {
            min_width - width
        } else {
            1
        };

        let right_pad = String::from(" ").repeat(right_pad);

        string.push_str(&format!("{left_pad}{cmd}{right_pad}{info}\n"));
    }

    pub fn print_help() {
        let mut string = String::new();

        let min_width = 40;

        let ignore_groups = Some("IGNORE_GROUPS");
        let ingore_items = Some("IGNORE_ITEMS");
        let filters = Some("FILTERS");

        string += &format!("{} ({})\n\n", PROGRAM_NAME, PROGRAM_VERSION);
        string += &format!("Commands:\n");

        Args::add_command_to_string(
            &mut string,
            CREATE,
            CREATE_S,
            ignore_groups,
            "Creates a .gitignore at same level of .git directory with given ignore groups.",
            min_width,
        );

        Args::add_command_to_string(
            &mut string,
            ADD,
            ADD_S,
            ignore_groups,
            "Adds given ignore group to .gitignore.",
            min_width,
        );

        Args::add_command_to_string(
            &mut string,
            ADD_ITEM,
            ADD_ITEM_S,
            ingore_items,
            "Adds given specific files/directories to .gitignore.",
            min_width,
        );

        Args::add_command_to_string(
            &mut string,
            REMOVE,
            REMOVE_S,
            ignore_groups,
            "Removes given ignore groups from .gitignore.",
            min_width,
        );

        Args::add_command_to_string(
            &mut string,
            REMOVE_ITEM,
            REMOVE_ITEM_S,
            ingore_items,
            "Removes given specific files/directories from .gitignore.",
            min_width,
        );

        Args::add_command_to_string(
            &mut string,
            LIST,
            LIST_S,
            filters,
            "Lists all ignore groups containing one of the given filters if provided.",
            min_width,
        );

        Args::add_command_to_string(
            &mut string,
            LIST_EXHAUSTIVE,
            LIST_EXHAUSTIVE_S,
            filters,
            "Lists given ignoregroups and contents.",
            min_width,
        );

        Args::add_command_to_string(
            &mut string,
            VERSION,
            VERSION_S,
            None,
            "Prints version.",
            min_width,
        );

        Args::add_command_to_string(
            &mut string,
            HELP,
            HELP_S,
            None,
            "Prints help menu.",
            min_width,
        );

        let bottom_msg = format!(
            "\n.gitignores are built based on the publicly available templates
            at https://github.com/github/gitignore. I do not claim to own or
            to have created any of the .gitignores there. {PROGRAM_NAME}
            is explicitly a tool to help manage .gitignore files."
        );

        for line in bottom_msg.lines() {
            string += " ";
            string += line.trim();
            string += "\n"
        }

        println!("{string}");
    }
}
