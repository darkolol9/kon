mod cli;
mod config;
mod db;
mod repl;

use clap::Parser;
use cli::{Cli, Commands};
use config::{Config, Connection};
use inquire::{Confirm, Password, Select, Text};
use std::process;

fn print_header(text: &str) {
    println!("\n\x1b[1;36m{}\x1b[0m", text);
    println!("\x1b[1;36m{}\x1b[0m", "─".repeat(text.len()));
}

fn print_error(msg: &str) {
    eprintln!("\x1b[1;31merror:\x1b[0m {msg}");
}

fn print_success(msg: &str) {
    println!("\x1b[1;32m✓\x1b[0m {msg}");
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut config = Config::load();

    let result = match cli.command {
        Some(Commands::Connect) => cmd_connect(&mut config),
        Some(Commands::Ls) => cmd_ls(&config),
        Some(Commands::Set { pattern }) => cmd_set(&mut config, &pattern),
        None => cmd_repl(&config).await,
    };

    if let Err(e) = result {
        print_error(&e);
        process::exit(1);
    }
}

fn cmd_connect(config: &mut Config) -> Result<(), String> {
    print_header("New Connection");

    let name = Text::new("Connection name:")
        .with_placeholder("e.g. my-db, staging, prod")
        .prompt()
        .map_err(|e| format!("Prompt failed: {e}"))?;

    let host = Text::new("Host:")
        .with_default("localhost")
        .prompt()
        .map_err(|e| format!("Prompt failed: {e}"))?;

    let port: u16 = Text::new("Port:")
        .with_default("3306")
        .prompt()
        .map_err(|e| format!("Prompt failed: {e}"))?
        .parse()
        .map_err(|_| "Invalid port number".to_string())?;

    let user = Text::new("User:")
        .with_default("root")
        .prompt()
        .map_err(|e| format!("Prompt failed: {e}"))?;

    let password = Password::new("Password:")
        .without_confirmation()
        .prompt()
        .map_err(|e| format!("Prompt failed: {e}"))?;

    let database = Text::new("Database:")
        .with_default("mysql")
        .prompt()
        .map_err(|e| format!("Prompt failed: {e}"))?;

    let conn = Connection {
        host,
        port,
        user,
        password,
        database,
    };

    let set_active = Confirm::new("Set as active connection?")
        .with_default(true)
        .prompt()
        .map_err(|e| format!("Prompt failed: {e}"))?;

    config.add_connection(name.clone(), conn)?;

    if set_active {
        config.set_active(&name)?;
    }

    print_success(&format!("Connection '{}' saved", name));
    Ok(())
}

fn cmd_ls(config: &Config) -> Result<(), String> {
    let list = config.list_connections();
    if list.is_empty() {
        println!("No connections saved. Use \x1b[1mkon connect\x1b[0m to add one.");
        return Ok(());
    }

    print_header("Saved Connections");

    for (name, conn) in &list {
        let active = config.active_connection.as_deref() == Some(name);
        let marker = if active { "\x1b[1;32m*\x1b[0m " } else { "  " };
        let style = if active { "\x1b[1;33m" } else { "" };
        println!(
            "{}{}{} {}@{}:{}/{}",
            marker, style, name, conn.user, conn.host, conn.port, conn.database
        );
    }

    Ok(())
}

fn cmd_set(config: &mut Config, pattern: &str) -> Result<(), String> {
    let matches: Vec<String> = config
        .wildcard_match(pattern)
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    match matches.len() {
        0 => {
            return Err(format!(
                "No connections match '{}'. Use \x1b[1mkon ls\x1b[0m to list connections.",
                pattern
            ));
        }
        1 => {
            config.set_active(&matches[0])?;
            print_success(&format!("Active connection set to '{}'", matches[0]));
        }
        _ => {
            let selection = Select::new(
                &format!("Multiple connections match '{}':", pattern),
                matches,
            )
            .prompt()
            .map_err(|e| format!("Selection failed: {e}"))?;

            config.set_active(&selection)?;
            print_success(&format!("Active connection set to '{}'", selection));
        }
    }

    Ok(())
}

async fn cmd_repl(config: &Config) -> Result<(), String> {
    let active = config.active();
    match active {
        Some((name, conn)) => {
            let db = db::Database::connect(conn).await?;
            let app = repl::App::new(db, name.to_string());
            let terminal = ratatui::init();
            
            repl::run(terminal, app).await
        }
        None => {
            Err("No active connection. Use \x1b[1mkon connect\x1b[0m to add one, or \x1b[1mkon set <name>\x1b[0m to select one.".to_string())
        }
    }
}
