use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShinyLogEntry {
    pub pokemon_name: String,
    pub form: String,
    pub date: String, // Keep this as String for JSON serialization
    pub details: String,
}

/// Logs a shiny capture to the specified log file.
pub fn log_shiny_capture(log_path: &str, entry: &ShinyLogEntry) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;

    // If the file is empty, initialize it with an opening bracket
    if file.metadata()?.len() == 0 {
        writeln!(file, "[]")?; // Create an empty JSON array
    }

    // Now read the existing content to modify it
    let data = fs::read_to_string(log_path)?;
    let mut entries: Vec<ShinyLogEntry> = serde_json::from_str(&data)?;

    // Add the new entry
    entries.push(entry.clone()); // Add the new entry

    // Write back the updated JSON array
    let updated_data = serde_json::to_string_pretty(&entries)?;
    fs::write(log_path, updated_data)?;

    Ok(())
}

/// Loads shiny log entries from the log file.
pub fn load_shiny_log(log_path: &str) -> io::Result<Vec<ShinyLogEntry>> {
    let data = fs::read_to_string(log_path)?;
    let entries: Vec<ShinyLogEntry> = serde_json::from_str(&data)?;
    Ok(entries)
}
