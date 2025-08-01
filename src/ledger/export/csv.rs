use std::fs::File;
use std::path::PathBuf;
use csv::Writer;
use crate::ledger::entry::Entry;
use rusqlite::Error as RusqliteError;  // Alias
use rusqlite::Result;

pub fn export_to_csv(entries: &Vec<Entry>, path: PathBuf) -> Result<()> {
    let file = File::create(path)
        .map_err(|e| RusqliteError::InvalidParameterName(e.to_string()))?;  // Convert IO error

    let mut writer = Writer::from_writer(file);

    // Write header row
    writer.write_record(&["id", "date", "amount", "description", "category", "hash"])
        .map_err(|e| RusqliteError::InvalidParameterName(e.to_string()))?;  // Convert CSV error

    // Write each entry as a row
    for entry in entries {
        writer.write_record(&[
            entry.id.map_or("".to_string(), |id| id.to_string()),
            entry.date.to_string(),
            entry.amount.to_string(),
            entry.description.clone(),
            entry.category.clone().unwrap_or("".to_string()),
            entry.hash.clone().unwrap_or("".to_string()),
        ])
        .map_err(|e| RusqliteError::InvalidParameterName(e.to_string()))?;  // Convert per-row error
    }

    writer.flush()
        .map_err(|e| RusqliteError::InvalidParameterName(e.to_string()))?;  // Convert flush error

    Ok(())
}