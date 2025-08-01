use std::fs::File;
use std::path::PathBuf;
use csv::Writer;
use crate::ledger::entry::Entry;
use rusqlite::Result;

pub fn export_to_csv(entries: &Vec<Entry>, path: PathBuf) -> Result<()> {
    let file = File::create(path)?;  // Create the file
    let mut writer = Writer::from_writer(file);  // CSV writer

    // Write header row
    writer.write_record(&["id", "date", "amount", "description", "category", "hash"])?;

    // Write each entry as a row
    for entry in entries {
        writer.write_record(&[
            entry.id.map_or("".to_string(), |id| id.to_string()),
            entry.date.to_string(),
            entry.amount.to_string(),
            entry.description.clone(),
            entry.category.clone().unwrap_or("".to_string()),
            entry.hash.clone().unwrap_or("".to_string()),
        ])?;
    }

    writer.flush()?;  // Save changes
    Ok(())
}