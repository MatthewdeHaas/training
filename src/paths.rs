use std::env;
use std::path::PathBuf;

fn logs_dir() -> PathBuf {
    if let Ok(dir) = env::var("TRAINING_LOGS_DIR") {
        return PathBuf::from(dir);
    }
    PathBuf::from(env::var("HOME").expect("$HOME not set")).join("Training").join("logs")
}

pub fn food_log_path() -> PathBuf {
    logs_dir().join("food.md")
}

pub fn training_log_path() -> PathBuf {
    logs_dir().join("training.md")
}
