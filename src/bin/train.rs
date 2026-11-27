use anyhow::Result;
use clap::{Args, Parser, Subcommand, ValueEnum};
use time::OffsetDateTime;
use std::fmt;
use std::str::FromStr;
use training::{ file_writer, paths, db };
use rusqlite::Connection;

#[derive(Parser, Debug)]
#[command(version, about = "Food/calorie logger", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Log an activity
    Add(AddArgs),

    /// See recently logged activities. Defaults to five
    Recent {
        #[arg(short = 'n', long)]
        number_of_activities: Option<i32>
    },

    /// See what you've logged today so far
    Today,

    /// See what you've logged this week so far
    Week
}

#[derive(Args, Debug)]
#[group(multiple = true)]
struct AddArgs {
    #[arg(short = 't', long, requires_all = ["summary"])]
    activity_type: Option<ActivityType>,

    #[arg(short = 's', long, requires_all = ["activity_type"])]
    summary: Option<String>,

    #[arg(short = 'd', long, requires_all = ["activity_type", "summary"])]
    details: Option<String>
}

#[derive(ValueEnum, Clone, Debug)]
enum ActivityType {
    Run,
    Bike,
    Hike,
    Swim,
    Lift
}

struct TrainingEntry {
    activity_type: ActivityType,
    summary: String,
    details: Option<String>
}

impl file_writer::LogEntry for TrainingEntry {
    fn to_markdown(&self) -> String {
        format!(
            "\n## {} · {}\n\
             {}\n\
             \n\
             Details:\n\
             {}\n\
             \n\
             ---\n",
            OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc()).date(),
            self.activity_type,
            self.summary,
            self.details.as_deref().unwrap_or("(No details)"),
        )
    }

    fn save_to_db(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO training (meal_type, kcal, meal_name, details) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                self.activity_type.to_string(),
                self.summary,
                self.details
            ]
        )?;
        Ok(())
    }
}

impl fmt::Display for ActivityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActivityType::Run => { write!(f, "run") }
            ActivityType::Bike => { write!(f, "bike") }
            ActivityType::Hike => { write!(f, "hike") }
            ActivityType::Swim => { write!(f, "swim") }
            ActivityType::Lift => { write!(f, "lift") }
        }
    }
}

impl FromStr for ActivityType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
           "run" | "r" => Ok(ActivityType::Run),
            "bike" | "b" => Ok(ActivityType::Bike),
            "hike" | "h" => Ok(ActivityType::Hike),
            "swim" | "s" => Ok(ActivityType::Swim),
            "lift" | "l" => Ok(ActivityType::Lift),
            _ => Err("Valid options: run, bike, hike, swim, lift")
        }
    }
}

fn main() -> Result<()> {
    let mut conn = db::init_db(&paths::db_path()?)?;
    match Cli::parse().command {
        Commands::Add(AddArgs { activity_type: Some(activity_type), summary: Some(summary), details}) => {
            file_writer::add_log_entry(
                TrainingEntry { activity_type, summary, details }, 
                paths::training_log_path,
                &mut conn
            )?;
            Ok(())
        }
        Commands::Add(_) => {
            file_writer::add_log_entry(
                TrainingEntry { 
                    activity_type: file_writer::parse_stdin("Enter the training type (r, b, h, s, l)", true).unwrap(), 
                    summary: file_writer::parse_stdin("", true).unwrap(), 
                    details: file_writer::parse_stdin("Enter details about the session", false)
                }, 
                paths::training_log_path,
                &mut conn
            )?;
            Ok(())
        }
        Commands::Recent { number_of_activities: _ } => { 
            println!("Showing the most recent activities...");
            Ok(())
        }
        Commands::Today => { 
            println!("Showing activities for today...");
            Ok(())
        }
        Commands::Week => {
            println!("Showing activities for this week..."); 
            Ok(())
        }
    }
}
