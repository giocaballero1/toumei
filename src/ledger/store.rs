use rusqlite::{Connection, Result};
use std::path::PathBuf;
use chrono::NaiveDateTime;
use rust_decimal::prelude::*;  // This brings in Decimal::from_str and other helpers
use super::entry::Entry;  // Pulls in Entry from entry.rs

pub struct LedgerStore {
    conn: Connection,  // Holds the DB connection
}

impl LedgerStore {
    // New: Opens (or creates) the DB file
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }

    // Init schema: Creates the table if it doesn't exist
    pub fn init_schema(&mut self) -> Result<()> {
        super::schema::migrate(&mut self.conn)
    }
    // Add an entry to the DB
    pub fn add_entry(&mut self, entry: &Entry) -> Result<i64> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO entries (date, amount, description, category, hash)
             VALUES (?, ?, ?, ?, ?)",
        )?;
        let id = stmt.insert((
            entry.date.to_string(),  // Store as string
            entry.amount.to_string(),
            &entry.description,
            entry.category.as_ref(),  // Handles Option
            entry.hash.as_ref(),
        ))?;
        Ok(id)  // Returns the new ID
    }

    // Get all entries from the DB
    pub fn get_all_entries(&self) -> Result<Vec<Entry>> {
        let mut stmt = self.conn.prepare("SELECT * FROM entries")?;
        let entries_iter = stmt.query_map([], |row| -> Result<Entry> {
            let date_str: String = row.get(1)?;
            let amount_str: String = row.get(2)?;

            let date = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S")
                .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

            let amount = Decimal::from_str(&amount_str)
                .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

            Ok(Entry {
                id: Some(row.get(0)?),
                date,
                amount,
                description: row.get(3)?,
                category: row.get(4)?,
                hash: row.get(5)?,
            })
        })?;

        let mut entries = Vec::new();
        for entry_result in entries_iter {
            entries.push(entry_result?);
        }
        Ok(entries)
    }
    // Update an entry by ID (e.g., change amount or description)
    pub fn update_entry(&mut self, id: i64, new_entry: &Entry) -> Result<()> {
        let mut stmt = self.conn.prepare(
            "UPDATE entries SET date = ?, amount = ?, description = ?, category = ?, hash = ? WHERE id = ?",
        )?;
        stmt.execute((
            new_entry.date.to_string(),
            new_entry.amount.to_string(),
            &new_entry.description,
            new_entry.category.as_ref(),
            new_entry.hash.as_ref(),
            id,
        ))?;
        Ok(())
    }
    // Delete an entry by ID
    pub fn delete_entry(&mut self, id: i64) -> Result<()> {
        let mut stmt = self.conn.prepare("DELETE FROM entries WHERE id = ?")?;
        stmt.execute([id])?;
        Ok(())
    }
}