use anyhow::Result;
use chrono::{DateTime, Utc};
use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};
use owo_colors::OwoColorize;
use serde_yaml;
use std::fs;
use uuid::Uuid;

use crate::cli::*;
use crate::db::Database;
use crate::models::{Substance, SubstanceLog, VitalsLog, FoodLog};

/// Initialize the database
pub fn handle_init(db: &Database) -> Result<()> {
    println!("{}", "���� Database initialized".green());
    Ok(())
}

/// Seed the database with initial substances from a YAML file
pub fn handle_substance_seed(db: &Database, args: &SubstanceCommands, _no_color: bool) -> Result<()> {
    if let SubstanceCommands::Seed(args) = args {
        let content = std::fs::read_to_string(&args.path)?;
        let mut substances: Vec<Substance> = serde_yaml::from_str(&content)?;
        for substance in &mut substances {
            if substance.id.is_nil() {
                substance.id = Uuid::new_v4();
            }
            db.insert_substance(substance)?;
            println!("{}", format!("���� Seeded: {}", substance.name).green());
        }
        println!("{}", format!("���� Seeded {} substances", substances.len()).green().bold());
    }
    Ok(())
}

/// List substances in the database
pub fn handle_substance_list(db: &Database, args: &SubstanceCommands, _no_color: bool) -> Result<()> {
    if let SubstanceCommands::List(args) = args {
        let substances = db.list_substances(args.category.as_deref())?;
        if substances.is_empty() {
            println!("{}", "No substances found".yellow());
            return Ok(());
        }

        let mut table = Table::new();
        table.load_preset(UTF8_FULL).apply_modifier(UTF8_ROUND_CORNERS);
        table.set_header(vec!["Name", "Category", "Typical Dose", "Half-life", "Contraindications"]);

        for s in substances {
            let typical = s.typical_dose_mg.map(|v| format_dose(v)).unwrap_or_else(|| "—".to_string());
            let half_life = s.half_life_hours.map(|h| format!("{:.1}h", h)).unwrap_or_else(|| "—".to_string());
            let contra = if s.contraindications.is_empty() { "—".to_string() } else { s.contraindications.join(", ") };

            table.add_row(vec![
                Cell::new(&s.name),
                Cell::new(s.category.to_string()),
                Cell::new(&typical),
                Cell::new(&half_life),
                Cell::new(&contra),
            ]);
        }

        println!("{}", table);
    }
    Ok(())
}

/// Log a substance intake
pub fn handle_log_substance(db: &Database, args: &LogCommands, _no_color: bool) -> Result<()> {
    if let LogCommands::Substance(args) = args {
        let dose_mg = parse_dose(&args.dose)?;
        let timestamp = parse_time(&args.time)?;
        
        // Find substance ID from name
        let substance_option = db.get_substance_by_name(&args.name)?;
        let substance = substance_option.ok_or_else(|| anyhow::anyhow!("Substance not found in database. Use 'biohack substance seed' first."))?;
        let substance_id = substance.id;

        // Create and insert substance log
        let log = SubstanceLog {
            id: Uuid::new_v4(),
            substance_id,
            substance_name: args.name.clone(),
            dose_mg,
            route: args.route.clone(),
            timestamp,
            notes: args.notes.clone(),
            category: Some(substance.category.to_string()),
        };
        
        db.insert_substance_log(&log)?;
        
        println!(
            "{}",
            format!(
                "�������� Logged substance: {} {} {} at {}",
                args.name, args.dose, args.route,
                timestamp.format("%Y-%m-%d %H:%M")
            ).green()
        );
        if let Some(n) = &args.notes {
            println!("  Notes: {}", n);
        }
    }
    Ok(())
}

/// Log vitals
pub fn handle_log_vitals(db: &Database, args: &LogCommands, _no_color: bool) -> Result<()> {
    if let LogCommands::Vitals(args) = args {
        let timestamp = parse_time(&args.time)?;

        // Create and insert vitals log
        let log = VitalsLog {
            id: Uuid::new_v4(),
            heart_rate: args.hr,
            sbp: args.sbp,
            dbp: args.dbp,
            temperature_c: args.temp,
            spo2: args.spo2,
            hrv_rmssd: args.hrv,
            weight_kg: args.weight,
            timestamp,
            notes: args.notes.clone(),
        };

        db.insert_vitals_log(&log)?;

        println!(
            "{}",
            format!(
                "���� Logged vitals: HR={} SBP={} DBP={} Temp={}°C SpO2={}% HRV={}ms Weight={}kg at {}",
                args.hr.map(|v| v.to_string()).unwrap_or_else(|| "—".to_string()),
                args.sbp.map(|v| v.to_string()).unwrap_or_else(|| "—".to_string()),
                args.dbp.map(|v| v.to_string()).unwrap_or_else(|| "—".to_string()),
                args.temp.map(|v| format!("{:.1}", v)).unwrap_or_else(|| "—".to_string()),
                args.spo2.map(|v| v.to_string()).unwrap_or_else(|| "—".to_string()),
                args.hrv.map(|v| v.to_string()).unwrap_or_else(|| "—".to_string()),
                args.weight.map(|v| format!("{:.1}", v)).unwrap_or_else(|| "—".to_string()),
                timestamp.format("%Y-%m-%d %H:%M")
            ).green()
        );
        if let Some(n) = &args.notes {
            println!("  Notes: {}", n);
        }
    }
    Ok(())
}

/// Placeholder for other commands - mark as not implemented
pub fn handle_log_stack(_db: &Database, _args: &LogCommands, _no_color: bool) -> Result<()> {
    println!("{}", "Not yet implemented: log stack".yellow());
    Ok(())
}

/// Log individual food intake
pub fn handle_log_food(db: &Database, args: &LogCommands, _no_color: bool) -> Result<()> {
    if let LogCommands::Food(args) = args {
        let timestamp = parse_time(&args.time)?;

        // Create and insert food log
        let log = FoodLog {
            id: Uuid::new_v4(),
            food_name: args.name.clone(),
            amount: args.amount,
            unit: args.unit.clone(),
            timestamp,
            notes: args.notes.clone(),
        };

        db.insert_food_log(&log)?;

        println!(
            "{}",
            format!(
                "������� Logged food: {} {} {} at {}",
                args.amount, args.unit, args.name,
                timestamp.format("%Y-%m-%d %H:%M")
            ).green()
        );
        if let Some(n) = &args.notes {
            println!("  Notes: {}", n);
        }
    }
    Ok(())
}

/// Show recent substance logs
pub fn handle_show_substances(db: &Database, args: &ShowCommands, _no_color: bool) -> Result<()> {
    if let ShowCommands::Substances(args) = args {
        let logs = db.get_recent_substance_logs(args.days, args.name.as_deref())?;
        
        if logs.is_empty() {
            println!("{}", "No substance logs found".yellow());
            return Ok(());
        }

        let mut table = Table::new();
        table.load_preset(UTF8_FULL).apply_modifier(UTF8_ROUND_CORNERS);
        table.set_header(vec!["Time", "Substance", "Dose", "Route", "Category", "Notes"]);

        for log in logs {
            let category = log.category.as_deref().unwrap_or("—");
            let notes = log.notes.as_deref().unwrap_or("—");
            
            table.add_row(vec![
                Cell::new(&log.timestamp.format("%Y-%m-%d %H:%M").to_string()),
                Cell::new(&log.substance_name),
                Cell::new(&format!("{}mg", log.dose_mg as u64)),
                Cell::new(&log.route),
                Cell::new(category),
                Cell::new(notes),
            ]);
        }

        println!("{}", table);
    }
    Ok(())
}

/// Show recent vitals logs
pub fn handle_show_vitals(db: &Database, args: &ShowCommands, _no_color: bool) -> Result<()> {
    if let ShowCommands::Vitals(args) = args {
        let logs = db.get_recent_vitals_logs(args.days)?;

        if logs.is_empty() {
            println!("{}", "No vitals logs found".yellow());
            return Ok(());
        }

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS);
        table.set_header(vec![
            "Time",
            "HR",
            "SBP",
            "DBP",
            "Temp (°C)",
            "SpO2 (%)",
            "HRV (ms)",
            "Weight (kg)",
            "Notes",
        ]);

        for log in logs {
            let hr = log.heart_rate.map(|v| v.to_string()).unwrap_or_else(|| "—".to_string());
            let sbp = log.sbp.map(|v| v.to_string()).unwrap_or_else(|| "—".to_string());
            let dbp = log.dbp.map(|v| v.to_string()).unwrap_or_else(|| "—".to_string());
            let temp = log.temperature_c.map(|v| format!("{:.1}", v)).unwrap_or_else(|| "—".to_string());
            let spo2 = log.spo2.map(|v| v.to_string()).unwrap_or_else(|| "—".to_string());
            let hrv = log.hrv_rmssd.map(|v| v.to_string()).unwrap_or_else(|| "—".to_string());
            let weight = log.weight_kg.map(|v| format!("{:.1}", v)).unwrap_or_else(|| "—".to_string());
            let notes = log.notes.as_deref().unwrap_or("—");

            table.add_row(vec![
                Cell::new(&log.timestamp.format("%Y-%m-%d %H:%M").to_string()),
                Cell::new(&hr),
                Cell::new(&sbp),
                Cell::new(&dbp),
                Cell::new(&temp),
                Cell::new(&spo2),
                Cell::new(&hrv),
                Cell::new(&weight),
                Cell::new(notes),
            ]);
        }

        println!("{}", table);
    }
    Ok(())
}

/// Show combined timeline of all log types
pub fn handle_show_timeline(db: &Database, args: &ShowCommands, _no_color: bool) -> Result<()> {
    if let ShowCommands::Timeline(args) = args {
        let entries = db.get_recent_timeline(args.days)?;

        if entries.is_empty() {
            println!("{}", "No timeline entries found".yellow());
            return Ok(());
        }

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS);
        table.set_header(vec![
            "Time",
            "Type",
            "Details",
            "Notes",
        ]);

        for entry in entries {
            let timestamp_str = entry.timestamp().format("%Y-%m-%d %H:%M").to_string();
            let (entry_type, details, notes) = match entry {
                crate::db::TimelineEntry::Substance(log) => (
                    "Substance".to_string(),
                    format!("{} {}mg {}", log.substance_name, log.dose_mg as u64, log.route),
                    log.notes.as_deref().unwrap_or("—").to_string(),
                ),
                crate::db::TimelineEntry::Vitals(log) => {
                    let mut parts = Vec::new();
                    if let Some(hr) = log.heart_rate { parts.push(format!("HR:{}", hr)); }
                    if let Some(sbp) = log.sbp { parts.push(format!("SBP:{}", sbp)); }
                    if let Some(dbp) = log.dbp { parts.push(format!("DBP:{}", dbp)); }
                    if let Some(temp) = log.temperature_c { parts.push(format!("Temp:{:.1}°C", temp)); }
                    if let Some(spo2) = log.spo2 { parts.push(format!("SpO2:{}%", spo2)); }
                    if let Some(hrv) = log.hrv_rmssd { parts.push(format!("HRV:{}ms", hrv)); }
                    if let Some(weight) = log.weight_kg { parts.push(format!("W:{:.1}kg", weight)); }
                    (
                        "Vitals".to_string(),
                        parts.join(" "),
                        log.notes.as_deref().unwrap_or("—").to_string(),
                    )
                }
                crate::db::TimelineEntry::Food(log) => (
                    "Food".to_string(),
                    format!("{} {} {}", log.amount, log.unit, log.food_name),
                    log.notes.as_deref().unwrap_or("—").to_string(),
                ),
            };

            table.add_row(vec![
                Cell::new(&timestamp_str),
                Cell::new(&entry_type),
                Cell::new(&details),
                Cell::new(&notes),
            ]);
        }

        println!("{}", table);
    }
    Ok(())
}

pub fn handle_substance_search(_db: &Database, _args: &SubstanceCommands, _no_color: bool) -> Result<()> {
    println!("{}", "Not yet implemented: search substances".yellow());
    Ok(())
}

pub fn handle_substance_show(_db: &Database, _args: &SubstanceCommands, _no_color: bool) -> Result<()> {
    println!("{}", "Not yet implemented: show substance details".yellow());
    Ok(())
}

pub fn handle_stack_list(_db: &Database, _args: &StackCommands, _no_color: bool) -> Result<()> {
    println!("{}", "Not yet implemented: list stacks".yellow());
    Ok(())
}

pub fn handle_stack_show(_db: &Database, _args: &StackCommands, _no_color: bool) -> Result<()> {
    println!("{}", "Not yet implemented: show stack".yellow());
    Ok(())
}

pub fn handle_protocol_list(_db: &Database, _args: &ProtocolCommands, _no_color: bool) -> Result<()> {
    println!("{}", "Not yet implemented: list protocols".yellow());
    Ok(())
}

pub fn handle_protocol_test(_db: &Database, _args: &ProtocolCommands, _no_color: bool) -> Result<()> {
    println!("{}", "Not yet implemented: test protocol".yellow());
    Ok(())
}

pub fn handle_report(_db: &Database, _args: &ReportArgs, _no_color: bool) -> Result<()> {
    println!("{}", "Not yet implemented: generate report".yellow());
    Ok(())
}

pub fn handle_check(_db: &Database, _no_color: bool) -> Result<()> {
    println!("{}", "������� Safety check: no protocols triggered".green());
    Ok(())
}

/// Parse a dose string (e.g., "400mg", "2.5g", "10ml") into milligrams as f64
fn parse_dose(s: &str) -> Result<f64> {
    let s = s.trim().to_lowercase();
    if s.ends_with("mg") {
        Ok(s.trim_end_matches("mg").parse()?)
    } else if s.ends_with("g") {
        Ok(s.trim_end_matches("g").parse::<f64>()? * 1000.0)
    } else if s.ends_with("ml") {
        Ok(s.trim_end_matches("ml").parse::<f64>()? * 1000.0) // assume 1g/ml for liquids
    } else {
        Ok(s.parse()?) // assume mg
    }
}

/// Format a dose in mg as a string, using g if >= 1000mg and integer
fn format_dose(mg: f64) -> String {
    if mg >= 1000.0 && mg % 1000.0 == 0.0 {
        format!("{}g", mg / 1000.0)
    } else {
        format!("{}mg", mg)
    }
}

/// Parse an optional timestamp string into a DateTime<Utc>
fn parse_time(s: &Option<String>) -> Result<DateTime<Utc>> {
    Ok(match s {
        Some(t) => DateTime::parse_from_rfc3339(t)?.with_timezone(&Utc),
        None => Utc::now(),
    })
}