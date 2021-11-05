use anyhow::Result;

use rusqlite::{Connection, OpenFlags};
use std::path::Path;

const DB_NAME: &str = "qbox.db";
const SCHEMA: &str = include_str!("schema.sql");

pub(crate) fn init() -> Result<()> {
    init_table()?;
    Ok(())
}

fn init_table() -> Result<()> {
    let db = opendb()?;
    db.execute_batch(SCHEMA)?;
    let _ = db.close();
    Ok(())
}

pub(crate) fn create_bar_table(db: &Connection, name: &str) -> Result<()> {
    db.execute_batch(
        format!(
            "
        BEGIN;
        CREATE TABLE IF NOT EXISTS quote_bar_{} (
            security_id TEXT PRIMARY KEY,
            time INTEGER NOT NULL,
            avg REAL,
            open REAL,
            high REAL,
            low REAL,
            close REAL,
            last REAL,
            volume REAL,
            turnover REAL
        );
        COMMIT;
    ",
            name
        )
        .as_str(),
    )?;
    Ok(())
}

pub(crate) fn opendb() -> Result<Connection> {
    let db_path = Path::new(crate::get_data_path().as_str()).join(DB_NAME);
    let db = Connection::open_with_flags(
        db_path,
        OpenFlags::SQLITE_OPEN_CREATE
            | OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_SHARED_CACHE,
    )?;
    Ok(db)
}

pub fn opendb_read_only() -> Result<Connection> {
    let db_path = Path::new(crate::get_data_path().as_str()).join(DB_NAME);
    let db = Connection::open_with_flags(
        db_path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | { OpenFlags::SQLITE_OPEN_SHARED_CACHE },
    )?;
    Ok(db)
}
