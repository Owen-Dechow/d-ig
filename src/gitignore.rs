use crate::{
    ignore_groups::{IgnoreGroup, IgnoreGroups, IgnoreItem},
    r#const::{CLEAR, CLEAR_S, GIT_IGNORE, TITLE_WRAPPER_CLOSE, TITLE_WRAPPER_OPEN},
    Error,
};
use std::{
    collections::HashMap,
    env::current_dir,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

pub struct GitIgnore(HashMap<String, String>);

impl GitIgnore {
    pub fn load(new: bool) -> Result<GitIgnore, Error> {
        let path = match new {
            false => GitIgnore::path(),
            true => GitIgnore::build_path(),
        }?;

        let file = match File::open(path) {
            Ok(file) => file,
            Err(err) => return Err(Error(format!("Could not open {GIT_IGNORE}: {err}"))),
        };

        let reader = BufReader::new(file);

        let mut map = HashMap::new();
        let mut current_name = String::from("");
        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(err) => {
                    return Err(Error(format!(
                        "Could not open read from {GIT_IGNORE}: {err}"
                    )))
                }
            };

            if line.starts_with(TITLE_WRAPPER_OPEN) && line.ends_with(TITLE_WRAPPER_CLOSE) {
                let name = line
                    .strip_prefix(TITLE_WRAPPER_OPEN)
                    .unwrap()
                    .strip_suffix(TITLE_WRAPPER_CLOSE)
                    .unwrap()
                    .trim();

                current_name = name.to_string();
                map.insert(current_name.clone(), String::new());
            } else {
                match map.get_mut(&current_name) {
                    Some(s) => s,
                    None => return Err(Error(format!("{GIT_IGNORE} has been corrupted. Please run {CLEAR}, {CLEAR_S} to clear and restart."))),
                }
                .push_str(&line);
            }
        }

        return Ok(GitIgnore(map));
    }

    pub fn write(self) -> Result<(), Error> {
        let path = GitIgnore::path()?;
        let mut content = String::new();

        for (key, val) in self.0 {
            content += &format!("{TITLE_WRAPPER_OPEN} {key} {TITLE_WRAPPER_CLOSE}\n");
            for line in val.lines() {
                let val = val.trim();
                if !val.is_empty() {
                    content += val;
                    content += "\n";
                }

                content += line;
            }

            content += "\n\n"
        }

        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(err) => return Err(Error(format!("Could not open {GIT_IGNORE}: {err}"))),
        };

        return match file.write_all(content.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error(format!("Could not write to {GIT_IGNORE}: {err}"))),
        };
    }

    fn build_path() -> Result<PathBuf, Error> {
        match current_dir() {
            Ok(path) => GitIgnore::search_for_build_path(path),
            Err(err) => return Err(Error(format!("Could not get directory directory: {}", err))),
        }
    }

    fn search_for_build_path(dir: PathBuf) -> Result<PathBuf, Error> {
        let git_path = dir.join(".git");

        if git_path.is_dir() {
            let file_path = dir.join(GIT_IGNORE);

            if file_path.exists() {
                return Err(Error(format!(
                    "{} already exists at {}.",
                    GIT_IGNORE,
                    file_path.to_string_lossy()
                )));
            }

            return match std::fs::File::create(&file_path) {
                Ok(_) => Ok(file_path),
                Err(err) => Err(Error(format!("Could not create {}: {}", GIT_IGNORE, err))),
            };
        }

        return match dir.parent() {
            Some(parent) => GitIgnore::search_for_build_path(parent.to_path_buf()),
            None => Err(Error(format!("Could not find {}", GIT_IGNORE))),
        };
    }

    pub fn path() -> Result<PathBuf, Error> {
        match current_dir() {
            Ok(path) => GitIgnore::search_for_ignore(path),
            Err(err) => {
                return Err(Error(format!(
                    "Could not get directory getting directory: {}",
                    err
                )))
            }
        }
    }

    fn search_for_ignore(dir: PathBuf) -> Result<PathBuf, Error> {
        let file_path = dir.join(GIT_IGNORE);

        if file_path.is_file() {
            return Ok(file_path);
        }

        return match dir.parent() {
            Some(parent) => GitIgnore::search_for_ignore(parent.to_path_buf()),
            None => Err(Error(format!("Could not find {}", GIT_IGNORE))),
        };
    }

    pub fn has_group(&self, group: &str) -> bool {
        self.0.contains_key(group)
    }

    pub fn add_group(
        &mut self,
        group: &IgnoreGroup,
        force: bool,
        igs: &IgnoreGroups,
    ) -> Result<(), Error> {
        if self.0.contains_key(&group.name) && !force {
            return Ok(());
        }

        let mut string = String::new();

        let mut last_was_comment = false;

        for item in &group.items {
            match item {
                IgnoreItem::Item(item) => {
                    string.push_str(&format!("{item}\n"));
                    last_was_comment = false;
                }
                IgnoreItem::Comment(comment) => {
                    if !last_was_comment {
                        string += "\n";
                        last_was_comment = true;
                    }

                    string.push_str(&format!("# {comment}\n"));
                }
                IgnoreItem::Dependency(dep) => self.add_group(igs.get(&dep)?, force, igs)?,
            }
        }

        self.0.insert(group.name.clone(), string);

        return Ok(());
    }

    pub fn add_item(&mut self, item: &str) {
        for g in self.0.values() {
            for line in g.lines() {
                if line.trim_end() == item {
                    return;
                }
            }
        }

        let default = match self.0.get_mut("_") {
            Some(default) => default,
            None => {
                self.0.insert("_".to_string(), "".to_string());
                self.0.get_mut("_").unwrap()
            }
        };

        default.push_str(&format!("\n{}", item.trim_end()));
    }

    pub fn remove_item(&mut self, item: &str) {
        for g in self.0.values_mut() {
            *g = g.replace(item, "");
        }
    }

    pub fn remove_group(&mut self, group: &str) -> Result<(), Error> {
        match self.0.remove(group) {
            Some(_) => Ok(()),
            None => Err(Error(format!(
                "{GIT_IGNORE} does not have ignore group '{group}'."
            ))),
        }
    }
}
