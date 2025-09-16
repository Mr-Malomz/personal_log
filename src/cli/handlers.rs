use crate::models::db::DB;
use std::error::Error;
use std::fmt::{self, Debug};

// Custom error type for CLI operations
#[derive(Debug)]
pub enum CliError {
    DatabaseError(String),
    InvalidInput(String),
    ExportError(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CliError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            CliError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            CliError::ExportError(msg) => write!(f, "Export error: {}", msg),
        }
    }
}

impl Error for CliError {}

// Add a new entry
pub fn add_entry(db: &DB, content: &str, verbose: bool) -> Result<(), CliError> {
    if content.trim().is_empty() {
        return Err(CliError::InvalidInput(
            "Entry content cannot be empty".to_string(),
        ));
    }

    db.create_entry(content)
        .map_err(|e| CliError::DatabaseError(e.to_string()))?;

    if verbose {
        println!("Entry added successfully.");
    }

    Ok(())
}

// List all entries
pub fn list_entries(
    db: &DB,
    limit: Option<usize>,
    reverse: bool,
    _verbose: bool,
) -> Result<(), CliError> {
    let mut entries = db
        .get_entries()
        .map_err(|e| CliError::DatabaseError(e.to_string()))?;

    if reverse {
        entries.reverse();
    }

    let entries = match limit {
        Some(n) => entries.into_iter().take(n).collect::<Vec<_>>(),
        None => entries,
    };

    if entries.is_empty() {
        println!("No entries found.");
        return Ok(());
    }

    println!("{:<5} {:<20} {}", "ID", "TIMESTAMP", "CONTENT");
    println!("{:-<5} {:-<20} {:-<50}", "", "", "");

    for entry in entries {
        // Format timestamp to be more readable (assuming ISO format)
        let timestamp = if entry.created_at.len() > 19 {
            &entry.created_at[0..19].replace("T", " ")
        } else {
            &entry.created_at
        };

        println!("{:<5} {:<20} {}", entry.id, timestamp, entry.content);
    }

    Ok(())
}

// Search entries by keyword
pub fn search_entries(db: &DB, keyword: &str, verbose: bool) -> Result<(), CliError> {
    if keyword.trim().is_empty() {
        return Err(CliError::InvalidInput(
            "Search keyword cannot be empty".to_string(),
        ));
    }

    let entries = db
        .search_entries(keyword)
        .map_err(|e| CliError::DatabaseError(e.to_string()))?;

    if entries.is_empty() {
        println!("No entries found matching '{}'.", keyword);
        return Ok(());
    }

    if verbose {
        println!("Found {} entries matching '{}':", entries.len(), keyword);
    }

    println!("{:<5} {:<20} {}", "ID", "TIMESTAMP", "CONTENT");
    println!("{:-<5} {:-<20} {:-<50}", "", "", "");

    for entry in entries {
        // Format timestamp to be more readable
        let timestamp = if entry.created_at.len() > 19 {
            &entry.created_at[0..19].replace("T", " ")
        } else {
            &entry.created_at
        };

        println!("{:<5} {:<20} {}", entry.id, timestamp, entry.content);
    }

    Ok(())
}

// Update an existing entry
pub fn update_entry(db: &DB, id: i32, content: &str, verbose: bool) -> Result<(), CliError> {
    if content.trim().is_empty() {
        return Err(CliError::InvalidInput(
            "New content cannot be empty".to_string(),
        ));
    }

    db.update_entry(id, content)
        .map_err(|e| CliError::DatabaseError(e.to_string()))?;

    if verbose {
        println!("Entry #{} updated successfully.", id);
    }

    Ok(())
}

// Delete an entry
pub fn delete_entry(db: &DB, id: i32, force: bool, verbose: bool) -> Result<(), CliError> {
    if !force {
        println!("Deleting entry #{}. Use --force to skip this message.", id);
    }

    db.delete_entry(id)
        .map_err(|e| CliError::DatabaseError(e.to_string()))?;

    if verbose {
        println!("Entry #{} deleted successfully.", id);
    }

    Ok(())
}

// Export entries to CSV
pub fn export_entries(
    db: &DB,
    output_path: &str,
    from_date: Option<String>,
    to_date: Option<String>,
    verbose: bool,
) -> Result<(), CliError> {
    let entries = db
        .get_entries()
        .map_err(|e| CliError::DatabaseError(e.to_string()))?;

    // Filter entries by date if specified
    let filtered_entries = entries
        .into_iter()
        .filter(|entry| {
            let matches_from = match &from_date {
                Some(date) => entry.created_at.starts_with(date),
                None => true,
            };

            let matches_to = match &to_date {
                Some(date) => entry.created_at.starts_with(date),
                None => true,
            };

            matches_from && matches_to
        })
        .collect::<Vec<_>>();

    if filtered_entries.is_empty() {
        return Err(CliError::ExportError(
            "No entries found to export".to_string(),
        ));
    }

    // Create CSV file
    let file = std::fs::File::create(output_path)
        .map_err(|e| CliError::ExportError(format!("Failed to create file: {}", e)))?;

    let mut wtr = csv::Writer::from_writer(file);

    // Write header
    wtr.write_record(&["ID", "CONTENT", "CREATED_AT"])
        .map_err(|e| CliError::ExportError(format!("Failed to write CSV header: {}", e)))?;

    // Write entries
    for entry in filtered_entries {
        wtr.write_record(&[entry.id.to_string(), entry.content, entry.created_at])
            .map_err(|e| CliError::ExportError(format!("Failed to write CSV record: {}", e)))?;
    }

    wtr.flush()
        .map_err(|e| CliError::ExportError(format!("Failed to flush CSV writer: {}", e)))?;

    if verbose {
        println!("Exported entries to {}", output_path);
    }

    Ok(())
}
