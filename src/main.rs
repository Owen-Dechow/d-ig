mod args;
pub mod r#const;
mod gitignore;
mod ignore_groups;

use std::fs::File;

use args::{Args, Change};
use gitignore::GitIgnore;
use ignore_groups::IgnoreGroups;
use r#const::{FORCE_ADD, FORCE_ADD_S, GIT_IGNORE, PROGRAM_NAME, PROGRAM_VERSION, RED};

#[derive(Debug)]
struct Error(String);

impl Error {
    fn log(self) {
        println!("{RED}ERROR: {}", self.0)
    }
}

fn run() -> Result<(), Error> {
    let args = Args::parse()?;

    if args.version {
        println!("{} ({})", PROGRAM_NAME, PROGRAM_VERSION);
        return Ok(());
    }

    if args.help {
        Args::print_help();
        return Ok(());
    }

    if args.clear {
        let path = GitIgnore::path()?;

        return match File::create(path) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error(format!("Could not clear {GIT_IGNORE}: {err}"))),
        };
    }

    let igs = IgnoreGroups::parse()?;

    if let Some(lst) = args.list {
        match args.list_exhaustive {
            true => {
                let keys = igs.filter(&lst);

                for key in keys {
                    igs.cat(&key);
                }

                return Ok(());
            }
            false => {
                let keys = igs.filter(&lst);
                igs.cat_keys(keys);
                return Ok(());
            }
        }
    }

    let mut gitignore = GitIgnore::load(args.create)?;

    for change in args.changes {
        match change {
            Change::AddG(group) => {
                if !gitignore.has_group(&group) {
                    gitignore.add_group(igs.get(&group)?, false, &igs)?
                } else {
                    return Err(Error(format!(
                        "Group '{}' already exists in {}. Perhaps use {}, {}.",
                        group, GIT_IGNORE, FORCE_ADD, FORCE_ADD_S
                    )));
                }
            }
            Change::AddI(item) => gitignore.add_item(&item),
            Change::RemoveG(group) => gitignore.remove_group(&group)?,
            Change::RemoveI(item) => gitignore.remove_item(&item),
            Change::AddGF(group) => gitignore.add_group(igs.get(&group)?, true, &igs)?,
        }
    }

    gitignore.write()?;

    return Ok(());
}

fn main() {
    if let Err(err) = run() {
        err.log();
    }
}
