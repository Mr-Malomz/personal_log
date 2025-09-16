use rusqlite::{Connection, Result};
use std::path::Path;

pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn new() -> Self {
        let conn = Connection::open_in_memory().expect("Failed to open database");
        DB { conn }
    }
    
    pub fn new_file(path: &str) -> Result<Self> {
        let conn = Connection::open(Path::new(path))?;
        Ok(DB { conn })
    }

    pub fn initialize(&self) -> Result<()> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS entries (
                id INTEGER PRIMARY KEY,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
                [],
            )?;
        Ok(())
    }

    pub fn create_entry(&self, content: &str) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        self.conn
            .execute(
                "INSERT INTO entries (content, created_at) VALUES (?1, ?2)",
                &[content, &now],
            )?;
        Ok(())
    }

    pub fn get_entries(&self) -> Result<Vec<crate::models::data::Entry>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, content, created_at FROM entries ORDER BY id")?;
        let entry_iter = stmt
            .query_map([], |row| {
                Ok(crate::models::data::Entry {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    created_at: row.get(2)?,
                })
            })?;

        let mut entries = Vec::new();
        for entry in entry_iter {
            entries.push(entry?);
        }
        Ok(entries)
    }

    pub fn delete_entry(&self, id: i32) -> Result<()> {
        self.conn
            .execute("DELETE FROM entries WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn update_entry(&self, id: i32, new_content: &str) -> Result<()> {
        self.conn
            .execute(
                "UPDATE entries SET content = ?1 WHERE id = ?2",
                rusqlite::params![new_content, id],
            )?;
        Ok(())
    }

    pub fn search_entries(&self, keyword: &str) -> Result<Vec<crate::models::data::Entry>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, content, created_at FROM entries WHERE content LIKE ?1 ORDER BY id")?;
        let pattern = format!("%{}%", keyword);
        let entry_iter = stmt
            .query_map([pattern], |row| {
                Ok(crate::models::data::Entry {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    created_at: row.get(2)?,
                })
            })?;

        let mut entries = Vec::new();
        for entry in entry_iter {
            entries.push(entry?);
        }
        Ok(entries)
    }
}
