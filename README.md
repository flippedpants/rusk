# Rusk (Only for linux right now)

`Rusk` is a small Rust-based command-line tool for managing tasks from your terminal.
- add tasks with a priority
- list them (all / pending / completed), mark tasks as completed
- delete tasks

Tasks are saved locally in `tasks.jsonl` which is in `~/.rusk`

## Installation
```bash
curl -sL https://github.com/flippedpants/rusk/releases/latest/download/install.sh | bash
```

## Commands

### add
Add a new task (it starts as `Pending`).

```bash
rusk add -t "Buy milk" -p High
```

Options:
- `-t, --title <TITLE>`: Task title (required)
- `-p, --priority <Low|Medium|High>`: Priority (required and case sensitive)

### ls
List tasks.

```bash
# list all tasks
rusk ls

# list only pending tasks
rusk ls --pending

# list only completed tasks
rusk ls --completed
# or: rusk ls -c
```

### done
Mark a task as completed (matches by title, case-insensitive).

```bash
rusk done "Buy milk"
```

### delete
Delete tasks.

```bash
# delete by title (case-insensitive)
rusk delete "Buy milk"

# delete all pending/completed/all tasks (asks for confirmation unless --force)
rusk delete --pending
rusk delete --completed
rusk delete --all

# skip confirmation prompt
rusk delete --pending --force
```

Notes:
- For `delete`, use either a `TITLE` or one of `--all`, `--pending`, `--completed`.
- `--force` skips the confirmation prompt for bulk deletes.

### update
Update to the latest version.

```bash
rusk update
```

## Built With
- Rust
- Clap
- Serde