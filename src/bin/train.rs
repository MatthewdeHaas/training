use clap::{Args, Parser, Subcommand, ValueEnum};
use std::fmt;

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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(AddArgs { activity_type: Some(activity_type), summary: Some(summary), details}) => {
            match &details {
                Some(d) =>  println!("Logging activity {:?} with summary {} and details {}", activity_type, summary, d),
                None => println!("Logging activity {:?} with summary {} (no details provided)", activity_type, summary)
            }
        } 
        Commands::Add(_) => {
            println!("Starting interactive logger...");
        }
        Commands::Recent { number_of_activities } => {
            match number_of_activities {
                Some(n) => println!("Logging the {} most recent activities...", n),
                None => println!("Logging the 5 most recent activities")
            }
        }
        Commands::Today => {
            println!("Showing activities for today...")
        }
        Commands::Week => {
            println!("Showing activities for this week...")
        }
    }

}
