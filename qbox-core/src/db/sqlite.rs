use anyhow::Result;

use rusqlite::{Connection, OpenFlags};
use std::path::Path;

const DB_NAME: &str = "qbox.db";

pub(crate) fn init() -> Result<()> {
    let db = opendb(false)?;
    create_table(&db)?;
    Ok(())
}

fn create_table(db: &Connection) -> Result<()> {
    db.execute_batch(
        "
        BEGIN;

        CREATE TABLE IF NOT EXISTS qbox (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
        );

        CREATE TABLE IF NOT EXISTS units (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            passwd TEXT NOT NULL,
            created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS daemos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            pid INTEGER,
            kind TEXT NOT NULL, 
            name TEXT NOT NULL,
            uri TEXT NOT NULL,
            state TEXT NOT NULL,
            created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS brokers (
            name TEXT NOT NULL PRIMARY KEY, 
            items TEXT NOT NULL,
            remark TEXT NOT NULL,
            created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        
        CREATE TABLE IF NOT EXISTS unit_instruments (
            unit TEXT PRIMARY KEY,
            group TEXT PRIMARY KEY, 
            security_id TEXT PRIMARY KEY,
            quota REAL,
            created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS instruments (
            security_id TEXT PRIMARY KEY,
            exchange TEXT,
            symbol TEXT,
            kind TEXT NOT NULL,
            base_currency TEXT NOT NULL,
            quote_currency TEXT NOT NULL,
            multiplier INTEGER NOT NULL DEFAULT 1,
            state TEXT NOT NULL,
            items TEXT,
            created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS quote_level1 (
            security_id TEXT PRIMARY KEY,
            exchange TEXT,
            time INTEGER NOT NULL,
            avg REAL,
            open REAL,
            high REAL,
            low REAL,
            close REAL,
            last REAL,
            last_volum REAL,
            ask1 REAL,
            bid1 REAL,
            ask1_volume REAL,
            bid1_volume REAL,
            volume REAL,
            turnover REAL,
            score REAL
        );

        CREATE TABLE IF NOT EXISTS quote_level2 (
            security_id TEXT PRIMARY KEY,
            exchange TEXT,
            time INTEGER NOT NULL,
            asks TEXT NOT NULL,
            bids TEXT NOT NULL,
            created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        
        CREATE TABLE IF NOT EXISTS orders (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            out_id TEXT UNIQUE INDEX,
            security_id TEXT NOT NULL INDEX,
            exchange TEXT NOT NULL,
            kind TEXT NOT NULL,
            side TEXT NOT NULL,
            offset TEXT NOT NULL,
            price REAL,
            quantity REAL,
            multiplier INTEGER DEFAULT 1,
            pov TEXT NOT NULL DEFAULT 'GTC',
            filled_quantity REAL,
            filled_amount REAL,
            avg_price REAL,
            last_filled_time INTEGER NOT NULL,
            items TEXT,
            remark TEXT,
            owner TEXT INDEX,
            strategy TEXT INDEX,
            state TEXT NOT NULL DEFAULT 'Created',
            created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            order_id TEXT NOT NULL INDEX,
            out_id TEXT INDEX,
            security_id TEXT NOT NULL INDEX,
            exchange TEXT NOT NULL,
            side TEXT NOT NULL,
            into_side TEXT,
            price REAL,
            quantity REAL,
            ask_order_id TEXT,
            bid_order_id TEXT,
            created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS positions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            pid INTEGER,
            order_idINTEGER,
            security_id TEXT NOT NULL UNIQUE INDEX,
            exchange TEXT NOT NULL,
            side TEXT NOT NULL,
            offset TEXT NOT NULL,
            quantity REAL,
            frozen REAL,
            last REAL,
            average REAL,
            settlement  REAL,
            cost REAL,
            margin REAL,
            realized_pnl REAL,
            unrealized_pnl REAL,
            position_pnl REAL,
            created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            unit TEXT NOT NULL UNIQUE INDEX,
            account TEXT NOT NULL UNIQUE INDEX,
            amount REAL,
            associate INTEGER,
            created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS ledgers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            account_id INTEGER NOT NULL,
            side TEXT NOT NULL,
            opcode TEXT NOT NULL,
            amount REAL,
            use TEXT NOT NULL,
            created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        COMMIT;
    ",
    )?;
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

pub fn opendb(read_only: bool) -> Result<Connection> {
    let db_path = Path::new(crate::get_log_path().as_str()).join(DB_NAME);

    let db = if !read_only {
        Connection::open_with_flags(
            db_path,
            OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_SHARED_CACHE,
        )?
    } else {
        Connection::open_with_flags(
            db_path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | { OpenFlags::SQLITE_OPEN_SHARED_CACHE },
        )?
    };
    Ok(db)
}
