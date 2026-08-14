-- biohack database schema

-- Substances reference database
CREATE TABLE IF NOT EXISTS substances (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    aliases TEXT NOT NULL DEFAULT '[]',
    category TEXT NOT NULL,
    min_dose_mg REAL,
    max_dose_mg REAL,
    typical_dose_mg REAL,
    half_life_hours REAL,
    contraindications TEXT NOT NULL DEFAULT '[]',
    interactions TEXT NOT NULL DEFAULT '[]',
    notes TEXT,
    sources TEXT NOT NULL DEFAULT '[]',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_substances_category ON substances(category);
CREATE INDEX IF NOT EXISTS idx_substances_name ON substances(name);

-- Substance intake logs
CREATE TABLE IF NOT EXISTS substance_logs (
    id TEXT PRIMARY KEY,
    substance_id TEXT NOT NULL,
    substance_name TEXT NOT NULL,
    dose_mg REAL NOT NULL,
    route TEXT NOT NULL DEFAULT 'oral',
    timestamp TEXT NOT NULL,
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (substance_id) REFERENCES substances(id)
);

CREATE INDEX IF NOT EXISTS idx_substance_logs_timestamp ON substance_logs(timestamp);
CREATE INDEX IF NOT EXISTS idx_substance_logs_substance_id ON substance_logs(substance_id);

-- Vitals logs
CREATE TABLE IF NOT EXISTS vitals_logs (
    id TEXT PRIMARY KEY,
    heart_rate INTEGER,
    sbp INTEGER,
    dbp INTEGER,
    temperature_c REAL,
    spo2 INTEGER,
    hrv_rmssd INTEGER,
    weight_kg REAL,
    timestamp TEXT NOT NULL,
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_vitals_logs_timestamp ON vitals_logs(timestamp);

-- Stacks
CREATE TABLE IF NOT EXISTS stacks (
    name TEXT PRIMARY KEY,
    description TEXT,
    items TEXT NOT NULL DEFAULT '[]',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Protocols (stored as YAML/JSON)
CREATE TABLE IF NOT EXISTS protocols (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    definition TEXT NOT NULL,
    version TEXT NOT NULL DEFAULT '1.0',
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);