# Command Reference

Complete reference for all biohack commands with examples.

## Global Options

These flags work with any command:

| Flag | Short | Description |
|------|-------|-------------|
| `--db-path PATH` | `-d PATH` | Database file path (default: `~/.local/share/biohack/biohack.db`) |
| `--no-color` | | Disable colored output |
| `-v`, `--verbose` | | Enable verbose output |
| `-h`, `--help` | | Show help for command |
| `-V`, `--version` | | Show version |

## Commands

### `biohack init`

Initialize the database.

```bash
biohack init
```

**Output:**
```
[OK] Database initialized
```

Creates the database file and tables if they don't exist. Safe to run multiple times.

---

### `biohack log`

Log substance intake, vitals, food, or stack.

#### `biohack log substance`

Log a substance intake.

```bash
biohack log substance --name NAME --dose DOSE [--route ROUTE] [--time TIME] [--notes NOTES]
```

**Options:**
| Option | Short | Required | Description |
|--------|-------|----------|-------------|
| `--name` | `-n` | Yes | Substance name (fuzzy-matched against database) |
| `--dose` | `-d` | Yes | Dose (e.g., "400mg", "2.5g", "10ml", "5000IU") |
| `--route` | `-r` | No | Route of administration (default: "oral") |
| `--time` | `-t` | No | Timestamp ISO 8601 (default: now) |
| `--notes` | | No | Additional notes |

**Examples:**
```bash
# Basic usage
biohack log substance --name "L-Theanine" --dose 400mg

# With route and time
biohack log substance --name "Caffeine" --dose 100mg --route sublingual --time "2024-01-15T10:30:00Z"

# With notes
biohack log substance --name "Magnesium Glycinate" --dose 400mg --notes "Before bed"

# International Units
biohack log substance --name "Vitamin D3" --dose 5000IU
```

**Output:**
```
[OK] Logged substance: L-Theanine 400mg oral at 2024-01-15 10:30
  Notes: Before bed
```

#### `biohack log vitals`

Log vital signs.

```bash
biohack log vitals [--hr HR] [--sbp SBP] [--dbp DBP] [--temp TEMP] [--spo2 SPO2] [--hrv HRV] [--weight WEIGHT] [--time TIME] [--notes NOTES]
```

**Options:**
| Option | Short | Required | Description |
|--------|-------|----------|-------------|
| `--hr` | | No | Heart rate (bpm) |
| `--sbp` | | No | Systolic blood pressure (mmHg) |
| `--dbp` | | No | Diastolic blood pressure (mmHg) |
| `--temp` | | No | Temperature (Celsius) |
| `--spo2` | | No | SpO2 (%) |
| `--hrv` | | No | HRV RMSSD (ms) |
| `--weight` | | No | Weight (kg) |
| `--time` | `-t` | No | Timestamp ISO 8601 (default: now) |
| `--notes` | | No | Additional notes |

**Examples:**
```bash
# Full vitals
biohack log vitals --hr 72 --sbp 120 --dbp 80 --temp 36.8 --spo2 98 --hrv 45 --weight 75.5

# Just heart rate and BP
biohack log vitals --hr 88 --sbp 135 --dbp 85

# With time
biohack log vitals --hr 95 --time "2024-01-15T14:30:00Z"
```

**Output:**
```
[OK] Logged vitals: HR=88 SBP=135 DBP=85 Temp=-C SpO2=-% HRV=-ms Weight=-kg at 2024-01-15 14:30
```

#### `biohack log food`

Log individual food item (MVP).

```bash
biohack log food --name NAME --amount AMOUNT [--unit UNIT] [--time TIME] [--notes NOTES]
```

**Options:**
| Option | Short | Required | Description |
|--------|-------|----------|-------------|
| `--name` | `-n` | Yes | Food name |
| `--amount` | `-a` | Yes | Amount consumed |
| `--unit` | `-u` | No | Unit (default: "g") |
| `--time` | `-t` | No | Timestamp ISO 8601 (default: now) |
| `--notes` | | No | Additional notes |

**Examples:**
```bash
biohack log food --name "Broccoli" --amount 2 --unit cups
biohack log food --name "Salmon" --amount 150 --unit g --notes "Wild caught"
```

**Output:**
```
[OK] Logged food: 2 cups Broccoli at 2024-01-15 12:00
  Notes: Wild caught
```

#### `biohack log stack`

Log a predefined stack.

```bash
biohack log stack --name NAME [--time TIME]
```

**Options:**
| Option | Short | Required | Description |
|--------|-------|----------|-------------|
| `--name` | | Yes | Stack name |
| `--time` | `-t` | No | Timestamp ISO 8601 (default: now) |

**Examples:**
```bash
biohack log stack --name "Morning Stack"
biohack log stack --name "Evening Stack" --time "2024-01-15T20:00:00Z"
```

**Output:**
```
[OK] Logged stack 'Morning Stack': 3 items at 2024-01-15 08:00
  [OK] L-Theanine 200mg oral
  [OK] Vitamin D3 5000IU oral
  [OK] Omega-3 Fish Oil 2g oral
```

---

### `biohack substance`

Manage substance database.

#### `biohack substance list`

List all substances in database.

```bash
biohack substance list [--category CATEGORY]
```

**Options:**
| Option | Short | Required | Description |
|--------|-------|----------|-------------|
| `--category` | `-c` | No | Filter by category |

**Examples:**
```bash
biohack substance list
biohack substance list --category nootropic
biohack substance list --category stimulant
```

**Output:**
```
+----------------------+------------+----------------+-----------+------------------------------------------------------------------+
| Name                 | Category   | Typical Dose   | Half-life | Contraindications                                                |
+----------------------+------------+----------------+-----------+------------------------------------------------------------------+
| L-Theanine           | nootropic  | 200mg          | 1.2h      | low blood pressure                                               |
+----------------------+------------+----------------+-----------+------------------------------------------------------------------+
| Caffeine             | stimulant  | 100mg          | 5.0h      | severe anxiety, arrhythmia, uncontrolled hypertension            |
+----------------------+------------+----------------+-----------+------------------------------------------------------------------+
```

#### `biohack substance search`

Search substances by name.

```bash
biohack substance search QUERY
```

**Example:**
```bash
biohack substance search magnesium
```

#### `biohack substance show`

Show substance details (stub).

```bash
biohack substance show NAME
```

#### `biohack substance seed`

Seed database from YAML file.

```bash
biohack substance seed [--path PATH]
```

**Options:**
| Option | Short | Required | Default | Description |
|--------|-------|----------|---------|-------------|
| `--path` | `-p` | No | `data/seeds/substances.yaml` | Path to YAML seed file |

**Examples:**
```bash
# Built-in seed
biohack substance seed

# Custom seed file
biohack substance seed --path /path/to/custom.yaml
```

**Output:**
```
[OK] Seeded: L-Theanine
[OK] Seeded: Caffeine
...
[OK] Seeded 27 substances
```

---

### `biohack show`

View recent logs.

#### `biohack show substances`

Show recent substance logs.

```bash
biohack show substances [--days DAYS] [--name NAME]
```

**Options:**
| Option | Short | Required | Default | Description |
|--------|-------|----------|---------|-------------|
| `--days` | `-d` | No | 3 | Days to look back |
| `--name` | `-n` | No | | Filter by substance name |

**Examples:**
```bash
biohack show substances
biohack show substances --days 7
biohack show substances --name caffeine
```

#### `biohack show vitals`

Show recent vitals logs.

```bash
biohack show vitals [--days DAYS]
```

**Options:**
| Option | Short | Required | Default | Description |
|--------|-------|----------|---------|-------------|
| `--days` | `-d` | No | 3 | Days to look back |

**Example:**
```bash
biohack show vitals --days 7
```

#### `biohack show timeline`

Show combined timeline of all logs.

```bash
biohack show timeline [--days DAYS]
```

**Options:**
| Option | Short | Required | Default | Description |
|--------|-------|----------|---------|-------------|
| `--days` | `-d` | No | 3 | Days to look back |

**Example:**
```bash
biohack show timeline --days 7
```

---

### `biohack check`

Run safety protocols against recent logs.

```bash
biohack check
```

**Output (no protocols triggered):**
```
[OK] Safety check: no protocols triggered
```

**Output (protocol triggered):**
```
[ALERT] Safety check: 1 protocol(s) triggered

=== Stimulant-Associated Tachycardia ===
Status: TRIGGERED
Matched: vitals.heart_rate > 100, substance.recent.category contains stimulant
Actions (priority order):
  1. [ALERT] HR 110bpm with stimulant in last 4h -- likely sympathetic overdrive
  2. [SUGGESTION] Cold face immersion (30s ice water or cold pack)
  3. [SUGGESTION] 500ml water + electrolytes
  4. [SUGGESTION] Magnesium glycinate 400mg
  5. [SUGGESTION] L-theanine 200-400mg
  6. [CONSTRAINT] No further stimulants for 6 hours
```

---

### `biohack stack`

Manage stacks.

```bash
biohack stack list
biohack stack show NAME
biohack stack create PATH
```

**Examples:**
```bash
# List all stacks
biohack stack list

# Show stack details
biohack stack show "Morning Stack"

# Create stack from YAML file
biohack stack create morning-stack.yaml
```

**YAML Format:**
```yaml
name: "Morning Stack"
description: "Daily morning supplement stack"
items:
  - substance_name: "L-Theanine"
    dose: "200mg"
    route: "oral"
    schedule: "morning"
  - substance_name: "Vitamin D3"
    dose: "5000IU"
    route: "oral"
    schedule: "morning"
```

**Schedules:** `morning`, `evening`, `prn` (as needed)

---

### `biohack protocol`

Protocol commands.

```bash
biohack protocol list
biohack protocol test ID
biohack protocol show ID
```

**Examples:**
```bash
# List all available protocols
biohack protocol list

# Test a specific protocol with current data
biohack protocol test stimulant_tachycardia

# Test with verbose output
biohack -v protocol test hypertension_urgency
```

**Output (protocol list):**
```
[PROTOCOL] Stimulant-Associated Tachycardia (stimulant_tachycardia)
  Triggered when heart rate > 100 bpm with stimulant use in last 4 hours
  Version: 1.0
  Actions: 6

[PROTOCOL] Hypertensive Urgency (hypertension_urgency)
  Triggered when SBP >= 180 or DBP >= 120 without acute end-organ symptoms
  Version: 1.0
  Actions: 6

[PROTOCOL] Serotonin Syndrome Risk (serotonin_syndrome_risk)
  Triggered when multiple serotonergic agents logged within 24h
  Version: 1.0
  Actions: 4
```

**Output (protocol test):**
```
[TEST] Testing protocol: Stimulant-Associated Tachycardia (stimulant_tachycardia)
Description: Triggered when heart rate > 100 bpm with stimulant use in last 4 hours

Triggered: NO

Matched conditions:
  - substance.recent.category contains "stimulant"
```

---

### `biohack report`

Generate reports.

```bash
biohack report [--days DAYS] [--format FORMAT] [--output PATH]
```

**Options:**
| Option | Short | Required | Default | Description |
|--------|-------|----------|---------|-------------|
| `--days` | `-d` | No | 7 | Days to include |
| `--format` | `-f` | No | markdown | Output format: markdown, csv |
| `--output` | `-o` | No | stdout | Output file path |

**Examples:**
```bash
# Markdown report (default)
biohack report --days 7

# CSV report to file
biohack report --days 30 --format csv --output monthly-report.csv

# Markdown to file
biohack report --days 14 --format markdown --output two-week-report.md
```

**Output (markdown):**
```markdown
# Biohack Health Report

**Generated:** 2024-01-15 10:30 UTC
**Period:** 2024-01-08 to 2024-01-15 (7 days)

## Summary
- **Substance Logs:** 42
- **Unique Substances:** 8
- **Vitals Logs:** 14
- **Food Logs:** 21
- **Defined Stacks:** 2

## Substance Intake Log
| Date & Time | Substance | Dose | Route | Category | Notes |
|-------------|-----------|------|-------|----------|-------|
| 2024-01-15 08:00 | L-Theanine | 200mg | oral | nootropic | — |

## Substance Frequency
| Substance | Log Count |
|-----------|-----------|
| L-Theanine | 14 |
| Caffeine | 10 |
```

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | Database error |
| 4 | Not found |
| 5 | Validation error |

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `BIOHACK_DB` | Database path | `~/.local/share/biohack/biohack.db` |
| `RUST_LOG` | Logging level (trace, debug, info, warn, error) | `info` |

---

## See Also

- [User Guide](user-guide.md) — Workflow and concepts
- [Protocol Authoring](protocol-authoring.md) — Custom protocols
- [Configuration](configuration.md) — Advanced config
- [Troubleshooting](troubleshooting.md) — Common issues