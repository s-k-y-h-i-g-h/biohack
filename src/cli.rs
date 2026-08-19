use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "biohack",
    version,
    about = "Biohacker's safety-first tracking CLI",
    long_about = "\
A local-first tool for logging substances, vitals, food, stacks, and running deterministic safety protocols.

EXAMPLES:
  # Initialize database and seed with 27 curated substances
  biohack init
  biohack substance seed

  # Log a supplement
  biohack log substance --name \"L-Theanine\" --dose 400mg

  # Log vitals
  biohack log vitals --hr 88 --sbp 120 --dbp 80 --temp 37.0

  # Log food (MVP)
  biohack log food --name \"Broccoli\" --amount 2 --unit cups

  # Run safety check
  biohack check

  # View recent logs
  biohack show substances --days 7
  biohack show vitals --days 3
  biohack show timeline --days 7

  # Browse substance database
  biohack substance list
  biohack substance list --category nootropic
  biohack substance search magnesium

DOCUMENTATION:
  https://github.com/s-k-y-h-i-g-h/biohack/blob/main/README.md

SAFETY DISCLAIMER:
  This tool is for informational and tracking purposes only.
  It does not provide medical advice. Always consult a qualified
  healthcare provider for medical concerns.\
"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Database path (default: ~/.local/share/biohack/biohack.db)
    #[arg(short = 'd', long, global = true, env = "BIOHACK_DB")]
    pub db_path: Option<PathBuf>,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Log substance intake, vitals, food, or stack
    #[command(subcommand)]
    Log(LogCommands),

    /// View recent logs
    #[command(subcommand)]
    Show(ShowCommands),

    /// Manage substance database
    #[command(subcommand)]
    Substance(SubstanceCommands),

    /// Manage stacks
    #[command(subcommand)]
    Stack(StackCommands),

    /// Safety protocol commands
    #[command(subcommand)]
    Protocol(ProtocolCommands),

    /// Nutrient tracking commands
    #[command(subcommand)]
    Nutrient(NutrientCommands),

    /// Generate reports
    Report(ReportArgs),

    /// Run safety check against current logs
    Check,

    /// Initialize database and config
    Init,

    /// Remove a log entry by ID
    #[command(subcommand)]
    Remove(RemoveCommands),
}

#[derive(Subcommand, Debug)]
pub enum LogCommands {
    /// Log a substance intake
    ///
    /// Dose formats: 400mg, 2.5g, 10ml, 400 (assumes mg)
    /// Routes: oral, sublingual, transdermal, injection, inhalation, rectal, nasal
    Substance(SubstanceArgs),

    /// Log vitals
    ///
    /// At least one vital sign is required.
    Vitals(VitalsArgs),

    /// Log a predefined stack
    Stack(StackArgs),

    /// Log individual food intake (MVP)
    Food(FoodArgs),
}

#[derive(Args, Debug)]
pub struct SubstanceArgs {
    /// Substance name (fuzzy-matched against database)
    #[arg(short = 'n', long)]
    pub name: String,

    /// Dose (e.g., "400mg", "2.5g", "10ml")
    #[arg(long)]
    pub dose: String,

    /// Route of administration
    #[arg(long, default_value = "oral")]
    pub route: String,

    /// Timestamp (ISO 8601, default: now)
    #[arg(short = 't', long)]
    pub time: Option<String>,

    /// Additional notes
    #[arg(long)]
    pub notes: Option<String>,
}

#[derive(Args, Debug)]
pub struct VitalsArgs {
    /// Heart rate (bpm)
    #[arg(long)]
    pub hr: Option<u32>,

    /// Systolic blood pressure (mmHg)
    #[arg(long)]
    pub sbp: Option<u32>,

    /// Diastolic blood pressure (mmHg)
    #[arg(long)]
    pub dbp: Option<u32>,

    /// Temperature (Celsius)
    #[arg(long)]
    pub temp: Option<f32>,

    /// SpO2 (%)
    #[arg(long)]
    pub spo2: Option<u32>,

    /// HRV (ms, RMSSD)
    #[arg(long)]
    pub hrv: Option<u32>,

    /// Weight (kg)
    #[arg(long)]
    pub weight: Option<f32>,

    /// Timestamp (ISO 8601, default: now)
    #[arg(short = 't', long)]
    pub time: Option<String>,

    /// Additional notes
    #[arg(long)]
    pub notes: Option<String>,
}

#[derive(Args, Debug)]
pub struct StackArgs {
    /// Stack name
    pub name: String,

    /// Timestamp (ISO 8601, default: now)
    #[arg(short = 't', long)]
    pub time: Option<String>,
}

#[derive(Args, Debug)]
pub struct FoodArgs {
    /// Food name (to be matched against food database in v1.1)
    #[arg(short = 'n', long)]
    pub name: String,

    /// Amount consumed
    #[arg(short = 'a', long)]
    pub amount: f32,

    /// Unit of measurement (g, mg, cups, slices, etc.)
    #[arg(short = 'u', long, default_value = "g")]
    pub unit: String,

    /// Timestamp (ISO 8601, default: now)
    #[arg(short = 't', long)]
    pub time: Option<String>,

    /// Additional notes
    #[arg(long)]
    pub notes: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum ShowCommands {
    /// Show recent substance logs
    Substances(ShowSubstancesArgs),

    /// Show recent vitals logs
    Vitals(ShowVitalsArgs),

    /// Show all logs combined timeline
    Timeline(ShowTimelineArgs),

    /// Search USDA FoodData Central for foods and nutrients
    FoodSearch(FoodSearchArgs),
}

#[derive(Args, Debug)]
pub struct ShowSubstancesArgs {
    /// Days to look back
    #[arg(long, default_value = "3")]
    pub days: u32,

    /// Filter by substance name
    #[arg(short = 'n', long)]
    pub name: Option<String>,
}

#[derive(Args, Debug)]
pub struct ShowVitalsArgs {
    /// Days to look back
    #[arg(long, default_value = "3")]
    pub days: u32,
}

#[derive(Args, Debug)]
pub struct ShowTimelineArgs {
    /// Days to look back
    #[arg(long, default_value = "3")]
    pub days: u32,
}

#[derive(Args, Debug)]
pub struct FoodSearchArgs {
    /// Search query
    pub query: String,

    /// Maximum results to show
    #[arg(long, default_value = "10")]
    pub limit: usize,
}

#[derive(Subcommand, Debug)]
pub enum SubstanceCommands {
    /// List all substances in database
    List(SubstanceListArgs),

    /// Search substances
    Search(SubstanceSearchArgs),

    /// Show substance details (not yet implemented)
    Show(SubstanceShowArgs),

    /// Add a custom substance (not yet implemented)
    Add(SubstanceAddArgs),

    /// Seed database with initial substances
    Seed(SubstanceSeedArgs),
}

#[derive(Args, Debug)]
pub struct SubstanceListArgs {
    /// Filter by category
    #[arg(short = 'c', long)]
    pub category: Option<String>,
}

#[derive(Args, Debug)]
pub struct SubstanceSearchArgs {
    pub query: String,
}

#[derive(Args, Debug)]
pub struct SubstanceShowArgs {
    pub name: String,
}

#[derive(Args, Debug)]
pub struct SubstanceAddArgs {
    pub name: String,
    pub category: String,
    pub min_dose: String,
    pub max_dose: String,
    pub half_life_hours: Option<f32>,
    pub contraindications: Option<String>,
}

#[derive(Args, Debug)]
pub struct SubstanceSeedArgs {
    /// Path to YAML seed file (default: data/seeds/substances.yaml)
    #[arg(short = 'p', long, default_value = "data/seeds/substances.yaml")]
    pub path: PathBuf,
}

#[derive(Subcommand, Debug)]
pub enum StackCommands {
    /// List defined stacks
    List,

    /// Show stack contents
    Show(StackShowArgs),

    /// Create stack from YAML file
    Create(StackCreateArgs),
}

#[derive(Args, Debug)]
pub struct StackShowArgs {
    pub name: String,
}

#[derive(Args, Debug)]
pub struct StackCreateArgs {
    pub path: PathBuf,
}

#[derive(Subcommand, Debug)]
pub enum ProtocolCommands {
    /// List available protocols
    List,

    /// Test a protocol with simulated data
    Test(ProtocolTestArgs),

    /// Show protocol details
    Show(ProtocolShowArgs),

    /// Save built-in protocols to database
    Seed,

    /// Migrate protocol YAML files to current schema version
    Migrate(ProtocolMigrateArgs),
}

#[derive(Args, Debug)]
pub struct ProtocolMigrateArgs {
    /// Protocol ID to migrate (if omitted, migrates all)
    pub protocol_id: Option<String>,

    /// Force migration even if version is current
    #[arg(long)]
    pub force: bool,

    /// Dry run - show what would be migrated without saving
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Subcommand, Debug)]
pub enum NutrientCommands {
    /// Show nutrient intake vs RDI status
    Status(NutrientStatusArgs),
}

#[derive(Args, Debug)]
pub struct NutrientStatusArgs {
    /// Days to look back
    #[arg(long, default_value = "7")]
    pub days: u32,
}

#[derive(Args, Debug)]
pub struct ProtocolTestArgs {
    pub protocol_id: String,
}

#[derive(Args, Debug)]
pub struct ProtocolShowArgs {
    pub protocol_id: String,
}

#[derive(Args, Debug)]
pub struct ReportArgs {
    /// Days to include
    #[arg(long, default_value = "7")]
    pub days: u32,

    /// Output format (markdown, json, csv)
    #[arg(short = 'f', long, default_value = "markdown")]
    pub format: String,

    /// Output file (default: stdout)
    #[arg(short = 'o', long)]
    pub output: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum RemoveCommands {
    /// Remove a substance log by ID
    Substance { id: String },

    /// Remove a vitals log by ID
    Vitals { id: String },

    /// Remove a food log by ID
    Food { id: String },
}
