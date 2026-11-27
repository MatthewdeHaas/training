use std::env;
use anyhow::Result;
use std::path::PathBuf;

pub fn food_log_path() -> Result<PathBuf> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("logs");
    path.push("food.md");
    Ok(path)
}

pub fn training_log_path() -> Result<PathBuf> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("logs");
    path.push("training.md");
    Ok(path)
}
