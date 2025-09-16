use clap::{Parser, Subcommand};
use crate::models::db::DB;
use crate::cli::handlers::{CliError, add_entry, list_entries, search_entries, update_entry, delete_entry, export_entries};

// Function to create a database connection
pub fn create_db(database_path: Option<String>) -> Result<DB, CliError> {
    match database_path {
        Some(path) => DB::new_file(&path).map_err(|e| CliError::DatabaseError(e.to_string())),
        None => Ok(DB::new())
    }
}

// Handler for executing CLI commands
pub fn execute_command(cli: Cli) -> Result<(), CliError> {
    let db = create_db(cli.database)?;
    db.initialize().map_err(|e| CliError::DatabaseError(e.to_string()))?;
    
    match cli.command {
        Commands::Add { content } => add_entry(&db, &content, cli.verbose),
        Commands::List { limit, reverse } => list_entries(&db, limit, reverse, cli.verbose),
        Commands::Search { keyword } => search_entries(&db, &keyword, cli.verbose),
        Commands::Update { id, content } => update_entry(&db, id, &content, cli.verbose),
        Commands::Delete { id, force } => delete_entry(&db, id, force, cli.verbose),
        Commands::Export { output, from_date, to_date } => export_entries(&db, &output, from_date, to_date, cli.verbose),
    }
}


#[derive(Parser)]
#[command(name = "personal_log")]
#[command(about = "A CLI tool for managing personal log entries", long_about = None)]
#[command(version = "0.1.0")]
pub struct Cli {
    /// Database file path (defaults to in-memory database)
    #[arg(short, long, global = true)]
    pub database: Option<String>,
    
    /// Verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
    
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        #[arg(value_name = "CONTENT")]
        content: String,
    },

    
    List {
        #[arg(short, long)]
        limit: Option<usize>,
        #[arg(short, long)]
        reverse: bool,
    },

    Search {
        #[arg(value_name = "KEYWORD")]
        keyword: String,
    },

    Update {
        #[arg(value_name = "ID")]
        id: i32,
        #[arg(value_name = "CONTENT")]
        content: String,
    },
    
    Delete {
        #[arg(value_name = "ID")]
        id: i32,
        #[arg(short, long)]
        force: bool,
    },
    
    Export {
        #[arg(short, long, value_name = "FILE")]
        output: String,
        #[arg(long)]
        from_date: Option<String>,
        /// Export entries until a specific date (YYYY-MM-DD format)
        #[arg(long)]
        to_date: Option<String>,
    },
}

