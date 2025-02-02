use crate::{
    r#const::{BLUE, GREEN, IGNORE_FILE, RESET, YELLOW},
    Error,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct IgnoreGroups {
    groups: HashMap<String, IgnoreGroup>,
}

impl IgnoreGroups {
    pub fn parse() -> Result<IgnoreGroups, Error> {
        let mut groups = HashMap::new();
        let mut active_group = None;
        for line in IGNORE_FILE.lines() {
            if line.is_empty() {
                continue;
            }

            if line.starts_with("#") {
                let group_name = line.strip_prefix("#").unwrap().trim();
                groups.insert(
                    group_name.to_string(),
                    IgnoreGroup {
                        items: Vec::new(),
                        name: group_name.to_string(),
                    },
                );
                active_group = Some(groups.get_mut(group_name).unwrap());
                continue;
            }

            if line.starts_with("Item") {
                let file_name = line
                    .strip_prefix("Item(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .trim();

                if let Some(ref mut ig) = active_group {
                    ig.items.push(IgnoreItem::Item(file_name.to_string()));
                    continue;
                }
            }

            if line.starts_with("Comment") {
                let file_name = line
                    .strip_prefix("Comment(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .trim();

                if let Some(ref mut ig) = active_group {
                    ig.items.push(IgnoreItem::Comment(file_name.to_string()));
                    continue;
                }
            }

            if line.starts_with("Dependency") {
                let file_name = line
                    .strip_prefix("Dependency(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .trim();

                if let Some(ref mut ig) = active_group {
                    ig.items.push(IgnoreItem::Dependency(file_name.to_string()));
                    continue;
                }
            }

            return Err(Error(format!("Could not parse ingore group '{}'", line)));
        }

        return Ok(IgnoreGroups { groups });
    }

    pub fn filter(&self, filters: &Vec<String>) -> Vec<String> {
        let mut keys: Vec<String> = if filters.is_empty() {
            self.groups.keys().cloned().collect()
        } else {
            let pred = |x: &str| {
                for y in filters {
                    if x.to_lowercase().contains(&y.trim().to_lowercase()) {
                        return true;
                    }
                }
                return false;
            };

            self.groups.keys().filter(|x| pred(x)).cloned().collect()
        };

        keys.sort();

        return keys;
    }

    pub fn cat(&self, key: &String) {
        let ig = &self.groups[key];

        let mut string = format!("##### {key} #####\n");

        for item in &ig.items {
            string += &item.doc();
            string += "\n"
        }

        string += &format!("##### {key} #####");

        println!("{string}\n");
    }

    pub fn cat_keys(&self, keys: Vec<String>) {
        for (idx, key) in keys.into_iter().enumerate() {
            let color = [GREEN, YELLOW, BLUE][idx % 3];
            if idx % 4 == 3 {
                println!("{color}{key}{RESET}");
            } else {
                print!("{color}{key}{RESET}, ");
            }
        }
    }

    pub fn get(&self, group: &str) -> Result<&IgnoreGroup, Error> {
        match self.groups.get(group) {
            Some(group) => Ok(group),
            None => Err(Error(format!("Ignore group {group} does not exist."))),
        }
    }
}

#[derive(Debug)]
pub struct IgnoreGroup {
    pub name: String,
    pub items: Vec<IgnoreItem>,
}

#[derive(Debug)]
pub enum IgnoreItem {
    Item(String),
    Comment(String),
    Dependency(String),
}

impl IgnoreItem {
    fn doc(&self) -> String {
        match self {
            IgnoreItem::Item(item) => format!("{GREEN}Item({item}){RESET}"),
            IgnoreItem::Comment(comment) => format!("{YELLOW}Comment({comment}){RESET}"),
            IgnoreItem::Dependency(dep) => format!("{BLUE}Comment({dep}){RESET}"),
        }
    }
}
