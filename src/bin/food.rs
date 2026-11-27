use anyhow::Result;
use clap::{Args, Parser, Subcommand, ValueEnum};
use time::{Date, macros::format_description, OffsetDateTime};
use std::fmt;
use std::str::FromStr;
use training::{ file_writer, paths };


#[derive(Parser, Debug)]
#[command(version, about = "Food/calorie logger", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Log food. Leave empty for interactive input
    Add(AddArgs),

    /// See what you logged today so far
    Today,

    /// See what you logged this week so far
    Week,

    /// Prints calories within a certain range
    Range {
        #[arg(short = 'f', long, value_parser = parse_date)]
        from: Date,

        /// Uses today as the "to" date if blank
        #[arg(short = 't', long, requires = "from", value_parser = parse_date)]
        to: Option<Date>,
    }
}

#[derive(Args, Debug)]
#[group(multiple = true)]
struct AddArgs {
    #[arg(short = 't', long, requires_all = ["kcal", "meal_name"])]
    meal_type: Option<MealType>,

    #[arg(short = 'k', long, requires_all = ["meal_type", "meal_name"])]
    kcal: Option<i32>,

    #[arg(short = 'n', long, requires_all = ["meal_type", "kcal"])]
    meal_name: Option<String>,

    #[arg(short = 'd', long, requires_all = ["meal_type", "kcal", "meal_name"])]
    details: Option<String>
}

#[derive(ValueEnum, Clone, Debug)]
enum MealType {
    Breakfast,
    Lunch,
    Dinner,
    Snack
}

struct FoodEntry {
    meal_type: MealType,
    kcal: i32,
    meal_name: String,
    details: Option<String>
}


impl FoodEntry {
    pub fn to_markdown(&self) -> String {
        file_writer::format_entry(
            &self.meal_type,
            &self.meal_name,
            Some(self.kcal),
            self.details.as_deref()
        )
    }
}

impl FromStr for FoodEntry {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!("")
    }
}

impl fmt::Display for MealType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MealType::Breakfast => { write!(f, "breakfast") }
            MealType::Lunch => { write!(f, "lunch") }
            MealType::Dinner => { write!(f, "dinner") }
            MealType::Snack => { write!(f, "snack") }
        }
    }
}

impl FromStr for MealType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
           "breakfast" | "b" => Ok(MealType::Breakfast),
            "lunch" | "l" => Ok(MealType::Lunch),
            "dinner" | "d" => Ok(MealType::Dinner),
            "snack" | "s" => Ok(MealType::Snack),
            _ => Err("Valid options: breakfast, lunch, dinner, snack")
        }
    }
}

fn parse_date(s: &str) -> Result<Date, time::error::Parse> {
    let format = format_description!("[year]-[month]-[day]");
    Date::parse(s, &format)
}


// if `date` is none or greater than today, it returns today
fn clamp_to_today(date: &Option<Date>) -> Date {
    let today = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc()).date();
    date.map_or(today, |d| d.min(today))
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Commands::Add(args) => {
            let entry = FoodEntry {
                meal_type: file_writer::resolve(args.meal_type, "Enter the meal type (b, l, d, s)")?,
                kcal: file_writer::resolve(args.kcal, "Enter number of kcal")?,
                meal_name: file_writer::resolve(args.meal_name, "Enter the name of meal")?,
                details: args.details.or_else(|| file_writer::parse_stdin("Enter any details/notes on the meal", false)),
            };
            file_writer::add_log_entry(&paths::food_log_path(), &entry.to_markdown())?;
            Ok(())
        }
        Commands::Today => {
            println!("Summary for today");
            Ok(())
        }
        Commands::Week => {
            println!("Summary for this week starting from Monday 00:00 PT to today");
            Ok(())
        }
        Commands::Range { from, to } => {
            println!("Printing calories from {:?} to {:?}", from, clamp_to_today(&to));
            Ok(())
        }
    }
}
