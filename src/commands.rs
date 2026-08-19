use anyhow::Result;
use chrono::{DateTime, Utc};
use comfy_table::{Cell, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};
use owo_colors::OwoColorize;
use std::io::Write;
use uuid::Uuid;

use crate::cli::*;
use crate::db::Database;
use crate::models::{FoodLog, Stack, Substance, SubstanceLog, VitalsLog};
use crate::protocols::{ProtocolContext, ProtocolEngine};

/// Initialize the database
pub fn handle_init(_db: &Database) -> Result<()> {
    println!("{}", "���� Database initialized".green());
    Ok(())
}

/// Seed the database with initial substances from a YAML file
pub fn handle_substance_seed(
    db: &Database,
    args: &SubstanceCommands,
    _no_color: bool,
) -> Result<()> {
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
        println!(
            "{}",
            format!("���� Seeded {} substances", substances.len())
                .green()
                .bold()
        );
    }
    Ok(())
}

/// List substances in the database
pub fn handle_substance_list(
    db: &Database,
    args: &SubstanceCommands,
    _no_color: bool,
) -> Result<()> {
    if let SubstanceCommands::List(args) = args {
        let substances = db.list_substances(args.category.as_deref())?;
        if substances.is_empty() {
            println!("{}", "No substances found".yellow());
            return Ok(());
        }

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS);
        table.set_header(vec![
            "Name",
            "Category",
            "Typical Dose",
            "Half-life",
            "Contraindications",
        ]);

        for s in substances {
            let typical = s
                .typical_dose_mg
                .map(format_dose)
                .unwrap_or_else(|| "—".to_string());
            let half_life = s
                .half_life_hours
                .map(|h| format!("{:.1}h", h))
                .unwrap_or_else(|| "—".to_string());
            let contra = if s.contraindications.is_empty() {
                "—".to_string()
            } else {
                s.contraindications.join(", ")
            };

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
        let substance = substance_option.ok_or_else(|| {
            anyhow::anyhow!("Substance not found in database. Use 'biohack substance seed' first.")
        })?;
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
                args.name,
                args.dose,
                args.route,
                timestamp.format("%Y-%m-%d %H:%M")
            )
            .green()
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
                args.amount,
                args.unit,
                args.name,
                timestamp.format("%Y-%m-%d %H:%M")
            )
            .green()
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
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS);
        table.set_header(vec![
            "Time",
            "Substance",
            "Dose",
            "Route",
            "Category",
            "Notes",
        ]);

        for log in logs {
            let category = log.category.as_deref().unwrap_or("—");
            let notes = log.notes.as_deref().unwrap_or("—");

            table.add_row(vec![
                Cell::new(log.timestamp.format("%Y-%m-%d %H:%M").to_string()),
                Cell::new(&log.substance_name),
                Cell::new(format!("{}mg", log.dose_mg as u64)),
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
            let hr = log
                .heart_rate
                .map(|v| v.to_string())
                .unwrap_or_else(|| "—".to_string());
            let sbp = log
                .sbp
                .map(|v| v.to_string())
                .unwrap_or_else(|| "—".to_string());
            let dbp = log
                .dbp
                .map(|v| v.to_string())
                .unwrap_or_else(|| "—".to_string());
            let temp = log
                .temperature_c
                .map(|v| format!("{:.1}", v))
                .unwrap_or_else(|| "—".to_string());
            let spo2 = log
                .spo2
                .map(|v| v.to_string())
                .unwrap_or_else(|| "—".to_string());
            let hrv = log
                .hrv_rmssd
                .map(|v| v.to_string())
                .unwrap_or_else(|| "—".to_string());
            let weight = log
                .weight_kg
                .map(|v| format!("{:.1}", v))
                .unwrap_or_else(|| "—".to_string());
            let notes = log.notes.as_deref().unwrap_or("—");

            table.add_row(vec![
                Cell::new(log.timestamp.format("%Y-%m-%d %H:%M").to_string()),
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
        table.set_header(vec!["Time", "Type", "Details", "Notes"]);

        for entry in entries {
            let timestamp_str = entry.timestamp().format("%Y-%m-%d %H:%M").to_string();
            let (entry_type, details, notes) = match entry {
                crate::db::TimelineEntry::Substance(log) => (
                    "Substance".to_string(),
                    format!(
                        "{} {}mg {}",
                        log.substance_name, log.dose_mg as u64, log.route
                    ),
                    log.notes.as_deref().unwrap_or("—").to_string(),
                ),
                crate::db::TimelineEntry::Vitals(log) => {
                    let mut parts = Vec::new();
                    if let Some(hr) = log.heart_rate {
                        parts.push(format!("HR:{}", hr));
                    }
                    if let Some(sbp) = log.sbp {
                        parts.push(format!("SBP:{}", sbp));
                    }
                    if let Some(dbp) = log.dbp {
                        parts.push(format!("DBP:{}", dbp));
                    }
                    if let Some(temp) = log.temperature_c {
                        parts.push(format!("Temp:{:.1}°C", temp));
                    }
                    if let Some(spo2) = log.spo2 {
                        parts.push(format!("SpO2:{}%", spo2));
                    }
                    if let Some(hrv) = log.hrv_rmssd {
                        parts.push(format!("HRV:{}ms", hrv));
                    }
                    if let Some(weight) = log.weight_kg {
                        parts.push(format!("W:{:.1}kg", weight));
                    }
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

pub fn handle_substance_search(
    _db: &Database,
    _args: &SubstanceCommands,
    _no_color: bool,
) -> Result<()> {
    println!("{}", "Not yet implemented: search substances".yellow());
    Ok(())
}

pub fn handle_substance_show(
    _db: &Database,
    _args: &SubstanceCommands,
    _no_color: bool,
) -> Result<()> {
    println!("{}", "Not yet implemented: show substance details".yellow());
    Ok(())
}

/// Create a stack from a YAML file
pub fn handle_stack_create(db: &Database, args: &StackCommands, _no_color: bool) -> Result<()> {
    if let StackCommands::Create(args) = args {
        let content = std::fs::read_to_string(&args.path)?;
        let stack: Stack = serde_yaml::from_str(&content)?;

        // Validate each substance exists in the database
        for item in &stack.items {
            let substance = db.get_substance_by_name(&item.substance_name)?;
            if substance.is_none() {
                anyhow::bail!(
                    "Substance '{}' not found in database. Run 'biohack substance seed' or add it first.",
                    item.substance_name
                );
            }
        }

        db.insert_stack(&stack)?;
        println!(
            "{}",
            format!("��� Created stack: {}", stack.name).green().bold()
        );
        if let Some(desc) = &stack.description {
            println!("  Description: {}", desc);
        }
        println!("  Items: {}", stack.items.len());
        for item in &stack.items {
            let schedule_str = item
                .schedule
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "unscheduled".to_string());
            println!(
                "  - {} {} {} ({})",
                item.substance_name,
                item.dose,
                item.route.as_deref().unwrap_or("oral"),
                schedule_str
            );
        }
    }
    Ok(())
}

/// List all stacks
pub fn handle_stack_list(db: &Database, _args: &StackCommands, _no_color: bool) -> Result<()> {
    let stacks = db.list_stacks()?;

    if stacks.is_empty() {
        println!("{}", "No stacks found".yellow());
        return Ok(());
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);
    table.set_header(vec!["Name", "Description", "Items", "Schedules"]);

    for stack in stacks {
        let desc = stack.description.as_deref().unwrap_or("—");
        let schedules: std::collections::HashSet<String> = stack
            .items
            .iter()
            .filter_map(|i| i.schedule.as_ref().map(|s| s.to_string()))
            .collect();
        let schedule_str = if schedules.is_empty() {
            "unscheduled".to_string()
        } else {
            schedules.into_iter().collect::<Vec<_>>().join(", ")
        };

        table.add_row(vec![
            Cell::new(&stack.name),
            Cell::new(desc),
            Cell::new(stack.items.len().to_string()),
            Cell::new(schedule_str),
        ]);
    }

    println!("{}", table);
    Ok(())
}

/// Show stack details
pub fn handle_stack_show(db: &Database, args: &StackCommands, _no_color: bool) -> Result<()> {
    if let StackCommands::Show(args) = args {
        let stack = db.get_stack(&args.name)?;

        match stack {
            Some(stack) => {
                println!("{}", format!("���� Stack: {}", stack.name).green().bold());
                if let Some(desc) = &stack.description {
                    println!("  Description: {}", desc);
                }
                println!("  Items: {}", stack.items.len());
                for item in &stack.items {
                    let schedule_str = item
                        .schedule
                        .as_ref()
                        .map(|s| format!(" [{}]", s))
                        .unwrap_or_else(|| " [unscheduled]".to_string());
                    println!(
                        "  - {} {} {}{}",
                        item.substance_name,
                        item.dose,
                        item.route.as_deref().unwrap_or("oral"),
                        schedule_str
                    );
                }
            }
            None => {
                println!("{}", format!("Stack '{}' not found", args.name).red());
            }
        }
    }
    Ok(())
}

/// Log a stack (log all substances in the stack at once)
pub fn handle_log_stack(db: &Database, args: &LogCommands, _no_color: bool) -> Result<()> {
    if let LogCommands::Stack(args) = args {
        let timestamp = parse_time(&args.time)?;

        // Get the stack
        let stack = db
            .get_stack(&args.name)?
            .ok_or_else(|| anyhow::anyhow!("Stack '{}' not found", args.name))?;

        if stack.items.is_empty() {
            println!("{}", "Stack has no items".yellow());
            return Ok(());
        }

        let mut logged = 0;
        for item in &stack.items {
            let substance = db
                .get_substance_by_name(&item.substance_name)?
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Substance '{}' in stack not found in database",
                        item.substance_name
                    )
                })?;

            let dose_mg = parse_dose(&item.dose)?;

            let log = SubstanceLog {
                id: Uuid::new_v4(),
                substance_id: substance.id,
                substance_name: item.substance_name.clone(),
                dose_mg,
                route: item.route.clone().unwrap_or_else(|| "oral".to_string()),
                timestamp,
                notes: Some(format!("Logged via stack: {}", stack.name)),
                category: Some(substance.category.to_string()),
            };

            db.insert_substance_log(&log)?;
            logged += 1;
            println!(
                "  {}",
                format!(
                    "��� {} {} {}",
                    item.substance_name,
                    item.dose,
                    item.route.as_deref().unwrap_or("oral")
                )
                .green()
            );
        }

        println!(
            "{}",
            format!(
                "���� Logged stack '{}': {} items at {}",
                stack.name,
                logged,
                timestamp.format("%Y-%m-%d %H:%M")
            )
            .green()
            .bold()
        );
    }
    Ok(())
}

pub fn handle_check(_db: &Database, _no_color: bool) -> Result<()> {
    println!("{}", "������� Safety check: no protocols triggered".green());
    Ok(())
}

/// Protocol list
pub fn handle_protocol_list(
    _db: &Database,
    _args: &ProtocolCommands,
    _no_color: bool,
) -> Result<()> {
    let mut engine = ProtocolEngine::new();
    engine.load_builtin_protocols()?;

    if engine.protocols.is_empty() {
        println!("{}", "No protocols found".yellow());
        return Ok(());
    }

    for protocol in &engine.protocols {
        println!(
            "{}",
            format!("[PROTOCOL] {} ({})", protocol.name, protocol.id)
                .green()
                .bold()
        );
        println!("  {}", protocol.description);
        println!("  Version: {}", protocol.version);
        println!("  Actions: {}", protocol.actions.len());
        println!();
    }
    Ok(())
}

/// Protocol test
pub fn handle_protocol_test(db: &Database, args: &ProtocolCommands, _no_color: bool) -> Result<()> {
    if let ProtocolCommands::Test(args) = args {
        let mut engine = ProtocolEngine::new();
        engine.load_builtin_protocols()?;

        let protocol = engine
            .protocols
            .iter()
            .find(|p| p.id == args.protocol_id)
            .ok_or_else(|| anyhow::anyhow!("Protocol '{}' not found", args.protocol_id))?;

        println!(
            "{}",
            format!(
                "[TEST] Testing protocol: {} ({})",
                protocol.name, protocol.id
            )
            .green()
            .bold()
        );
        println!("Description: {}", protocol.description);
        println!();

        // Get recent data for testing
        let recent_substances = db.get_recent_substance_logs(24, None)?;
        let recent_vitals = db.get_recent_vitals_logs(24)?;
        let current_vitals = recent_vitals.first().cloned();

        let ctx = ProtocolContext {
            recent_substances,
            recent_vitals,
            current_vitals,
        };

        let results = engine.evaluate(&ctx);
        let result = results
            .iter()
            .find(|r| r.protocol_id == args.protocol_id)
            .ok_or_else(|| anyhow::anyhow!("Protocol evaluation failed"))?;

        println!("Triggered: {}", if result.triggered { "YES" } else { "NO" });
        println!();

        if !result.matched_conditions.is_empty() {
            println!("Matched conditions:");
            for cond in &result.matched_conditions {
                println!("  - {}", cond);
            }
            println!();
        }

        if !result.actions.is_empty() {
            println!("Actions (by priority):");
            for action in &result.actions {
                let prefix = match action.action_type.as_str() {
                    "alert" => "[ALERT]",
                    "suggestion" => "[SUGGESTION]",
                    "constraint" => "[CONSTRAINT]",
                    _ => "[ACTION]",
                };
                println!(
                    "  {} {} (priority {})",
                    prefix, action.message, action.priority
                );
                if let Some(rationale) = &action.rationale {
                    println!("    Rationale: {}", rationale);
                }
            }
        }
    }
    Ok(())
}

/// Generate markdown report
fn generate_markdown_report(db: &Database, days: u32, _format: &str) -> Result<String> {
    let summary = db.get_report_summary(days)?;
    let substance_logs = db.get_recent_substance_logs_detailed(days)?;
    let vitals_logs = db.get_recent_vitals_logs_detailed(days)?;
    let food_logs = db.get_recent_food_logs_detailed(days)?;
    let stacks = db.list_stacks()?;

    let mut report = String::new();
    let now = Utc::now();
    let date_range_start = now - chrono::Duration::days(days as i64);

    // Header
    report.push_str("# Biohack Health Report\n\n");
    report.push_str(&format!(
        "**Generated:** {}\n",
        now.format("%Y-%m-%d %H:%M UTC")
    ));
    report.push_str(&format!(
        "**Period:** {} to {} ({} days)\n\n",
        date_range_start.format("%Y-%m-%d"),
        now.format("%Y-%m-%d"),
        days
    ));

    // Summary
    report.push_str("## Summary\n\n");
    report.push_str(&format!(
        "- **Substance Logs:** {}\n",
        summary.substance_logs
    ));
    report.push_str(&format!(
        "- **Unique Substances:** {}\n",
        summary.unique_substances
    ));
    report.push_str(&format!("- **Vitals Logs:** {}\n", summary.vitals_logs));
    report.push_str(&format!("- **Food Logs:** {}\n", summary.food_logs));
    report.push_str(&format!("- **Defined Stacks:** {}\n\n", stacks.len()));

    // Stacks
    if !stacks.is_empty() {
        report.push_str("## Defined Stacks\n\n");
        for stack in &stacks {
            report.push_str(&format!("### {}\n", stack.name));
            if let Some(desc) = &stack.description {
                report.push_str(&format!("*{desc}*\n\n"));
            }
            for item in &stack.items {
                let schedule = item
                    .schedule
                    .as_ref()
                    .map(|s| format!("[{}] ", s))
                    .unwrap_or_default();
                let route = item.route.as_deref().unwrap_or("oral");
                report.push_str(&format!(
                    "- {schedule}{dose} {route} — {name}\n",
                    dose = item.dose,
                    name = item.substance_name
                ));
            }
            report.push('\n');
        }
    }

    // Substance Logs
    if !substance_logs.is_empty() {
        report.push_str("## Substance Intake Log\n\n");
        report.push_str("| Date & Time | Substance | Dose | Route | Category | Notes |\n");
        report.push_str("|-------------|-----------|------|-------|----------|-------|\n");
        for log in &substance_logs {
            let category = log.category.as_deref().unwrap_or("—");
            let notes = log.notes.as_deref().unwrap_or("—");
            let dose_str = format_dose(log.dose_mg);
            report.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} |\n",
                log.timestamp.format("%Y-%m-%d %H:%M"),
                log.substance_name,
                dose_str,
                log.route,
                category,
                notes.replace('|', "\\|")
            ));
        }
        report.push('\n');
    }

    // Vitals Logs
    if !vitals_logs.is_empty() {
        report.push_str("## Vitals Log\n\n");
        report.push_str("| Date & Time | HR | SBP | DBP | Temp (°C) | SpO2 (%) | HRV (ms) | Weight (kg) | Notes |\n");
        report.push_str("|-------------|----|-----|-----|-----------|----------|----------|-------------|-------|\n");
        for log in &vitals_logs {
            let hr = log
                .heart_rate
                .map(|v| v.to_string())
                .unwrap_or_else(|| "—".to_string());
            let sbp = log
                .sbp
                .map(|v| v.to_string())
                .unwrap_or_else(|| "—".to_string());
            let dbp = log
                .dbp
                .map(|v| v.to_string())
                .unwrap_or_else(|| "—".to_string());
            let temp = log
                .temperature_c
                .map(|v| format!("{:.1}", v))
                .unwrap_or_else(|| "—".to_string());
            let spo2 = log
                .spo2
                .map(|v| v.to_string())
                .unwrap_or_else(|| "—".to_string());
            let hrv = log
                .hrv_rmssd
                .map(|v| v.to_string())
                .unwrap_or_else(|| "—".to_string());
            let weight = log
                .weight_kg
                .map(|v| format!("{:.1}", v))
                .unwrap_or_else(|| "—".to_string());
            let notes = log.notes.as_deref().unwrap_or("—");

            report.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
                log.timestamp.format("%Y-%m-%d %H:%M"),
                hr,
                sbp,
                dbp,
                temp,
                spo2,
                hrv,
                weight,
                notes.replace('|', "\\|")
            ));
        }
        report.push('\n');
    }

    // Food Logs
    if !food_logs.is_empty() {
        report.push_str("## Food Intake Log\n\n");
        report.push_str("| Date & Time | Food | Amount | Unit | Notes |\n");
        report.push_str("|-------------|------|--------|------|-------|\n");
        for log in &food_logs {
            let notes = log.notes.as_deref().unwrap_or("—");
            report.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                log.timestamp.format("%Y-%m-%d %H:%M"),
                log.food_name,
                log.amount,
                log.unit,
                notes.replace('|', "\\|")
            ));
        }
        report.push('\n');
    }

    // Vitals Summary (clinician-friendly)
    if !vitals_logs.is_empty() {
        report.push_str("## Vitals Summary (for Clinician Review)\n\n");

        let hr_vals: Vec<u32> = vitals_logs.iter().filter_map(|l| l.heart_rate).collect();
        let sbp_vals: Vec<u32> = vitals_logs.iter().filter_map(|l| l.sbp).collect();
        let dbp_vals: Vec<u32> = vitals_logs.iter().filter_map(|l| l.dbp).collect();
        let temp_vals: Vec<f32> = vitals_logs.iter().filter_map(|l| l.temperature_c).collect();
        let spo2_vals: Vec<u32> = vitals_logs.iter().filter_map(|l| l.spo2).collect();

        if !hr_vals.is_empty() {
            let avg_hr = hr_vals.iter().sum::<u32>() as f32 / hr_vals.len() as f32;
            let min_hr = *hr_vals.iter().min().unwrap();
            let max_hr = *hr_vals.iter().max().unwrap();
            report.push_str(&format!(
                "- **Heart Rate:** avg {avg_hr:.0} bpm (range {min_hr}–{max_hr})\n"
            ));
        }
        if !sbp_vals.is_empty() && !dbp_vals.is_empty() {
            let avg_sbp = sbp_vals.iter().sum::<u32>() as f32 / sbp_vals.len() as f32;
            let avg_dbp = dbp_vals.iter().sum::<u32>() as f32 / dbp_vals.len() as f32;
            let min_sbp = *sbp_vals.iter().min().unwrap();
            let max_sbp = *sbp_vals.iter().max().unwrap();
            let min_dbp = *dbp_vals.iter().min().unwrap();
            let max_dbp = *dbp_vals.iter().max().unwrap();
            report.push_str(&format!("- **Blood Pressure:** avg {avg_sbp:.0}/{avg_dbp:.0} mmHg (SBP range {min_sbp}–{max_sbp}, DBP range {min_dbp}–{max_dbp})\n"));
        }
        if !temp_vals.is_empty() {
            let avg_temp = temp_vals.iter().sum::<f32>() / temp_vals.len() as f32;
            let min_temp = temp_vals.iter().fold(f32::INFINITY, |a, &b| a.min(b));
            let max_temp = temp_vals.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
            report.push_str(&format!(
                "- **Temperature:** avg {avg_temp:.1}°C (range {min_temp:.1}–{max_temp:.1})\n"
            ));
        }
        if !spo2_vals.is_empty() {
            let min_spo2 = *spo2_vals.iter().min().unwrap();
            let max_spo2 = *spo2_vals.iter().max().unwrap();
            report.push_str(&format!("- **SpO2:** range {min_spo2}–{max_spo2}%\n"));
        }
        report.push('\n');
    }

    // Substance Frequency
    if !substance_logs.is_empty() {
        report.push_str("## Substance Frequency\n\n");
        let mut freq: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for log in &substance_logs {
            *freq.entry(log.substance_name.clone()).or_insert(0) += 1;
        }
        let mut freq_vec: Vec<_> = freq.into_iter().collect();
        freq_vec.sort_by_key(|b| std::cmp::Reverse(b.1));

        report.push_str("| Substance | Log Count |\n");
        report.push_str("|-----------|-----------|\n");
        for (name, count) in freq_vec {
            report.push_str(&format!("| {} | {} |\n", name, count));
        }
        report.push('\n');
    }

    // Footer
    report.push_str("---\n");
    report.push_str("*Report generated by biohack v0.1.0 — This tool is for informational and tracking purposes only. It does not provide medical advice. Always consult a qualified healthcare provider for medical concerns.*\n");

    Ok(report)
}

/// Generate CSV export
fn generate_csv_report(db: &Database, days: u32) -> Result<String> {
    let substance_logs = db.get_recent_substance_logs_detailed(days)?;
    let vitals_logs = db.get_recent_vitals_logs_detailed(days)?;
    let food_logs = db.get_recent_food_logs_detailed(days)?;

    let mut csv = String::new();

    // Substance logs CSV
    if !substance_logs.is_empty() {
        csv.push_str("# Substance Logs\n");
        csv.push_str("timestamp,substance_name,dose_mg,route,category,notes\n");
        for log in &substance_logs {
            let category = log.category.as_deref().unwrap_or("");
            let notes = log.notes.as_deref().unwrap_or("").replace('"', "\"\"");
            csv.push_str(&format!(
                "{},{},{},{},{},{}\n",
                log.timestamp.to_rfc3339(),
                log.substance_name.replace('"', "\"\""),
                log.dose_mg,
                log.route,
                category,
                notes
            ));
        }
        csv.push('\n');
    }

    // Vitals logs CSV
    if !vitals_logs.is_empty() {
        csv.push_str("# Vitals Logs\n");
        csv.push_str("timestamp,heart_rate,sbp,dbp,temperature_c,spo2,hrv_rmssd,weight_kg,notes\n");
        for log in &vitals_logs {
            let notes = log.notes.as_deref().unwrap_or("").replace('"', "\"\"");
            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                log.timestamp.to_rfc3339(),
                log.heart_rate.map(|v| v.to_string()).unwrap_or_default(),
                log.sbp.map(|v| v.to_string()).unwrap_or_default(),
                log.dbp.map(|v| v.to_string()).unwrap_or_default(),
                log.temperature_c
                    .map(|v| format!("{:.1}", v))
                    .unwrap_or_default(),
                log.spo2.map(|v| v.to_string()).unwrap_or_default(),
                log.hrv_rmssd.map(|v| v.to_string()).unwrap_or_default(),
                log.weight_kg
                    .map(|v| format!("{:.1}", v))
                    .unwrap_or_default(),
                notes
            ));
        }
        csv.push('\n');
    }

    // Food logs CSV
    if !food_logs.is_empty() {
        csv.push_str("# Food Logs\n");
        csv.push_str("timestamp,food_name,amount,unit,notes\n");
        for log in &food_logs {
            let notes = log.notes.as_deref().unwrap_or("").replace('"', "\"\"");
            csv.push_str(&format!(
                "{},{},{},{},{}\n",
                log.timestamp.to_rfc3339(),
                log.food_name.replace('"', "\"\""),
                log.amount,
                log.unit,
                notes
            ));
        }
        csv.push('\n');
    }

    Ok(csv)
}

/// Write report to file or stdout
fn write_report(output: Option<&std::path::Path>, content: &str) -> Result<()> {
    if let Some(path) = output {
        let mut file = std::fs::File::create(path)?;
        file.write_all(content.as_bytes())?;
        println!(
            "{}",
            format!("Report written to {}", path.display()).green()
        );
    } else {
        print!("{content}");
    }
    Ok(())
}

/// Report generation command handler
pub fn handle_report(db: &Database, args: &ReportArgs, _no_color: bool) -> Result<()> {
    let format = args.format.to_lowercase();

    match format.as_str() {
        "markdown" | "md" => {
            let report = generate_markdown_report(db, args.days, &format)?;
            write_report(args.output.as_deref(), &report)?;
        }
        "csv" => {
            let report = generate_csv_report(db, args.days)?;
            write_report(args.output.as_deref(), &report)?;
        }
        _ => {
            anyhow::bail!(
                "Unknown format '{}'. Supported formats: markdown, csv",
                args.format
            );
        }
    }

    Ok(())
}

/// Parse a dose string (e.g., "400mg", "2.5g", "10ml", "50mcg", "5000IU") into milligrams as f64
fn parse_dose(s: &str) -> Result<f64> {
    let s = s.trim().to_lowercase();
    if s.is_empty() {
        anyhow::bail!(
            "Dose cannot be empty. Use format like '400mg', '2.5g', '10ml', '5000iu', or '400' (assumes mg)"
        );
    }
    if s.ends_with("mcg") || s.ends_with("µg") {
        let num_part = s.trim_end_matches("mcg").trim_end_matches("µg");
        num_part.parse::<f64>().map(|v| v / 1000.0).map_err(|_| {
            anyhow::anyhow!(
                "Invalid dose format: '{}'. Use format like '50mcg' or '50µg'",
                s
            )
        })
    } else if s.ends_with("mg") {
        let num_part = s.trim_end_matches("mg");
        num_part
            .parse::<f64>()
            .map_err(|_| anyhow::anyhow!("Invalid dose format: '{}'. Use format like '400mg'", s))
    } else if s.ends_with("g") {
        let num_part = s.trim_end_matches("g");
        num_part
            .parse::<f64>()
            .map(|v| v * 1000.0)
            .map_err(|_| anyhow::anyhow!("Invalid dose format: '{}'. Use format like '2.5g'", s))
    } else if s.ends_with("ml") {
        let num_part = s.trim_end_matches("ml");
        num_part
            .parse::<f64>()
            .map(|v| v * 1000.0)
            .map_err(|_| anyhow::anyhow!("Invalid dose format: '{}'. Use format like '10ml'", s))
    } else if s.ends_with("iu") {
        // International Units - treat as-is (no conversion to mg since it varies by substance)
        let num_part = s.trim_end_matches("iu");
        num_part
            .parse::<f64>()
            .map_err(|_| anyhow::anyhow!("Invalid dose format: '{}'. Use format like '5000iu'", s))
    } else {
        // Assume mg for bare numbers
        s.parse::<f64>()
            .map_err(|_| anyhow::anyhow!("Invalid dose format: '{}'. Use format like '400mg', '2.5g', '10ml', '5000iu', or '400' (assumes mg)", s))
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
        Some(t) => DateTime::parse_from_rfc3339(t)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|_| anyhow::anyhow!("Invalid timestamp format: '{}'. Use ISO 8601 format like '2024-01-15T10:30:00Z' or '2024-01-15'", t))?,
        None => Utc::now(),
    })
}
