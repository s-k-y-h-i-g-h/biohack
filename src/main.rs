#!/usr/bin/env rust
//! biohack - Biohacker's safety-first tracking CLI

use anyhow::Result;
use biohack::{
    cli::{
        Cli, Commands, LogCommands, NutrientCommands, ProtocolCommands, RemoveCommands,
        ShowCommands, StackCommands, SubstanceCommands,
    },
    commands,
    db::Database,
};
use clap::Parser;
use owo_colors::OwoColorize;
use uuid::Uuid;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let db = Database::new(cli.db_path.clone())?;

    match cli.command {
        Commands::Init => commands::handle_init(&db)?,

        Commands::Log(cmd) => match cmd {
            LogCommands::Substance(ref _args) => {
                commands::handle_log_substance(&db, &cmd, cli.no_color)?
            }
            LogCommands::Vitals(ref _args) => commands::handle_log_vitals(&db, &cmd, cli.no_color)?,
            LogCommands::Stack(ref _args) => commands::handle_log_stack(&db, &cmd, cli.no_color)?,
            LogCommands::Food(ref _args) => commands::handle_log_food(&db, &cmd, cli.no_color)?,
        },

        Commands::Show(cmd) => match cmd {
            ShowCommands::Substances { .. } => {
                commands::handle_show_substances(&db, &cmd, cli.no_color)?
            }
            ShowCommands::Vitals { .. } => commands::handle_show_vitals(&db, &cmd, cli.no_color)?,
            ShowCommands::Timeline { .. } => {
                commands::handle_show_timeline(&db, &cmd, cli.no_color)?
            }
            ShowCommands::FoodSearch(args) => {
                commands::handle_food_search(&db, &args, cli.no_color)?
            }
        },

        Commands::Substance(cmd) => match cmd {
            SubstanceCommands::List(ref _args) => {
                commands::handle_substance_list(&db, &cmd, cli.no_color)?
            }
            SubstanceCommands::Search(ref _args) => {
                commands::handle_substance_search(&db, &cmd, cli.no_color)?
            }
            SubstanceCommands::Show(ref _args) => {
                commands::handle_substance_show(&db, &cmd, cli.no_color)?
            }
            SubstanceCommands::Add(_args) => {
                println!("{}", "Not yet implemented".yellow());
            }
            SubstanceCommands::Seed(ref _args) => {
                commands::handle_substance_seed(&db, &cmd, cli.no_color)?
            }
        },

        Commands::Stack(cmd) => match cmd {
            StackCommands::List => commands::handle_stack_list(&db, &cmd, cli.no_color)?,
            StackCommands::Show { .. } => commands::handle_stack_show(&db, &cmd, cli.no_color)?,
            StackCommands::Create { .. } => commands::handle_stack_create(&db, &cmd, cli.no_color)?,
        },

        Commands::Protocol(cmd) => match cmd {
            ProtocolCommands::List => commands::handle_protocol_list(&db, &cmd, cli.no_color)?,
            ProtocolCommands::Test { .. } => {
                commands::handle_protocol_test(&db, &cmd, cli.no_color)?
            }
            ProtocolCommands::Show { .. } => {
                println!("{}", "Not yet implemented".yellow());
            }
            ProtocolCommands::Seed => commands::handle_protocol_seed(&db, &cmd, cli.no_color)?,
            ProtocolCommands::Migrate(args) => {
                commands::handle_protocol_migrate(&db, &args, cli.no_color)?
            }
        },

        Commands::Report(args) => commands::handle_report(&db, &args, cli.no_color)?,

        Commands::Nutrient(cmd) => match cmd {
            NutrientCommands::Status(args) => {
                commands::handle_nutrient_status(&db, &args, cli.no_color)?
            }
        },

        Commands::Check => commands::handle_check(&db, cli.no_color)?,

        Commands::Remove(cmd) => match cmd {
            RemoveCommands::Substance { id } => {
                let uuid = Uuid::parse_str(&id)?;
                commands::handle_remove_substance(&db, uuid, cli.no_color)?
            }
            RemoveCommands::Vitals { id } => {
                let uuid = Uuid::parse_str(&id)?;
                commands::handle_remove_vitals(&db, uuid, cli.no_color)?
            }
            RemoveCommands::Food { id } => {
                let uuid = Uuid::parse_str(&id)?;
                commands::handle_remove_food(&db, uuid, cli.no_color)?
            }
        },
    }

    Ok(())
}
