# Configuration

Complete configuration reference for biohack.

## Configuration Sources (Priority Order)

1. **CLI flags** — Highest priority
2. **Environment variables** — Medium priority  
3. **Config file** — Lowest priority (planned)
4. **Built-in defaults** — Fallback

---

## Database

### Location

| Source | Value |
|--------|-------|
| Default | `~/.local/share/biohack/biohack.db` |
| Environment | `BIOHACK_DB=/path/to/db` |
| CLI Flag | `--db-path /path/to/db` |

### Environment Variable

```bash
# Permanent (add to ~/.bashrc, ~/.zshrc, etc.)
export BIOHACK_DB="$HOME/data/biohack.db"

# Temporary (single command)
BIOHACK_DB=/tmp/test.db biohack substance list
```

### CLI Flag

```bash
biohack --db-path /data/biohack.db substance list
```

### Multiple Databases

You can maintain separate databases for different purposes:

```bash
# Main tracking
export BIOHACK_DB="$HOME/.local/share/biohack/main.db"

# Experiment-specific
export BIOHACK_DB="$HOME/.local/share/biohack/experiment-magnesium.db"
```

---

## Output

### Colors

| Source | Value |
|--------|-------|
| Default | Enabled |
| Environment | Not yet supported |
| CLI Flag | `--no-color` |

```bash
# Disable colors
biohack --no-color substance list

# Pipe-friendly output
biohack --no-color substance list | grep caffeine
```

### Format

Currently only table format is supported. Future versions will support:
- `json` — Machine-readable
- `csv` — Spreadsheet import
- `markdown` — Documentation

```bash
# Future
biohack --format json substance list
biohack --format csv substance list > export.csv
```

---

## Logging (RUST_LOG)

Control log verbosity via the standard `RUST_LOG` environment variable:

```bash
# Show info logs (default)
RUST_LOG=info biohack substance list

# Show debug logs
RUST_LOG=debug biohack substance seed

# Show only warnings/errors
RUST_LOG=warn biohack check

# Module-specific
RUST_LOG=biohack::protocols=debug,biohack::db=info biohack check
```

**Levels:** `trace`, `debug`, `info`, `warn`, `error`

---

## Config File (Planned)

Future versions will support `~/.config/biohack/config.toml`:

```toml
# ~/.config/biohack/config.toml

[database]
# Override default database path
path = "~/.local/share/biohack/biohack.db"

[output]
# Enable/disable colors
color = true

# Output format: "table", "json", "csv", "markdown"
format = "table"

# Show emoji indicators
emoji = true

[safety]
# Automatically run safety check after each log entry
check_on_log = false

# Custom protocol directory
protocol_dir = "~/.config/biohack/protocols/"

[cli]
# Default days for show commands
default_days = 3

# Auto-run check after logging
check_after_log = false

[logging]
# Log level: trace, debug, info, warn, error
level = "info"

# Log to file
file = "~/.local/share/biohack/biohack.log"
```

### Config File Locations (Priority Order)

1. `./biohack.toml` (current directory)
2. `~/.config/biohack/config.toml`
3. `/etc/biohack/config.toml` (system-wide)

---

## Safety Protocols

### Protocol Directory (Planned)

```toml
[safety]
protocol_dir = "~/.config/biohack/protocols/"
```

Custom protocols in this directory will be loaded automatically (future feature).

### Default Protocols

The three built-in protocols are always loaded:
1. `stimulant_tachycardia`
2. `hypertension_urgency`
3. `serotonin_syndrome_risk`

### Disable Protocols (Planned)

```toml
[safety]
disabled_protocols = ["serotonin_syndrome_risk"]
```

---

## Substance Database

### Seed File

Default seed location: `data/seeds/substances.yaml`

Override via CLI:
```bash
biohack substance seed --path /path/to/custom.yaml
```

### Custom Substances (Planned)

```toml
[database]
custom_substances_file = "~/.config/biohack/custom-substances.yaml"
```

---

## Shell Completion

Generate shell completions:

```bash
# Bash
biohack completions bash > /usr/share/bash-completion/completions/biohack

# Zsh
biohack completions zsh > ~/.zsh/completions/_biohack

# Fish
biohack completions fish > ~/.config/fish/completions/biohack.fish

# PowerShell
biohack completions powershell > biohack.ps1
```

---

## Environment Variables Summary

| Variable | Description | Default |
|----------|-------------|---------|
| `BIOHACK_DB` | Database file path | `~/.local/share/biohack/biohack.db` |
| `RUST_LOG` | Log level (trace/debug/info/warn/error) | `info` |
| `BIOHACK_CONFIG` | Config file path (planned) | Auto-discovered |

---

## Platform-Specific Notes

### Linux

- Database: `~/.local/share/biohack/biohack.db`
- Config: `~/.config/biohack/config.toml`
- Completions: `/usr/share/bash-completion/completions/` or `~/.local/share/bash-completion/completions/`

### macOS

- Database: `~/Library/Application Support/biohack/biohack.db`
- Config: `~/Library/Preferences/biohack/config.toml`
- Completions: `/usr/local/share/zsh/site-functions/` (zsh) or `~/.config/fish/completions/` (fish)

### Windows (WSL)

- Database: `~/.local/share/biohack/biohack.db` (Linux path)
- Config: `~/.config/biohack/config.toml`
- For native Windows build, use `%APPDATA%\biohack\biohack.db`

---

## Migration Between Config Versions

When config schema changes:

1. **Minor changes** — New fields get defaults automatically
2. **Major changes** — Migration tool will be provided (`biohack config migrate`)
3. **Breaking changes** — Documented in CHANGELOG.md and release notes

---

## Security Notes

- **No secrets in config** — Never store API keys, tokens, or passwords in config files
- **Database permissions** — `sled` creates files with default permissions (user-readable only)
- **Environment variables** — Preferred for sensitive paths in shared environments

---

## See Also

- [User Guide](user-guide.md) — Workflow examples
- [Command Reference](command-reference.md) — CLI flags for each command
- [Troubleshooting](troubleshooting.md) — Config-related issues