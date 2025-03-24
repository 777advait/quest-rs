## quest-rs

A simple obsidian-like command-line tool for note taking built in rust 🔥.

### Commands

#### 1. Vaults

- `quest vault list` - List all vaults
- `quest vault create <name>` - Create a new vault
- `quest vault delete <name>` - Delete a vault
- `quest vault rename <name> --new <new-name>` - Rename a vault
- `quest vault trunc <name>` - Truncate a vault

#### 2. Logs (notes)

- `quest log list <vault-name>` - List all logs in a vault
- `quest log create <name> --vault <vault-name>` - Create a new log in a vault
- `quest log delete <id>` - Delete a log
- `quest log rename <id> --new <new-name>` - Rename a log
- `quest log edit <id>` - Edit a log

### Roadmap

- [x] Setup models
  - [x] Vault
  - [x] Log
- [ ] Setup Repositories
  - Vault
    - [x] create vault
    - [ ] list vaults
    - [ ] delete vault
    - [ ] rename vault
    - [ ] truncate vault
  - Log
    - [ ] create log
    - [ ] list logs
    - [ ] delete log
    - [ ] rename log
    - [ ] edit log
- [ ] Setup CLI
- [ ] Setup TUI
