# d-ig

.gitignore CLI Manager

## Installation
```
cargo install d-ig
```

* crates.io page: https://crates.io/crates/d-ig/versions
* github repo: https://github.com/Owen-Dechow/d-ig

## CLI Help Menu
```
Dechow Git Ignore Builder (d-ig) (0.1.0)

Commands:
    --create, -c [IGNORE_GROUPS]            Creates a .gitignore at same level of .git directory with given ignore groups.
    --add, -a [IGNORE_GROUPS]               Adds given ignore group to .gitignore.
    --add-item, -ai [IGNORE_ITEMS]          Adds given specific files/directories to .gitignore.
    --remove, -r [IGNORE_GROUPS]            Removes given ignore groups from .gitignore.
    --remove-item, -ri [IGNORE_ITEMS]       Removes given specific files/directories from .gitignore.
    --list, -l [FILTERS]                    Lists all ignore groups containing one of the given filters if provided.
    --list-exhaustive, -le [FILTERS]        Lists given ignoregroups and contents.
    --version, -v                           Prints version.
    --help, -h                              Prints help menu.
 
 .gitignores are built based on the publicly available templates
 at https://github.com/github/gitignore. I do not claim to own or
 to have created any of the .gitignores there. Dechow Git Ignore Builder (d-ig)
 is explicitly a tool to help manage .gitignore files.
```

# Gitignore
All ignore groups are created using the gitignore repo: https://github.com/github/gitignore
