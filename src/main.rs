mod app;
mod cli;
mod cmd;
mod config;
mod db;
mod theme;
mod tui;

use clap::Parser;
use cli::{Cli, Commands};
use config::Config;
use inquire::Select;
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
        Some(Commands::Ls) => cmd_ls(&config),
        Some(Commands::Set { pattern }) => cmd_set(&mut config, &pattern),
        None => cmd_repl(&mut config).await,
    };

    if let Err(e) = result {
        print_error(&e);
        process::exit(1);
    }
}

fn cmd_ls(config: &Config) -> Result<(), String> {
    let list = config.list_connections();
    if list.is_empty() {
        println!("No connections saved. Run \x1b[1mkon\x1b[0m to set one up.");
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

async fn cmd_repl(config: &mut Config) -> Result<(), String> {
    let active = config.active();
    let theme_name = config.theme.as_deref().unwrap_or("default");
    let theme = theme::from_name(theme_name).unwrap_or(&theme::DEFAULT);
    match active {
        Some((name, conn)) => {
            let db = db::Database::connect(conn).await?;
            let header = if conn.database == "mysql" {
                name.to_string()
            } else {
                format!("{} ({})", name, conn.database)
            };
            let app = app::App::new(Some(db), header, theme);
            let terminal = ratatui::init();
            app::event::run(terminal, app).await
        }
        None => {
            let app = app::App::new_setup(config.clone(), theme);
            let terminal = ratatui::init();
            app::event::run(terminal, app).await
        }
    }
}
