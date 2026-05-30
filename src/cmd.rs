pub struct Command {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub description: &'static str,
    pub _category: &'static str,
}

pub static COMMANDS: &[Command] = &[
    Command {
        name: "theme",
        aliases: &["th"],
        description: "Select a color theme",
        _category: "view",
    },
    Command {
        name: "help",
        aliases: &["h"],
        description: "Show available commands",
        _category: "util",
    },
    Command {
        name: "clear",
        aliases: &["cls"],
        description: "Clear query results",
        _category: "util",
    },
    Command {
        name: "quit",
        aliases: &["exit", "q"],
        description: "Quit kon",
        _category: "util",
    },
    Command {
        name: "tables",
        aliases: &["tbl", "schemas"],
        description: "List database tables",
        _category: "sql",
    },
    Command {
        name: "refresh",
        aliases: &["rf"],
        description: "Refresh schema cache",
        _category: "util",
    },
];

#[allow(dead_code)]
pub fn resolve(name: &str) -> Option<&'static Command> {
    COMMANDS
        .iter()
        .find(|cmd| cmd.name == name || cmd.aliases.contains(&name))
}

pub fn all_names() -> Vec<&'static str> {
    COMMANDS.iter().map(|cmd| cmd.name).collect()
}
