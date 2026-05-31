use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "kon", about = "A MySQL CLI tool", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List saved connections
    Ls,
    /// Set active connection by name (supports wildcard *)
    Set {
        /// Connection name or wildcard pattern
        pattern: String,
    },
}
