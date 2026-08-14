# User Guide

Complete guide to using biohack for tracking substances, vitals, and food with safety protocols.

## Table of Contents

1. [Installation](#installation)
2. [Quick Start](#quick-start)
3. [Core Concepts](#core-concepts)
4. [Daily Workflow](#daily-workflow)
5. [Safety Protocols](#safety-protocols)
6. [Substance Database](#substance-database)
6. [Advanced Usage](#advanced-usage)
7. [Configuration](#configuration)
8. [Data Management](#data-management)

---

## Installation

### Prerequisites

- Rust 2024 edition (1.78+) — install via [rustup](https://rustup.rs/)
- Git

### From Source

```bash
# Clone the repository
git clone https://github.com/s-k-y-h-i-g-h/biohack
cd biohack

# Build release binary
cargo build --release

# Binary location
./target/release/biohack --help
```

### Install to PATH (Optional)

```bash
# Copy to a directory in your PATH
cp target/release/biohack ~/.local/bin/
# or
sudo cp target/release/biohack /usr/local/bin/
```

### Verify Installation

```bash
biohack --help
biohack --version
```

---

## Quick Start

```bash
# 1. Initialize database
biohack init

# 2. Seed with 27 curated substances
biohack substance seed

# 3. Log your morning stack
biohack log substance --name "Caffeine" --dose 100mg
biohack log substance --name "L-Theanine" --dose 200mg

# 4. Log morning vitals
biohack log vitals --hr 72 --sbp 118 --dbp 76 --temp 36.8

# 5. Run safety check
biohack check
```

Expected output:
```
���� Safety check: no protocols triggered
```

---

## Core Concepts

### Substances
Anything you ingest: supplements, medications, nootropics, drugs, herbs, hormones. Each has:
- **Name** — canonical name (e.g., "L-Theanine")
- **Dose** — amount with unit (e.g., "400mg", "2.5g", "10ml")
- **Route** — oral, sublingual, transdermal, injection, etc.
- **Time** — ISO 8601 timestamp (defaults to now)
- **Notes** — free text

### Vitals
Biometric measurements:
- **Heart Rate (HR)** — beats per minute
- **Systolic BP (SBP)** — mmHg
- **Diastolic BP (DBP)** — mmHg
- **Temperature** — Celsius
- **SpO₂** — percentage
- **HRV (RMSSD)** — milliseconds
- **Weight** — kilograms

### Food (MVP)
Individual food items:
- **Name** — food name (e.g., "Broccoli", "Salmon")
- **Amount** — numeric value
- **Unit** — g, mg, cups, slices, etc.
- **Time** — ISO 8601 (defaults to now)
- **Notes** — free text

### Stacks (Planned)
Predefined groups of substances (morning, evening, PRN). Not yet implemented.

### Safety Protocols
Deterministic rules that evaluate your recent logs and current vitals against known risk patterns. When triggered, they produce:
- **Alerts** — immediate attention needed
- **Suggestions** — evidence-based interventions
- **Constraints** — things to avoid

---

## Daily Workflow

### Morning
```bash
# Log your morning supplements
biohack log substance --name "Vitamin D3" --dose 2000IU
biohack log substance --name "Omega-3" --dose 2g

# Log morning vitals
biohack log vitals --hr 68 --sbp 122 --dbp 78 --temp 36.7
```

### Throughout the Day
```bash
# Log additional substances as you take them
biohack log substance --name "Caffeine" --dose 200mg --time 2024-01-15T10:30:00Z

# Log food
biohack log food --name "Salmon" --amount 150 --unit g

# Log vitals after exercise or if feeling unwell
biohack log vitals --hr 95 --sbp 135 --dbp 85
```

### Evening
```bash
# Log evening stack
biohack log substance --name "Magnesium Glycinate" --dose 400mg
biohack log substance --name "L-Theanine" --dose 200mg

# Evening vitals
biohack log vitals --hr 70 --sbp 118 --dbp 76 --temp 36.9

# Run safety check before bed
biohack check
```

### Weekly Review
```bash
# View last 7 days of substance logs
biohack show substances --days 7

# View last 7 days of vitals
biohack show vitals --days 7

# View combined timeline
biohack show timeline --days 7
```

---

## Safety Protocols

### How They Work

Protocols run deterministically against your recent logs. When you run `biohack check`, the engine:
1. Loads all 3 built-in protocols
2. Evaluates each against your recent substance logs and current vitals
3. Reports any triggered protocols with prioritized actions

### Running Checks

```bash
# Basic check
biohack check

# Check with verbose output
biohack -v check
```

### Understanding Output

**No protocols triggered:**
```
���� Safety check: no protocols triggered
```

**Protocol triggered:**
```
���� Safety check: 1 protocol(s) triggered

=== Stimulant-Associated Tachycardia ===
Status: TRIGGERED
Matched: vitals.heart_rate > 100, substance.recent.category contains stimulant
Actions (priority order):
  1. [ALERT] HR 110bpm with stimulant in last 4h — likely sympathetic overdrive
  2. [SUGGESTION] Cold face immersion (30s ice water or cold pack)
  3. [SUGGESTION] 500ml water + electrolytes
  4. [SUGGESTION] Magnesium glycinate 400mg
  5. [SUGGESTION] L-theanine 200-400mg
  6. [CONSTRAINT] No further stimulants for 6 hours
```

### Protocol Details

See [Protocol Authoring Guide](protocol-authoring.md) for YAML schema and customization.

---

## Substance Database

### Seeding

```bash
# Seed with built-in database (27 substances)
biohack substance seed

# Seed from custom YAML
biohack substance seed --path /path/to/custom.yaml
```

### Browsing

```bash
# List all substances
biohack substance list

# Filter by category
biohack substance list --category nootropic

# Search
biohack substance search magnesium

# Show details (stub)
biohack substance show "L-Theanine"
```

### Database Fields

Each substance entry includes:
- **id** — UUID
- **name** — canonical name
- **aliases** — alternative names
- **category** — nootropic, stimulant, supplement, vitamin, mineral, hormone, herb, medication, drug, peptide, electrolyte, other
- **min_dose_mg** / **max_dose_mg** — safety bounds
- **typical_dose_mg** — common dose
- **half_life_hours** — elimination half-life
- **contraindications** — conditions where use is unsafe
- **interactions** — known substance interactions
- **notes** — additional context
- **sources** — references (PubChem, Examine.com, FDA, etc.)

### Adding Custom Substances

```bash
# Not yet implemented via CLI
# Edit data/seeds/substances.yaml and re-seed
```

---

## Advanced Usage

### Custom Database Location

```bash
# Environment variable
export BIOHACK_DB=/data/biohack.db
biohack substance list

# CLI flag
biohack --db-path /data/biohack.db substance list
```

### Non-Interactive / Scripting

```bash
# Disable colors for logs
biohack --no-color substance list

# Verbose output
biohack -v log substance --name "Test" --dose 100mg
```

### Time Formats

All timestamps accept ISO 8601:
```bash
biohack log substance --name "Test" --dose 100mg --time "2024-01-15T10:30:00Z"
biohack log vitals --hr 80 --time "2024-01-15T10:30:00+00:00"
biohack log substance --name "Test" --dose 100mg --time "2024-01-15"
```

### Dose Formats

```bash
# Milligrams
biohack log substance --name "Test" --dose 400mg

# Grams
biohack log substance --name "Test" --dose 2.5g

# Milliliters
biohack log substance --name "Test" --dose 10ml

# Bare number (assumes mg)
biohack log substance --name "Test" --dose 400
```

---

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `BIOHACK_DB` | Database file path | `~/.local/share/biohack/biohack.db` |

### CLI Flags (Global)

| Flag | Description |
|------|-------------|
| `--db-path PATH` | Override database location |
| `--no-color` | Disable colored output |
| `-v, --verbose` | Enable verbose output |
| `-h, --help` | Show help |

### Configuration File (Planned)

Future versions will support `~/.config/biohack/config.toml`:
```toml
[database]
path = "~/.local/share/biohack/biohack.db"

[output]
color = true
format = "table"

[safety]
check_on_log = false
```

---

## Data Management

### Database Location

Default: `~/.local/share/biohack/biohack.db`

### Backup

```bash
# Simple file copy (database is a single file)
cp ~/.local/share/biohack/biohack.db ~/biohack-backup-$(date +%Y%m%d).db
```

### Export (Planned)

Future versions will support:
```bash
biohack export --format json --output backup.json
biohack export --format csv --output backup.csv
biohack export --format markdown --output report.md
```

### Migration

The database uses `sled` which handles migrations automatically. For protocol YAML changes, see [Protocol Authoring Guide](protocol-authoring.md).

### Reset Database

```bash
# Remove database file to start fresh
rm ~/.local/share/biohack/biohack.db
biohack init
biohack substance seed
```

---

## Next Steps

- Read the [Command Reference](command-reference.md) for complete command details
- Learn to author custom protocols in [Protocol Authoring](protocol-authoring.md)
- Configure advanced settings in [Configuration](configuration.md)
- Troubleshoot issues in [Troubleshooting](troubleshooting.md)

---

*biohack — Your local-first safety net for biohacking*