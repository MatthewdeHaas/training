use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::Path;

pub fn init_db(db_path: &Path) -> Result<Connection> {
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).with_context(|| format!("creating parent directory for {}", db_path.display()))?;
    }

    let conn = Connection::open(db_path).with_context(|| format!("opening database at {}", db_path.display()))?;

    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;

    // Create tables if they do not exist yet
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS food (
            id         INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            meal_type  TEXT NOT NULL,
            kcal       INTEGER NOT NULL,
            meal_name  TEXT NOT NULL,
            details    TEXT
        );

        CREATE TABLE IF NOT EXISTS training (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at    TEXT NOT NULL DEFAULT (datetime('now')),
            activity_type TEXT NOT NULL,
            summary       TEXT NOT NULL,
            details       TEXT
        );
        ",
    ).context("initializing database schema")?;

    Ok(conn)
}
