mod ledger;  // Imports the ledger module (entry.rs and store.rs)
use ledger::export::json::export_to_json;
use ledger::export::csv::export_to_csv;
use ledger::entry::Entry;
use ledger::store::LedgerStore;
use chrono::NaiveDateTime;
use rust_decimal::prelude::*;  // For Decimal::from_f64
use std::env;  // For home dir path
use std::path::PathBuf;  // For file paths
use rusqlite::Result;  // For error handling

fn main() -> Result<()> {
    // Set up DB path: Your Mac home dir + toumei.db (e.g., /Users/yourname/toumei.db)
    let db_path = PathBuf::from(env::var("HOME").unwrap()).join("toumei.db");

    // Create the store (opens/creates the DB)
    let mut store = LedgerStore::new(db_path)?;
    store.init_schema()?;  // Sets up the table if needed

    // Create an entry (your existing code)
    let date = NaiveDateTime::parse_from_str("2025-07-31 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let amount = Decimal::from_f64(100.50).unwrap();
    let entry = Entry::new(date, amount, "Salary".to_string(), Some("Income".to_string()));

    // Add it to the DB
    let id = store.add_entry(&entry)?;
    println!("Added entry with ID: {}", id);

    // Get and print all entries
    let all_entries = store.get_all_entries()?;
    println!("All entries: {:?}", all_entries);
    // Test update: Change the amount of the first entry (ID 1, if it exists)
    if let Some(first_entry) = all_entries.first() {
        let mut updated_entry = first_entry.clone();  // Make a copy to edit
        updated_entry.amount = Decimal::from_f64(200.00).unwrap();  // Change to $200 (test value)
        store.update_entry(first_entry.id.unwrap(), &updated_entry)?;
        println!("Updated entry ID {} to new amount: {}", first_entry.id.unwrap(), updated_entry.amount);
    }

    // Test delete: Remove the last entry (comment out if you don't want to delete right now)
    if let Some(last_entry) = all_entries.last() {
        store.delete_entry(last_entry.id.unwrap())?;
        println!("Deleted entry ID {}", last_entry.id.unwrap());
    }

    // Print the final list to see changes
    let final_entries = store.get_all_entries()?;
    println!("Final entries after changes: {:?}", final_entries);
    // Test exports
    let json_path = PathBuf::from("toumei.json");
    export_to_json(&final_entries, json_path)?;

    let csv_path = PathBuf::from("toumei.csv");
    export_to_csv(&final_entries, csv_path)?;

    println!("Exported to toumei.json and toumei.csv");
    Ok(())  // Returns success
}