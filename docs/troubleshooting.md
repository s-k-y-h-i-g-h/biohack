# Troubleshooting

Common issues, solutions, and FAQ for biohack.

## Installation Issues

### `cargo build` fails

**Problem:** Build fails with missing dependencies or compilation errors.

**Solutions:**
1. Update Rust: `rustup update`
2. Clean and rebuild: `cargo clean && cargo build`
3. Check Rust version: `rustc --version` (needs 1.78+)
4. Install build dependencies (Ubuntu/Debian):
   ```bash
   sudo apt update && sudo apt install build-essential pkg-config libssl-dev
   ```
5. On macOS: `xcode-select --install`

### `sled` compilation fails

**Problem:** `sled` (embedded database) fails to compile.

**Solutions:**
1. Ensure you have a C compiler: `gcc --version` or `clang --version`
2. On Ubuntu: `sudo apt install build-essential pkg-config`
3. On macOS: `xcode-select --install`
4. Try clean build: `cargo clean && cargo build`

### Binary not found after build

**Problem:** `biohack` command not found.

**Solution:**
```bash
# Add to PATH
export PATH="$HOME/.cargo/bin:$PATH"
# Or copy binary
cp target/release/biohack ~/.local/bin/
```

---

## Database Issues

### `biohack init` fails

**Problem:** Database initialization fails.

**Solutions:**
1. Check disk space: `df -h`
2. Check permissions: `ls -la ~/.local/share/biohack/`
3. Remove corrupted database and retry:
   ```bash
   rm ~/.local/share/biohack/biohack.db
   biohack init
   ```

### "Database locked" or "IO error"

**Problem:** Database is locked by another process.

**Solutions:**
1. Ensure no other biohack instances are running: `pkill biohack`
2. Check for stale lock files: `ls -la ~/.local/share/biohack/`
3. Restart terminal/session
4. Reboot if persistent

### Database corruption

**Problem:** Strange errors when reading/writing.

**Solutions:**
1. Backup first: `cp ~/.local/share/biohack/biohack.db ~/biohack-backup.db`
2. Remove and reinitialize:
   ```bash
   rm ~/.local/share/biohack/biohack.db
   biohack init
   biohack substance seed
   ```

---

## CLI Issues

### "Command not found" for subcommands

**Problem:** `biohack log substance` works but `biohack substance list` doesn't.

**Solution:** Check exact command spelling:
```bash
biohack --help          # Shows top-level commands
biohack substance --help # Shows substance subcommands
biohack log --help       # Shows log subcommands
```

### Short option conflicts

**Problem:** Error: "Short option names must be unique"

**Solution:** Use long options instead:
```bash
# Instead of -n (conflicts with notes)
biohack log substance --name "L-Theanine" --dose 400mg

# Instead of -d (conflicts with db-path)
biohack log substance --name "L-Theanine" --dose 400mg
```

### Colors not showing / garbled output

**Problem:** Colors don't appear or show escape codes.

**Solutions:**
```bash
# Force colors
biohack --no-color substance list

# Check terminal supports colors
echo $TERM
# Should be xterm-256color, screen-256color, etc.

# Force color support
export TERM=xterm-256color
```

### Help text truncated

**Problem:** Long help text gets cut off.

**Solution:**
```bash
# Pipe to less
biohack --help | less
biohack log substance --help | less
```

---

## Database Seeding Issues

### "missing field `id`" error

**Problem:** Seed YAML missing UUIDs.

**Solution:** Ensure each substance has an `id` field:
```yaml
- name: "Test"
  id: "550e8400-e29b-41d4-a716-446655440001"
  category: "supplement"
  # ... other fields
```

Generate UUIDs with:
```bash
python3 -c "import uuid; print(uuid.uuid4())"
# or
uuidgen
```

### "unknown variant" for category

**Problem:** Category name not recognized.

**Solution:** Use exact category names (case-sensitive):
| Valid Categories |
|------------------|
| `supplement` |
| `medication` |
| `drug` |
| `nootropic` |
| `hormone` |
| `peptide` |
| `electrolyte` |
| `vitamin` |
| `mineral` |
| `herb` |
| `stimulant` |
| `other` (requires string: `other: "custom"`) |

### Seed file not found

**Problem:** `biohack substance seed` can't find file.

**Solution:** Use absolute path:
```bash
biohack substance seed --path /full/path/to/substances.yaml
```

---

## Safety Protocol Issues

### `biohack check` shows nothing

**Problem:** No protocols triggered even when expected.

**Checklist:**
1. Did you log substances with correct categories?
   ```bash
   biohack log substance --name "Caffeine" --dose 100mg
   # Caffeine must have category "stimulant" in database
   ```
2. Did you log vitals recently?
   ```bash
   biohack log vitals --hr 110 --sbp 140 --dbp 90
   ```
3. Check substance category in database:
   ```bash
   biohack substance show "Caffeine"
   # Should show: category: stimulant
   ```
4. Run with verbose output:
   ```bash
   biohack -v check
   ```

### Protocol not triggering for hypertension

**Problem:** BP readings don't trigger hypertensive urgency.

**Check:** Ensure you're logging systolic/diastolic correctly:
```bash
# Correct
biohack log vitals --sbp 185 --dbp 95

# Not this (missing sbp)
biohack log vitals --hr 90 --dbp 95
```

### Custom protocol not loading

**Problem:** Custom protocol not appearing in `biohack check`.

**Note:** Custom protocols currently require recompilation. Edit `src/protocols.rs` and rebuild:
```bash
cargo build --release
```

---

## Performance Issues

### Slow startup

**Problem:** `biohack` takes >1 second to start.

**Solutions:**
1. Use release build: `cargo build --release`
2. Check database size: `ls -lh ~/.local/share/biohack/biohack.db`
3. If database is large (>100MB), consider archiving old logs (future feature)

### High memory usage

**Problem:** biohack uses lots of RAM.

**Solutions:**
1. This is normal for Rust binaries (includes runtime)
2. Release builds are smaller: `cargo build --release`

---

## Logging Issues

### No logs appearing

**Problem:** `RUST_LOG=debug` produces no output.

**Solution:**
```bash
# Module-specific
RUST_LOG=biohack=debug biohack check

# All modules
RUST_LOG=trace biohack check 2>&1 | head -50
```

### Log output mixed with command output

**Problem:** Debug logs interfere with command output.

**Solution:** Separate stderr:
```bash
biohack check 2>debug.log
```

---

## Data Recovery

### Accidentally deleted database

**Problem:** Removed `~/.local/share/biohack/biohack.db`

**Recovery:**
1. Check for backups: `ls ~/biohack-backup*.db`
2. If no backup, reinitialize:
   ```bash
   biohack init
   biohack substance seed
   ```
3. Manually re-enter recent logs from memory/notes

### Corrupted YAML seed file

**Problem:** `substances.yaml` has syntax errors.

**Solution:** Validate YAML:
```bash
python3 -c "import yaml; yaml.safe_load(open('data/seeds/substances.yaml'))"
# Or use online YAML validator
```

---

## Platform-Specific Issues

### WSL (Windows Subsystem for Linux)

**Problem:** File permission errors, slow I/O.

**Solutions:**
1. Store database in Linux filesystem (not `/mnt/c/`):
   ```bash
   export BIOHACK_DB="$HOME/.local/share/biohack/biohack.db"
   ```
2. Use `wsl --shutdown` and restart if filesystem gets stuck

### macOS

**Problem:** "Developer cannot be verified" when running binary.

**Solution:**
```bash
# Remove quarantine attribute
xattr -d com.apple.quarantine ~/.local/bin/biohack
# Or allow in System Preferences > Security & Privacy > General
```

### Linux (no systemd)

**Problem:** Want to run as service.

**Solution:** Create systemd user service:
```ini
# ~/.config/systemd/user/biohack-check.timer
[Unit]
Description=Hourly biohack safety check

[Timer]
OnCalendar=hourly
Persistent=true

[Install]
WantedBy=timers.target
```

---

## Getting Help

### Debug Information

When reporting issues, include:
```bash
# System info
uname -a
rustc --version
cargo --version

# biohack info
biohack --version
biohack -v check 2>&1 | head -20

# Database info
ls -lh ~/.local/share/biohack/
```

### Reporting Bugs

1. Check existing issues: https://github.com/s-k-y-h-i-g-h/biohack/issues
2. Create new issue with:
   - biohack version
   - OS and version
   - Steps to reproduce
   - Expected vs actual behavior
   - Debug output from above

### Feature Requests

Open an issue with:
- Use case description
- Proposed solution
- Willingness to contribute implementation

---

## FAQ

**Q: Can I import data from other apps (Apple Health, Google Fit, etc.)?**
A: Not yet implemented. Planned for v1.1. See [Configuration](configuration.md#planned-features).

**Q: Can I use biohack on multiple devices?**
A: Yes, copy the database file (`~/.local/share/biohack/biohack.db`) or use a synced folder (Syncthing, Dropbox, etc.).

**Q: Is my data sent anywhere?**
A: No. biohack is 100% local-first. No network requests, no telemetry, no cloud sync.

**Q: Can I share protocols with others?**
A: Currently requires sharing source code. Future: protocol YAML export/import and registry.

**Q: How accurate are the safety protocols?**
A: Protocols are based on published guidelines (ACC/AHA, Hunter criteria) and harm-reduction principles. They are **not medical advice**. Always consult a healthcare provider.

**Q: Can I add my own substances?**
A: Yes, edit `data/seeds/substances.yaml` and run `biohack substance seed`. Or use `biohack substance seed --path custom.yaml`.

**Q: Does biohack support scheduled/recurring logs?**
A: Not yet. Planned: `biohack schedule add --cron "0 8 * * *" --name "Morning Stack"`.

**Q: Can I export data for my doctor?**
A: Not yet. Planned: `biohack export --format markdown|csv|json --days 30`.

---

*See also: [User Guide](user-guide.md), [Command Reference](command-reference.md), [Configuration](configuration.md)*