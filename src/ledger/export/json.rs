use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use crate::ledger::entry::Entry;
use serde_json;
use rusqlite::Error as RusqliteError;  // Alias to avoid name conflict
use rusqlite::Result;

pub fn export_to_json(entries: &Vec<Entry>, path: PathBuf) -> Result<()> {
    let json_string = serde_json::to_string_pretty(entries)
        .map_err(|e| RusqliteError::InvalidParameterName(e.to_string()))?;  // Convert JSON error

    let mut file = File::create(path)
        .map_err(|e| RusqliteError::InvalidParameterName(e.to_string()))?;  // Convert IO error

    file.write_all(json_string.as_bytes())
        .map_err(|e| RusqliteError::InvalidParameterName(e.to_string()))?;  // Convert write error

    Ok(())
}