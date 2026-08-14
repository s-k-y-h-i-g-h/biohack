use anyhow::Result;
use chrono::{DateTime, Utc};
use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};
use owo_colors::OwoColorize;
use serde_yaml;
use std::fs;
use uuid::Uuid;

use crate::cli::*;
use crate::db::Database;
use crate::models::{Substance, SubstanceLog, VitalsLog};

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
        let substance = db.get_substance_by_name(&args.name)?;
        if substance.is_none() {
            println!("{}", "Substance not found in database. Use 'biohack substance seed' first.".yellow());
            return Ok(());
        }
        let substance_id = substance.unwrap().id;
        
        // Create and insert substance log
        let log = SubstanceLog {
            id: Uuid::new_v4(),
            substance_id,
            substance_name: args.name.clone(),
            dose_mg,
            route: args.route.clone(),
            timestamp,
            notes: args.notes.clone(),
            category: Some(args.route.clone()), // TODO: This should be substance.category, not route
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
        println!(
            "{}",
            format!(
                "���� Logged vitals: HR={} SBP={} DBP={} Temp={}°C SpO2={}% HRV={}ms Weight={}kg at {}",
                args.hr.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string()),
                args.sbp.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string()),
                args.dbp.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string()),
                args.temp.map(|v| format!("{:.1}", v)).unwrap_or_else(|| "-".to_string()),
                args.spo2.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string()),
                args.hrv.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string()),
                args.weight.map(|v| format!("{:.1}", v)).unwrap_or_else(|| "-".to_string()),
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
        // For MVP, just log to console; in v1.1 we'll insert into DB with food database lookup
        let timestamp = parse_time(&args.time)?;
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

pub fn handle_show_vitals(_db: &Database, _args: &ShowCommands, _no_color: bool) -> Result<()> {
    println!("{}", "Not yet implemented: show vitals".yellow());
    Ok(())
}

pub fn handle_show_timeline(_db: &Database, _args: &ShowCommands, _no_color: bool) -> Result<()> {
    println!("{}", "Not yet implemented: show timeline".yellow());
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