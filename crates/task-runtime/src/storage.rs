//! SQLite persistence: one row per task holding the full snapshot JSON,
//! plus a key/value settings table. WAL mode.

use std::{path::Path, sync::Mutex};

use rusqlite::{params, Connection};

use crate::{RuntimeError, RuntimeSettings, TaskSnapshot};

pub(crate) struct Storage {
    connection: Mutex<Connection>,
}

impl Storage {
    pub fn open(path: &Path) -> Result<Self, RuntimeError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let connection = Connection::open(path)?;
        connection.execute_batch(
            "PRAGMA journal_mode=WAL;
             CREATE TABLE IF NOT EXISTS tasks (
               id TEXT PRIMARY KEY,
               status TEXT NOT NULL,
               snapshot_json TEXT NOT NULL,
               created_at INTEGER NOT NULL,
               updated_at INTEGER NOT NULL
             );
             CREATE INDEX IF NOT EXISTS idx_tasks_status_created
               ON tasks(status, created_at);
             CREATE TABLE IF NOT EXISTS settings (
               key TEXT PRIMARY KEY,
               value_json TEXT NOT NULL
             );",
        )?;
        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    pub fn upsert_task(&self, task: &TaskSnapshot) -> Result<(), RuntimeError> {
        let json = serde_json::to_string(task)?;
        self.connection.lock().expect("database mutex").execute(
            "INSERT INTO tasks(id, status, snapshot_json, created_at, updated_at)
             VALUES(?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(id) DO UPDATE SET
               status=excluded.status,
               snapshot_json=excluded.snapshot_json,
               updated_at=excluded.updated_at",
            params![
                task.id,
                task.status.name(),
                json,
                task.created_at,
                task.updated_at
            ],
        )?;
        Ok(())
    }

    pub fn delete_task(&self, id: &str) -> Result<(), RuntimeError> {
        self.connection
            .lock()
            .expect("database mutex")
            .execute("DELETE FROM tasks WHERE id=?1", [id])?;
        Ok(())
    }

    pub fn load_tasks(&self) -> Result<Vec<TaskSnapshot>, RuntimeError> {
        let connection = self.connection.lock().expect("database mutex");
        let mut statement =
            connection.prepare("SELECT snapshot_json FROM tasks ORDER BY created_at DESC")?;
        let rows = statement.query_map([], |row| row.get::<_, String>(0))?;
        let mut tasks = Vec::new();
        for row in rows {
            // a row from an older incompatible version must not brick startup
            match serde_json::from_str(&row?) {
                Ok(task) => tasks.push(task),
                Err(error) => eprintln!("task-runtime: skipping unreadable task row: {error}"),
            }
        }
        Ok(tasks)
    }

    pub fn save_settings(&self, value: &RuntimeSettings) -> Result<(), RuntimeError> {
        self.connection.lock().expect("database mutex").execute(
            "INSERT INTO settings(key, value_json) VALUES('runtime', ?1)
             ON CONFLICT(key) DO UPDATE SET value_json=excluded.value_json",
            [serde_json::to_string(value)?],
        )?;
        Ok(())
    }

    pub fn load_settings(&self) -> Result<RuntimeSettings, RuntimeError> {
        let connection = self.connection.lock().expect("database mutex");
        let result = connection.query_row(
            "SELECT value_json FROM settings WHERE key='runtime'",
            [],
            |row| row.get::<_, String>(0),
        );
        match result {
            Ok(value) => Ok(serde_json::from_str(&value)?),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(RuntimeSettings::default()),
            Err(error) => Err(error.into()),
        }
    }
}
