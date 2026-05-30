# UI Fix Plan

## Problem 1: Input bar invisible

**File:** `src/tui/editor.rs:16`
**Fix:** Change `let input_height = 1u16;` → `let input_height = 2u16;`

The block uses `Borders::TOP` which consumes 1 line. With height=1 the text is hidden off-screen. Height=2 gives 1 line for border + 1 line for input text and cursor.

---

## Problem 2: Panel switching never shown in bottom bar

**File:** `src/tui/bottom_bar.rs`

Append `"  |  Alt+1 Editor  Alt+2 Conn  Alt+3 Settings"` to every context-specific shortcut message. The `_` fallback can remain but is rarely reached.

Changes:
- Line 22: Editor input → `" ↵ Execute  ⌃P Palette ...  |  Alt+1 Editor  Alt+2 Conn  Alt+3 Settings"`
- Line 26: Editor results → `" ⌃V Toggle View  ⌃O Editor ...  |  Alt+1 Editor  Alt+2 Conn  Alt+3 Settings"`
- Line 29: Schema browser → `" ↑↓ Navigate  ↵ Insert  |  Alt+1 Editor  Alt+2 Conn  Alt+3 Settings"`
- Line 32: History browser → `" ↑↓ Navigate  ↵ Paste  |  Alt+1 Editor  Alt+2 Conn  Alt+3 Settings"`
- Lines 37-40: Connections list → `" ↑↓ Navigate  ↵ Activate  n New  |  Alt+1 Editor  Alt+2 Conn  Alt+3 Settings"`
- Line 44: Connection form → `" ⇥ Next  ↵ Save  ⎋ Cancel  |  Alt+1 Editor  Alt+2 Conn  Alt+3 Settings"`
- Line 47: Settings list → `" ↑↓ Navigate  ↵ Apply  |  Alt+1 Editor  Alt+2 Conn  Alt+3 Settings"`

---

## Problem 3 (Polish): Input prompt is just ` sql> `, no connection context

**File:** `src/tui/editor.rs:115`

Change prefix from `" sql> "` to include connection name, e.g.:
```rust
let prefix = format!(" {}> ", app.conn_name);
```
This mirrors MySQL CLI convention (`user@host:port [db]>`) and helps the user identify which connection they're on.

---

## Verification

```bash
cargo check && cargo clippy -- -D warnings && cargo fmt --check
```
