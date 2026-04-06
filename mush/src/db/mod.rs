use std::collections::HashMap;
use std::path::Path;

use color_eyre::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Row, SqlitePool};
use tokio::runtime::{Builder, Runtime};

use crate::shell::help_parser::CommandOption;

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

            sqlx::query(
                "CREATE TABLE IF NOT EXISTS command_help (
                    command_prefix TEXT PRIMARY KEY,
                    options_json   TEXT NOT NULL,
                    help_hash      TEXT NOT NULL,
                    updated_at     TEXT NOT NULL DEFAULT (datetime('now'))
                )",
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

    pub fn get_help_hash(&self, command_prefix: &str) -> Result<Option<String>> {
        self.rt.block_on(async {
            let row = sqlx::query(
                "SELECT help_hash FROM command_help WHERE command_prefix = ?1",
            )
            .bind(command_prefix)
            .fetch_optional(&self.pool)
            .await?;

            Ok(row.map(|r| r.get("help_hash")))
        })
    }

    pub fn upsert_help(
        &self,
        command_prefix: &str,
        options: &[CommandOption],
        help_hash: &str,
    ) -> Result<()> {
        self.rt.block_on(async {
            let json = serde_json::to_string(options)
                .map_err(|e| color_eyre::eyre::eyre!("JSON serialize error: {e}"))?;
            sqlx::query(
                "INSERT INTO command_help (command_prefix, options_json, help_hash)
                 VALUES (?1, ?2, ?3)
                 ON CONFLICT(command_prefix) DO UPDATE SET
                    options_json = excluded.options_json,
                    help_hash = excluded.help_hash,
                    updated_at = datetime('now')",
            )
            .bind(command_prefix)
            .bind(&json)
            .bind(help_hash)
            .execute(&self.pool)
            .await?;
            Ok(())
        })
    }

    pub fn load_all_help(&self) -> Result<HashMap<String, Vec<CommandOption>>> {
        self.rt.block_on(async {
            let rows = sqlx::query("SELECT command_prefix, options_json FROM command_help")
                .fetch_all(&self.pool)
                .await?;

            let mut map = HashMap::new();
            for row in rows {
                let prefix: String = row.get("command_prefix");
                let json: String = row.get("options_json");
                if let Ok(options) = serde_json::from_str::<Vec<CommandOption>>(&json) {
                    map.insert(prefix, options);
                }
            }
            Ok(map)
        })
    }
}
