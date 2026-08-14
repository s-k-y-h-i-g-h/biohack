# biohack

A local-first, deterministic safety CLI for biohackers to track substances, medications, vitals, and food intake with built-in safety protocols.

[![Build Status](https://github.com/s-k-y-h-i-g-h/biohack/workflows/CI/badge.svg)](https://github.com/s-k-y-h-i-g-h/biohack/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2024-orange.svg)](https://www.rust-lang.org/)

## Overview

**biohack** is a command-line tool designed for biohackers, quantified-self enthusiasts, and anyone who tracks their supplement/medication regimen and health metrics. It provides:

- **Substance & medication logging** — Track what you take, when, how much, and via what route
- **Vitals logging** — Heart rate, blood pressure, temperature, SpO₂, HRV, weight
- **Food logging** — Individual food items with amounts and units (MVP)
- **Deterministic safety protocols** — Built-in rules that check for:
  - **Stimulant-associated tachycardia** (HR > 100 bpm + stimulant in last 4h)
  - **Hypertensive urgency** (SBP ≥ 180 or DBP ≥ 120)
  - **Serotonin syndrome risk** (multiple serotonergic agents)
- **Local-first, zero-config storage** — Pure-Rust `sled` embedded database, your data never leaves your machine
- **Curated substance database** — 27 substances with dose ranges, categories, half-lives, contraindications
- **Beautiful terminal output** — Colored tables, bold/underline, readable timestamps

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/s-k-y-h-i-g-h/biohack
cd biohack

# Build (requires Rust 2024+)
cargo build --release

# The binary will be at target/release/biohack
# Optionally add to PATH or copy to /usr/local/bin
```

### Initialize & Seed Database

```bash
# Initialize database and seed with 27 curated substances
biohack init
biohack substance seed

# List available substances
biohack substance list
```

### Daily Usage

```bash
# Log a supplement
biohack log substance --name "L-Theanine" --dose 400mg

# Log vitals
biohack log vitals --hr 88 --sbp 120 --dbp 80 --temp 37.0

# Log food (MVP)
biohack log food --name "Broccoli" --amount 2 --unit cups

# Run safety check
biohack check
```

### View Logs

```bash
# Show substances from last 3 days
biohack substance list

# Show recent vitals
biohack show vitals --days 3

# Show combined timeline
biohack show timeline --days 3
```

## Commands Reference

| Command | Description |
|---------|-------------|
| `biohack init` | Initialize database |
| `biohack log substance --name NAME --dose DOSE [--route ROUTE] [--time TIME] [--notes NOTES]` | Log substance intake |
| `biohack log vitals [--hr HR] [--sbp SBP] [--dbp DBP] [--temp TEMP] [--spo2 SPO2] [--hrv HRV] [--weight WEIGHT] [--time TIME] [--notes NOTES]` | Log vitals |
| `biohack log food --name NAME --amount AMOUNT [--unit UNIT] [--time TIME] [--notes NOTES]` | Log food item (MVP) |
| `biohack log stack --name NAME [--time TIME]` | Log predefined stack (stub) |
| `biohack substance list [--category CATEGORY]` | List substances in database |
| `biohack substance search QUERY` | Search substances |
| `biohack substance show NAME` | Show substance details (stub) |
| `biohack substance seed [--path PATH]` | Seed database from YAML |
| `biohack check` | Run safety protocols against recent logs |
| `biohack show substances [--days N] [--name NAME]` | Show recent substance logs |
| `biohack show vitals [--days N]` | Show recent vitals logs |
| `biohack show timeline [--days N]` | Show combined timeline |

## Safety Protocols

biohack includes three built-in deterministic safety protocols:

### 1. Stimulant-Associated Tachycardia
**Trigger:** Heart rate > 100 bpm AND stimulant logged in last 4 hours

**Actions (priority order):**
1. ��� Alert: "HR {{hr}}bpm with stimulant in last 4h — likely sympathetic overdrive"
2. ��� Cold face immersion (30s ice water or cold pack) — triggers mammalian dive reflex
3. ��� 500ml water + electrolytes — addresses relative hypovolemia
4. ��� Magnesium glycinate 400mg — NMDA modulation, vascular relaxation
5. ��� L-theanine 200-400mg — alpha-wave promotion, counters caffeine jitters
6. ��� No further stimulants for 6 hours

### 2. Hypertensive Urgency
**Trigger:** SBP ≥ 180 OR DBP ≥ 120 (without acute end-organ symptoms)

**Actions:**
1. ��� Alert: "BP {{sbp}}/{{dbp}} — hypertensive urgency range"
2. ��� Slow breathing: 6 breaths/min for 5 minutes (baroreflex)
3. ��� Hydrate: 500ml water over 30 min
4. ��� Avoid caffeine, nicotine, stimulants, NSAIDs
5. ��� Recheck BP in 30 minutes
6. ��� If chest pain, dyspnea, neuro symptoms, vision changes → seek emergency care

### 3. Serotonin Syndrome Risk
**Trigger:** Multiple serotonergic agents logged within 24h

**Actions:**
1. ��� Alert: "Multiple serotonergic agents detected — serotonin syndrome risk"
2. ��� Monitor for: clonus, hyperreflexia, hyperthermia, diaphoresis, agitation
3. ��� Do not add further serotonergic agents
4. ��� If symptoms develop: seek emergency care, discontinue serotonergic agents

## Substance Database

The seed database includes 27 curated substances across categories:

| Category | Examples |
|----------|----------|
| **Nootropic** | L-Theanine, Rhodiola, Ashwagandha, Modafinil, Armodafinil |
| **Stimulant** | Caffeine, Nicotine |
| **Supplement** | Magnesium Glycinate, Omega-3, Creatine, NAC, CoQ10, Zinc, NR, Resveratrol, Curcumin, Sulbutiamine, L-Tyrosine |
| **Vitamin** | Vitamin D3, B12, Folate |
| **Mineral** | Zinc |
| **Hormone** | Melatonin, Testosterone Enanthate, Estradiol Valerate |
| **Herb** | Rhodiola, Ashwagandha |
| **Medication** | Modafinil, Armodafinil |
| **Drug** | Nicotine, Alcohol, Psilocybin, LSD |

Each entry includes: dose ranges, typical dose, half-life, contraindications, interactions, and sources.

## Configuration

### Database Location

Default: `~/.local/share/biohack/biohack.db`

Override with environment variable:
```bash
export BIOHACK_DB=/path/to/custom.db
```

Or CLI flag:
```bash
biohack --db-path /path/to/custom.db substance list
```

### Disable Colors

```bash
biohack --no-color substance list
```

### Verbose Output

```bash
biohack -v substance list
```

## Architecture

- **Language**: Rust 2024 edition
- **Database**: `sled` (pure-Rust embedded key-value store, zero C dependencies)
- **CLI**: `clap 4` with structured subcommands
- **Tables**: `comfy-table` + `owo-colors`
- **Serialization**: `serde` + `serde_yaml`
- **Time**: `chrono`
- **Testing**: Unit tests (8 protocol engine tests passing)

## Development

### Run Tests

```bash
cargo test
```

### Run with Debug Logs

```bash
RUST_LOG=debug cargo run -- substance list
```

### Format & Lint

```bash
cargo fmt --check
cargo clippy
```

## Documentation

- [User Guide](docs/user-guide.md) — Installation, quick start, workflow
- [Command Reference](docs/command-reference.md) — All commands with examples
- [Protocol Authoring](docs/protocol-authoring.md) — YAML schema, built-in protocols, custom protocols
- [Configuration](docs/configuration.md) — Database path, config file, env vars
- [Troubleshooting](docs/troubleshooting.md) — Common issues, FAQ

## License

Licensed under either of:
- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

## Contributing

Contributions welcome! Please read the contributing guidelines before submitting PRs.

## Safety Disclaimer

**This tool is for informational and tracking purposes only. It does not provide medical advice.** The safety protocols are based on published guidelines and harm-reduction principles but are not a substitute for professional medical evaluation. Always consult a qualified healthcare provider for medical concerns.

---

*biohack — Your local-first safety net for biohacking*