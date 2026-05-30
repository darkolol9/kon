# kon

A MySQL CLI tool with an interactive TUI editor.

## Features

- **Connection management** — save named connections, switch between them
- **Interactive TUI editor** — execute SQL queries in a terminal UI
- **Syntax highlighting** — keywords, strings, numbers colorized via `sqlparser`
- **Autocomplete** — Tab-completes SQL keywords, table names, and column names
- **Formatted table output** — results displayed with aligned columns and borders
- **History** — query history with up/down navigation

## Installation

### One-liner (Linux / macOS)

```bash
curl -fsSL https://github.com/darkolol9/kon/releases/latest/download/install.sh | sh
```

Installs to `/usr/local/bin`. Override with `KON_INSTALL_DIR`:

```bash
curl -fsSL ... | KON_INSTALL_DIR=~/.local/bin sh
```

### Cargo

```bash
cargo install kon
```

### Build from source

```bash
git clone https://github.com/darkolol9/kon
cd kon
cargo build --release
cp target/release/kon ~/.local/bin/
```

### Prebuilt binaries

Download from [GitHub Releases](https://github.com/darkolol9/kon/releases) — available for Linux (musl, static), macOS, and Windows.

## Quick Start

### 1. Add a connection

```bash
kon connect
```

Follow the prompts: name, host, port, user, password, database. Choose whether to set it as the active connection.

### 2. List saved connections

```bash
kon ls
```

The active connection is marked with `*`.

### 3. Switch connections

```bash
kon set <name>
```

Supports wildcard patterns (`*`, `?`). If multiple connections match, you'll be prompted to pick one.

### 4. Launch the REPL

```bash
kon
```

Opens the interactive SQL editor using the active connection.

## REPL Usage

```
┌─ kon · my-db ──────────────────────────────────┐
│                                                  │
│  mysql> SELECT name, email FROM users LIMIT 3;  │
│                                                  │
│  ┌──────────┬──────────────────────┐             │
│  │ name     │ email                │             │
│  ├──────────┼──────────────────────┤             │
│  │ Alice    │ alice@example.com    │             │
│  │ Bob      │ bob@example.com      │             │
│  │ Carol    │ carol@example.com    │             │
│  └──────────┴──────────────────────┘             │
│  3 row(s) in set (2 ms)                          │
│                                                  │
│  sql> _                                          │
├──────────────────────────────────────────────────┤
│ Ctrl+D:Quit | PgUp/PgDn:Scroll | 3 queries      │
└──────────────────────────────────────────────────┘
```

### Key Bindings

| Key       | Action                     |
|-----------|----------------------------|
| `Enter`   | Execute query              |
| `Tab`     | Cycle autocomplete options |
| `↑`       | Previous history entry     |
| `↓`       | Next history entry         |
| `←` `→`   | Move cursor                |
| `Home`    | Move to start of line      |
| `End`     | Move to end of line        |
| `Backspace` | Delete character before cursor |
| `Delete`  | Delete character at cursor |
| `PgUp`    | Scroll results up          |
| `PgDn`    | Scroll results down        |
| `Ctrl+C` / `Ctrl+D` | Quit               |

### Autocomplete

On connect, `kon` fetches the table and column schema. As you type, press `Tab` to cycle through matching SQL keywords, table names, and column names. The current candidate appears in the status bar.

## Configuration

readmeConnections are stored in `~/.config/kon/config.toml`:

```toml
active_connection = "my-db"

[connections.my-db]
host = "localhost"
port = 3306
user = "root"
password = "your-password"
database = "myapp"

[connections.staging]
host = "staging.example.com"
port = 3306
user = "deploy"
password = "deploy-password"
database = "myapp"
```

Passwords are stored in plaintext.

## Build Dependencies

- Rust 1.85+ (edition 2024)
- OpenSSL / `libssl-dev` (for sqlx MySQL TLS)
- `pkg-config`
