use std::fs::OpenOptions;
use std::io::{ self, Write };
use std::path::{ Path, PathBuf };
use anyhow::{Context, Result};
use fs2::FileExt;
use std::str::FromStr;

fn atomic_append(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).with_context(|| format!("create parent of {}", path.display()))?;
    }
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("opening {} for append", path.display()))?;
    f.write_all(bytes).context("writing entry")?;
    f.sync_data().context("syncing entry")?;
    Ok(())
}

fn with_lock<F, T>(lock_path: &Path, f: F) -> Result<T>
where
    F: FnOnce() -> Result<T>
{
    if let Some(parent) = lock_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let lock = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(lock_path)
        .with_context(|| format!("opening lock file {}", lock_path.display()))?;
    lock.lock_exclusive().context("acquring lock")?;
    let result = f();

    // Lock drops when the function ends, so a Drop() is redundant
    result
}

pub trait LogEntry {
    fn to_markdown(&self) -> String;
}

pub fn add_log_entry<T, F>(entry: T, path_fn: F) -> Result<()>
where
    T: LogEntry,
    F: FnOnce() -> Result<PathBuf>
{
    let log_path = path_fn()?;
    with_lock(&log_path.with_extension("md.lock"), || {
        atomic_append(&log_path, entry.to_markdown().as_bytes())?;
        Ok(())
    })?;
    println!("Logged to {}", log_path.display());
    Ok(())
}

pub fn parse_stdin<T>(label: &str, required: bool) -> Option<T>
where 
    T: FromStr,
    T::Err: std::fmt::Display
{
    loop {
        if required {
            print!("{}: ", label);
        } else {
            print!("{} (optional): ", label);
        }
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let trimmed = input.trim();

        // Check if the parameter is optional if it is empty
        if trimmed.is_empty() {
            if required {
                println!("This field is required. Please try again.");
                continue;
            } else {
                return None;
            }
        }

        // Ensure the input matches the required type
        match trimmed.parse::<T>() {
            Ok(val) => return Some(val),
            Err(e) => println!("Invalid input {}. Please try again.", e)
        }
    }
}
