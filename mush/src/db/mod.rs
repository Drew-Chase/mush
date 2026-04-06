use std::path::Path;

use color_eyre::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Row, SqlitePool};
use tokio::runtime::{Builder, Runtime};

pub struct HistoryDb {
    pool: SqlitePool,
    rt: Runtime,
}

pub struct HistoryRecord {
    pub command: String,
    pub exit_code: i32,
}

impl HistoryDb {
    pub fn open(path: &Path) -> Result<Self> {
        let rt = Builder::new_current_thread().enable_all().build()?;
        let pool = rt.block_on(async {
            let opts = SqliteConnectOptions::new()
                .filename(path)
                .create_if_missing(true);

            let pool = SqlitePoolOptions::new()
                .max_connections(1)
                .connect_with(opts)
                .await?;

            sqlx::query(
                "CREATE TABLE IF NOT EXISTS command_history (
                    id          INTEGER PRIMARY KEY AUTOINCREMENT,
                    command     TEXT    NOT NULL,
                    exit_code   INTEGER NOT NULL DEFAULT 0,
                    duration_ms INTEGER NOT NULL DEFAULT 0,
                    cwd         TEXT,
                    created_at  TEXT    NOT NULL DEFAULT (datetime('now'))
                )",
            )
            .execute(&pool)
            .await?;

            sqlx::query(
                "CREATE INDEX IF NOT EXISTS idx_history_command ON command_history(command)",
            )
            .execute(&pool)
            .await?;

            sqlx::query(
                "CREATE INDEX IF NOT EXISTS idx_history_created ON command_history(created_at DESC)",
            )
            .execute(&pool)
            .await?;

            Ok::<SqlitePool, color_eyre::Report>(pool)
        })?;

        Ok(Self { pool, rt })
    }

    pub fn insert(
        &self,
        command: &str,
        exit_code: i32,
        duration_ms: i64,
        cwd: Option<&str>,
    ) -> Result<()> {
        self.rt.block_on(async {
            sqlx::query(
                "INSERT INTO command_history (command, exit_code, duration_ms, cwd) VALUES (?1, ?2, ?3, ?4)",
            )
            .bind(command)
            .bind(exit_code)
            .bind(duration_ms)
            .bind(cwd)
            .execute(&self.pool)
            .await?;
            Ok(())
        })
    }

    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<HistoryRecord>> {
        self.rt.block_on(async {
            let rows = if query.is_empty() {
                sqlx::query(
                    "SELECT command, exit_code, MAX(id) as last_id
                     FROM command_history
                     GROUP BY command
                     ORDER BY last_id DESC
                     LIMIT ?1",
                )
                .bind(limit as i64)
                .fetch_all(&self.pool)
                .await?
            } else {
                sqlx::query(
                    "SELECT command, exit_code, MAX(id) as last_id
                     FROM command_history
                     WHERE command LIKE '%' || ?1 || '%'
                     GROUP BY command
                     ORDER BY last_id DESC
                     LIMIT ?2",
                )
                .bind(query)
                .bind(limit as i64)
                .fetch_all(&self.pool)
                .await?
            };

            Ok(rows
                .iter()
                .map(|row| HistoryRecord {
                    command: row.get("command"),
                    exit_code: row.get("exit_code"),
                })
                .collect())
        })
    }
}
