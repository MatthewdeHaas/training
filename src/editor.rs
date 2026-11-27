use std::env;
use std::io::Write;
use std::process::Command;
use anyhow::{Context, Result};


pub fn prompt_in_editor(template: &str) -> Result<Option<String>> {
    let editor = env::var("VISUAL").ok().filter(|s| !s.trim().is_empty()).or_else(|| env::var("EDITOR").ok().filter(|s| !s.trim().is_empty())).unwrap_or_else(|| "nvim".to_string());
    let mut scratch = tempfile::Builder::new()
        .prefix("train-entry-")
        .suffix(".md")
        .tempfile()
        .context("creating scratch file")?;
    scratch.write_all(template.as_bytes())?;
    scratch.flush()?;
    let path = scratch.path().to_owned();

    drop(scratch.reopen()?);

    let status = Command::new(&editor)
        .arg(&path)
        .status()
        .with_context(|| format!("spawning editor {}", editor))?;

    if !status.success() {
        anyhow::bail!("editor exited with {}", status);
    }

    let raw = std::fs::read_to_string(&path).context("reading scratch file back")?;
    let cleaned: String = raw.lines().filter(|l| !l.starts_with("#")).collect::<Vec<_>>().join("\n").trim().to_string();

    Ok(if cleaned.is_empty() { None } else { Some(cleaned.to_string()) })
}
