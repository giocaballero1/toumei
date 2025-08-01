use rusqlite::{Connection, Result};

pub fn migrate(conn: &mut Connection) -> Result<()> {
    // Create the schema_version table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (version INTEGER PRIMARY KEY)",
        (),
    )?;

    // Get the current version (default to 0 if no table or row)
    let version: i32 = conn.query_row(
        "SELECT version FROM schema_version",
        (),
        |row| row.get(0),
    ).unwrap_or(0);

    // If version < 1, run the initial migration (create entries table)
    if version < 1 {
        // Version 1: Create entries table if it doesn't exist (safe for old DBs)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS entries (
                id INTEGER PRIMARY KEY,
                date TEXT NOT NULL,
                amount TEXT NOT NULL,
                description TEXT NOT NULL,
                category TEXT,
                hash TEXT
            )",
            (),
        )?;
        conn.execute("INSERT OR REPLACE INTO schema_version (version) VALUES (1)", ())?;
    }
    // Future migrations go here, e.g., if version < 2 { add new column... }

    Ok(())
}