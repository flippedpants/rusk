# Review: potential bugs + flattening ideas

This review is based on the current code in:
- `src/commands.rs`
- `src/storage.rs`
- `src/task.rs`
- `src/main.rs`

## Potential bugs / correctness issues

### 1) `delete --completed/--pending` can delete even if user cancels
- **Where**: `src/commands.rs`, `Some(("delete", ...))` branch.
- **What happens**:
  - For `--completed` and `--pending`, you call `tasks.retain(...)` **before** asking for confirmation (`to_proceed(...)`).
  - If the user answers “No”, you print “Operation cancelled!”, but the vector is already filtered and you still call `save_to_json(tasks)` at the end of the branch.
- **Impact**: user can cancel and still lose tasks.
- **Fix**: do the confirmation **first**, then mutate `tasks` only if proceed is true.

### 2) `to_proceed(...)` returns `false` in cases where it should allow deletion
- **Where**: `src/commands.rs` `fn to_proceed(is_big_cmd: bool, force: bool) -> bool`.
- **What happens**:
  - If `is_big_cmd && !force` → prompts and returns true/false correctly.
  - **Else branch always returns `false`**, which means:
    - `--force` with a “big cmd” never proceeds.
    - A “small cmd” (non-big) never proceeds even though it doesn’t need confirmation.
- **Impact**: `delete --all` with `--force` will never delete; and the helper doesn’t match the intent implied by `--force`.
- **Fix**: the else case should return `true` (proceed) when there’s no need to prompt, i.e. `!is_big_cmd || force`.

### 3) Case-insensitive compare in single-title delete is likely wrong (and may not compile)
- **Where**: `src/commands.rs` inside `if let Some(title) = sub_arg.get_one::<String>("TITLE")`.
- **Code**: `tasks.retain(|t| t.title.to_lowercase() != *title.to_lowercase());`
- **Issues**:
  - `title.to_lowercase()` returns a `String`. Doing `*title.to_lowercase()` attempts to dereference a `String`, which is invalid.
  - Even if this compiled due to some earlier edits/variants, it’s doing extra allocations repeatedly.
- **Impact**: possible compile failure; otherwise inefficient and/or incorrect logic.
- **Fix**: precompute `let needle = title.to_lowercase();` then compare against `t.title.to_lowercase() != needle`.

### 4) `done` matches title case-sensitively, `delete` tries to match case-insensitively
- **Where**:
  - `done`: `if task.title == title`
  - `delete TITLE`: attempts a case-insensitive delete
- **Impact**: inconsistent UX (“done foo” fails but “delete foo” works, or vice-versa depending on casing).
- **Fix**: standardize matching rules for title across commands (prefer case-insensitive + trim, or exact-match, but be consistent).

### 5) `delete --completed/--pending` prints success even when not proceeding
- **Where**: `src/commands.rs` under `--completed` and `--pending`.
- **What happens**: after retention + `flag = to_proceed(...)`, you print success in `else if flag { ... }`, but you don’t restore tasks if `flag` is false. Combined with bug (1), this is especially risky.

### 6) Storage layer panics on common I/O and data errors
- **Where**: `src/storage.rs`
- **What happens**:
  - `File::create` panics on failure.
  - `read_to_string(path).unwrap()` panics on read failure.
  - `serde_json::from_str(...).expect(...)` panics if any line is corrupt.
  - `buf.flush()` result is ignored.
- **Impact**: a single corrupt line or transient filesystem error kills the CLI.
- **Fix**: return `Result<_, _>` from storage functions, or handle errors gracefully (skip bad lines with a warning; surface a user-friendly error message and exit with non-zero code).

### 7) `save_to_json(tasks: Vec<Task>)` consumes the vector unnecessarily
- **Where**: `src/storage.rs`
- **Impact**: forces callers to move `tasks` (you often don’t need that), makes control flow harder to flatten, and contributes to awkward ownership patterns.
- **Fix**: take `&[Task]` or `&Vec<Task]`.

### 8) JSONL format has no migration / versioning and rewrites whole file every time
- **Where**: `src/storage.rs`
- **Impact**: fine for small scale, but if file is partially written (crash mid-write) you can corrupt it; rewrite also increases risk window.
- **Fix**: write to a temp file then rename (atomic replace), and/or store a single JSON array if you prefer simpler parsing.

## Flattening / refactor suggestions (reduce nesting, duplication)

### A) Extract “load + mutate + save” into a small helper
Right now each command:
- loads tasks
- mutates
- saves
- prints
with lots of branching and repeated `save_to_json(tasks)` calls.

**Suggestion**:
- Make storage APIs return `Result`.
- Add helper like `with_tasks(|tasks| { ... })` that loads once, passes `&mut Vec<Task>`, and saves only when changes occurred.

### B) Replace “stringly typed” title lookups with `id`-based operations
You already generate `Uuid` in `add`, but you don’t use it for targeting.

**Suggestion**:
- Show IDs in `ls`.
- Allow `done <id>` and `delete <id>`.
- Optionally keep title as a secondary selector (with disambiguation if multiple tasks share a title).

### C) Flatten `delete` by making it “choose a predicate, maybe confirm, then apply”
Current `delete` is deeply nested (`if !empty` + 4-way else-if + per-branch logic).

**Suggestion**:
- Decide the delete “mode” first (title vs completed vs pending vs all).
- If the mode deletes multiple tasks, confirm first (unless `--force`).
- Then apply a single mutation path.
- Centralize the “how many were removed” reporting based on `(before_len - after_len)`.

### D) Make filtering logic allocation-free where possible
Repeated `.to_lowercase()` allocates.

**Suggestion**:
- Precompute the normalized needle once.
- Consider using `eq_ignore_ascii_case` if you’re fine with ASCII-only behavior.

### E) Convert `println!("{:#?}", task)` into a stable, user-friendly display
`Debug` output is noisy and not stable for CLI UX.

**Suggestion**:
- Implement `Display` for `Task`, or provide a `format_task(&Task) -> String`.
- Print columns (title, priority, status, due date, id).

### F) Use Clap derive (`#[derive(Parser, Subcommand)]`) to reduce boilerplate
The current `Command::new(...).arg(...).subcommand(...)` chain is long and contributes to nesting in `match` arms.

**Suggestion**:
- Move CLI structure into a `cli.rs` with Clap derive.
- Keep `commands.rs` focused on “execute command” logic.

### G) Align serde + parsing behavior for enums
You have:
- `#[serde(rename_all="lowercase")]` for JSON
- `EnumString` for parsing CLI strings (case-sensitive by default)

**Suggestion**:
- Explicitly accept case-insensitive inputs for CLI parsing, or document/validate more clearly.
- Alternatively parse priority/status manually to control error messages and accepted forms.

## Quick sanity checks worth adding
- **Empty tasks**: ensure `ls` prints “no tasks” instead of nothing.
- **Duplicate titles**: `done <title>` currently updates *all* matching titles (because you don’t break); that may be intended, but usually surprising.
- **Corrupt `list.jsonl`**: skip bad lines and continue, or back up the file and start fresh with a warning.

