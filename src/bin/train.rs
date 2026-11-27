use anyhow::Result;
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::fmt;
use std::str::FromStr;
use training::{ file_writer, paths };
use time::{Date, macros::format_description };

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

    /// See what you've logged this week so far
    Week,

    /// Get training entries for a given day
    Day {
        #[arg(short = 'd', long, value_parser = parse_date)]
        day: Date
    }
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

impl TrainingEntry {
    pub fn to_markdown(&self) -> String {
        file_writer::format_entry(
            &self.activity_type,
            &self.summary,
            None,
            self.details.as_deref()
        )
    }
}

fn parse_date(s: &str) -> Result<Date, time::error::Parse> {
    let format = format_description!("[year]-[month]-[day]");
    Date::parse(s, &format)
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
    match Cli::parse().command {
        Commands::Add(args) => {
            let entry = TrainingEntry {
                activity_type: file_writer::resolve(
                args.activity_type,
                "Enter the training type (r, b, h, s, l)",
                )?,
                summary: file_writer::resolve(args.summary, "Enter the session title")?,
                details: args
                .details
                .or_else(|| file_writer::parse_stdin("Enter details about the session", false)),
            };
            file_writer::add_log_entry(&paths::training_log_path(), &entry.to_markdown())?;
            Ok(())
        }
        Commands::Recent { number_of_activities: _ } => { 
            println!("Showing the most recent activities...");
            Ok(())
        }
        Commands::Week => {
            println!("Showing activities for this week..."); 
            Ok(())
        }
        Commands::Day { day } => {
            println!("Summary for {}", day);
            Ok(())
        }
    }
}
