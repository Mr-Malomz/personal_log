use rusqlite::Connection;

pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn new() -> Self {
        let conn = Connection::open_in_memory().expect("Failed to open database");
        DB { conn }
    }

    pub fn initialize(&self) {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS entries (
                id INTEGER PRIMARY KEY,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
                [],
            )
            .expect("Failed to create table");
    }

    pub fn create_entry(&self, content: &str) {
        let now = chrono::Utc::now().to_rfc3339();
        self.conn
            .execute(
                "INSERT INTO entries (content, created_at) VALUES (?1, ?2)",
                &[content, &now],
            )
            .expect("Failed to insert entry");
    }

    pub fn get_entries(&self) -> Vec<crate::models::data::Entry> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, content, created_at FROM entries")
            .expect("Failed to prepare statement");
        let entry_iter = stmt
            .query_map([], |row| {
                Ok(crate::models::data::Entry {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    created_at: row.get(2)?,
                })
            })
            .expect("Failed to query entries");

        entry_iter
            .map(|e| e.expect("Failed to map entry"))
            .collect()
    }

    pub fn delete_entry(&self, id: i32) {
        self.conn
            .execute("DELETE FROM entries WHERE id = ?1", [id])
            .expect("Failed to delete entry");
    }

    pub fn update_entry(&self, id: i32, new_content: &str) {
        self.conn
            .execute(
                "UPDATE entries SET content = ?1 WHERE id = ?2",
                rusqlite::params![new_content, id],
            )
            .expect("Failed to update entry");
    }

    pub fn search_entries(&self, keyword: &str) -> Vec<crate::models::data::Entry> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, content, created_at FROM entries WHERE content LIKE ?1")
            .expect("Failed to prepare statement");
        let pattern = format!("%{}%", keyword);
        let entry_iter = stmt
            .query_map([pattern], |row| {
                Ok(crate::models::data::Entry {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    created_at: row.get(2)?,
                })
            })
            .expect("Failed to query entries");

        entry_iter
            .map(|e| e.expect("Failed to map entry"))
            .collect()
    }
}
