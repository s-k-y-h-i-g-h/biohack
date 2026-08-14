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
| `--dose` | `-d` | Yes | Dose (e.g., "400mg", "2.5g", "10ml") |
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

Log a predefined stack (stub - not yet implemented).

```bash
biohack log stack --name NAME [--time TIME]
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

Manage stacks (stub - not yet implemented).

```bash
biohack stack list
biohack stack show NAME
biohack stack create PATH
```

---

### `biohack protocol`

Protocol commands (stub - not yet implemented).

```bash
biohack protocol list
biohack protocol test --id ID
biohack protocol show ID
```

---

### `biohack report`

Generate reports (stub - not yet implemented).

```bash
biohack report [--days DAYS] [--format FORMAT] [--output PATH]
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