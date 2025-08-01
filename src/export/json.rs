use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use crate::ledger::entry::Entry;  // Use your Entry struct
use serde_json;  // For JSON serialization
use rusqlite::Result;  // For error handling

pub fn export_to_json(entries: &Vec<Entry>, path: PathBuf) -> Result<()> {
    let json_string = serde_json::to_string_pretty(entries)?;  // Convert entries to pretty JSON
    let mut file = File::create(path)?;  // Create the file
    file.write_all(json_string.as_bytes())?;  // Write JSON to file
    Ok(())
}