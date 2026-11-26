use clap::{Args, Parser, Subcommand, ValueEnum};
use time::{Date, macros::format_description, OffsetDateTime};
use std::fmt;


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

    /// See what you logged on a given day
    Day {
        #[arg(value_parser = parse_date)]
        date: Date
    },

    /// See what you logged today so far
    Today,

    /// See what you logged this week so far
    Week,

    /// See the average calories over a period of time
    Avg(AvgArgs),

    /// Search for a given food or meal type
    Search(SearchArgs),

    /// Prints data from range
    Range {
        #[arg(short = 'f', long, value_parser = parse_date)]
        from: Date,

        /// Uses today as the "to" date if blank
        #[arg(short = 't', long, requires = "from", value_parser = parse_date)]
        to: Option<Date>,
    },

    /// Sync markdown file with sql file
    SyncDb
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

#[derive(Args, Debug)]
#[group(multiple = false)]
struct SearchArgs {
    #[arg(short = 'f', long)]
    food: Option<String>,

    #[arg(short = 't', long)]
    meal_type: Option<MealType>
}

#[derive(Args, Debug)]
#[clap(group(clap::ArgGroup::new("mode").required(true).args(["num_days_back", "from"])))]
struct AvgArgs {
    #[arg(conflicts_with_all = ["from", "to"])]
    num_days_back: Option<i32>,

    #[arg(short = 'f', long, conflicts_with = "num_days_back", value_parser = parse_date)]
    from: Option<Date>,

    /// Uses today as the "to" date if blank
    #[arg(short = 't', long, requires = "from", conflicts_with = "num_days_back", value_parser = parse_date)]
    to: Option<Date>
}

#[derive(ValueEnum, Clone, Debug)]
enum MealType {
    Breakfast,
    Lunch,
    Dinner,
    Snack
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

fn parse_date(s: &str) -> Result<Date, time::error::Parse> {
    let format = format_description!("[year]-[month]-[day]");
    Date::parse(s, &format)
}


// if `date` is none or greater than today, it returns today
fn clamp_to_today(date: &Option<Date>) -> Date {
    let today = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc()).date();
    date.map_or(today, |d| d.min(today))
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(AddArgs { meal_type: Some(meal_type), kcal: Some(kcal), meal_name: Some(meal_name), details }) => {
            match &details {
                Some(d) =>  println!("Logging {:?}: {} ({} kcal) - {}", meal_type, meal_name, kcal, d),
                None => println!("Logging {:?}: {} ({} kcal)", meal_type, meal_name, kcal)
            }
        }
        Commands::Add(_) => {
            println!("Starting interactive logger...");
        }
        Commands::Day { date } => {
            println!("Summary for {}", date);
        }
        Commands::Today => {
            println!("Summary for today");
        }
        Commands::Week => {
            println!("Summary for this week starting from Monday 00:00 PT to today");
        }
        Commands::Avg(AvgArgs { num_days_back: Some(num_days_back), .. }) => {
            println!("Avg calories for {} days back", num_days_back);
        }
        Commands::Avg(AvgArgs { from: Some(from), to, .. }) => {
            println!("Average calories burned per day from {:?} to {:?}", from, clamp_to_today(to));
        }
        Commands::Search(SearchArgs { food: Some(food), .. }) => {
            println!("Finding entries with food: {}...", food);
        }
        Commands::Search(SearchArgs {meal_type: Some(meal_type), .. }) => {
            println!("Finding entries with meal_type: {}...", meal_type);
        }
        Commands::Range { from, to } => {
            println!("Printing calories from {:?} to {:?}", from, clamp_to_today(to));
        }
        Commands::SyncDb => {
            println!("Sync the database with the markdown file.");
        }
        _ => unreachable!(),
    }

}
