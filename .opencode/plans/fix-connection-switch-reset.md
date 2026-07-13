# Fix: Connection switch doesn't reset editor and database browser state

## Problem
When activating a new connection via Enter in the Connections panel, `connect_to_active()` doesn't clear several state fields, leaving stale data from the old connection visible:
1. **Editor**: `query_blocks` (old query results) remain visible
2. **Database browser**: `db_browser_databases` (old database list) stays in memory  
3. **Completion engine**: `tables`/`columns` silently keep stale schema if `fetch_schema` fails
4. **Input buffer**: `input`/`cursor` keep old query text

## Fix
Add state resets in `connect_to_active()` at `src/app/connections.rs:28`:

```rust
self.query_blocks.clear();
self.active_block = 0;
self.scroll = 0;
self.scroll_x = 0;
self.input.clear();
self.cursor = 0;
self.db_browser_databases.clear();
self.db_browser_selection = 0;
self.db_browser_error = None;
self.completion.tables.clear();
self.completion.columns.clear();
```

Insert these before the `if let Some(ref db) = self.db { self.completion.fetch_schema(db).await; }` call.

## Verification
```sh
cargo check && cargo clippy -- -D warnings && cargo fmt --check
```
