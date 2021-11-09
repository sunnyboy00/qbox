use crate::broker::{Depth, Exchange, InstState, Instrument, Level1, TradeKind};
use crate::Parameter;
use anyhow::Result;
use rusqlite::{params, Connection, OpenFlags};
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
    // let _ = db.close();
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
    // let db_path = Path::new(crate::data_path().as_str()).join(DB_NAME);
    // let db = Connection::open_with_flags(
    //     db_path,
    //     OpenFlags::SQLITE_OPEN_CREATE
    //         | OpenFlags::SQLITE_OPEN_READ_WRITE
    //         | OpenFlags::SQLITE_OPEN_SHARED_CACHE,
    // )?;
    let db = Connection::open_in_memory_with_flags(
        OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_CREATE
            | OpenFlags::SQLITE_OPEN_SHARED_CACHE
            | OpenFlags::SQLITE_OPEN_NO_MUTEX
            | OpenFlags::SQLITE_OPEN_URI,
    )?;
    Ok(db)
}

pub fn opendb_read_only() -> Result<Connection> {
    // let db_path = Path::new(crate::data_path().as_str()).join(DB_NAME);
    // let db = Connection::open_with_flags(
    //     db_path,
    //     OpenFlags::SQLITE_OPEN_READ_ONLY
    //         | OpenFlags::SQLITE_OPEN_NO_MUTEX
    //         | OpenFlags::SQLITE_OPEN_SHARED_CACHE,
    // )?;
    let db = Connection::open_in_memory_with_flags(
        OpenFlags::SQLITE_OPEN_READ_ONLY
            | OpenFlags::SQLITE_OPEN_NO_MUTEX
            | OpenFlags::SQLITE_OPEN_SHARED_CACHE
            | OpenFlags::SQLITE_OPEN_URI,
    )?;
    Ok(db)
}

pub fn insert_instrument(db: &Connection, instr: &Instrument) -> Result<()> {
    const SQL: &str = r#"INSERT OR REPLACE INTO instruments (security_id,exchange,symbol,kind,base_currency,quote_currency,multiplier,state,items,updated_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,CURRENT_TIMESTAMP);"#;
    let exchange: &str = instr.exchange.into();
    let kind: &str = instr.kind.into();
    let state = format!("{:?}", instr.state);
    let items = ron::to_string(&instr.items)?;
    db.execute(
        SQL,
        params![
            instr.security_id,
            exchange,
            instr.symbol,
            kind,
            instr.base_currency,
            instr.quote_currency,
            instr.multiplier,
            state,
            items
        ],
    )?;
    Ok(())
}

pub fn find_all_instruments(db: &Connection) -> Result<Vec<Instrument>> {
    let mut ret = vec![];
    const SQL:&str = "SELECT security_id,exchange,symbol,kind,base_currency,quote_currency,multiplier,state,items FROM instruments;";
    {
        let mut stat = db.prepare(SQL)?;
        let list = stat.query_map([], |row| {
            let items: String = row.get(8)?;
            let exchange: String = row.get(1)?;
            let kind: String = row.get(3)?;
            let state: String = row.get(7)?;
            let items: Parameter = if let Ok(items) = ron::from_str::<Parameter>(&items) {
                items
            } else {
                Parameter::new()
            };
            Ok(Instrument {
                security_id: row.get(0)?,
                exchange: Exchange::from(&exchange),
                symbol: row.get(2)?,
                kind: TradeKind::from(kind.as_str()),
                base_currency: row.get(4)?,
                quote_currency: row.get(5)?,
                multiplier: row.get(6)?,
                state: InstState::from(state.as_str()),
                items,
            })
        })?;
        for instr in list {
            ret.push(instr?);
        }
    }
    Ok(ret)
}

pub fn update_level1(db: &Connection, level1: &Level1) -> Result<()> {
    const SQL_INSERT: &str = r#"INSERT INTO quote_level1 (
        security_id,
        exchange,
        time,
        avg,
        open,
        high,
        low,
        close,
        last,
        last_volume,
        asks,
        bids,
        volume,
        turnover,
        items,
        updated_at
    ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,CURRENT_TIMESTAMP);"#;
    const SQL_UPDATE: &str = r#"UPDATE quote_level1 SET
        time=?1,
        avg=?2,
        open=?3,
        high=?4,
        low=?5,
        close=?6,
        last=?7,
        last_volume=?8,
        asks=?9,
        bids=?10,
        volume=?11,
        turnover=?12,
        items=?13,
        updated_at=CURRENT_TIMESTAMP
    WHERE security_id=?14;"#;
    let exchange: &str = level1.exchange.into();
    let asks = ron::to_string(&level1.asks)?;
    let bids = ron::to_string(&level1.bids)?;
    let items = ron::to_string(&level1.items)?;
    let len = db.execute(
        SQL_UPDATE,
        params![
            level1.time,
            level1.average,
            level1.open,
            level1.high,
            level1.low,
            level1.close,
            level1.last,
            level1.last_volume,
            asks,
            bids,
            level1.volume,
            level1.turnover,
            items,
            level1.security_id,
        ],
    )?;
    if len == 0 {
        db.execute(
            SQL_INSERT,
            params![
                level1.security_id,
                exchange,
                level1.time,
                level1.average,
                level1.open,
                level1.high,
                level1.low,
                level1.close,
                level1.last,
                level1.last_volume,
                asks,
                bids,
                level1.volume,
                level1.turnover,
                items,
            ],
        )?;
    }

    Ok(())
}

pub fn find_all_level1(db: &Connection) -> Result<Vec<Level1>> {
    let mut ret = vec![];
    const SQL: &str = "SELECT 
        security_id,
        exchange,
        time,
        avg,
        open,
        high,
        low,
        close,
        last,
        last_volume,
        asks,
        bids,
        volume,
        turnover,
        items
    FROM 
        quote_level1 order by updated_at desc;";
    {
        let mut stat = db.prepare(SQL)?;
        let list = stat.query_map([], |row| {
            let items: Parameter = if let Ok(items) =
                ron::from_str::<Parameter>(&row.get::<_, String>(14).unwrap_or_default())
            {
                items
            } else {
                Parameter::new()
            };
            let asks =
                if let Ok(asks) = ron::from_str(&row.get::<_, String>(10).unwrap_or_default()) {
                    asks
                } else {
                    vec![]
                };
            let bids =
                if let Ok(bids) = ron::from_str(&row.get::<_, String>(11).unwrap_or_default()) {
                    bids
                } else {
                    vec![]
                };

            Ok(Level1::new()
                .with_secrity_id(row.get::<_, String>(0)?)
                .with_exchange(Exchange::from(&row.get::<_, String>(1)?))
                .with_time(row.get(2)?)
                .with_average(row.get(3).unwrap_or(f64::NAN))
                .with_open(row.get(4).unwrap_or(f64::NAN))
                .with_high(row.get(5).unwrap_or(f64::NAN))
                .with_low(row.get(6).unwrap_or(f64::NAN))
                .with_close(row.get(7).unwrap_or(f64::NAN))
                .with_last(row.get(8).unwrap_or(f64::NAN))
                .with_last_volume(row.get(9).unwrap_or(f64::NAN))
                .with_asks(asks)
                .with_bids(bids)
                .with_items(items)
                .with_volume(row.get(12).unwrap_or(f64::NAN))
                .with_turnover(row.get(13).unwrap_or(f64::NAN)))
        })?;
        for instr in list {
            ret.push(instr?);
        }
    }
    Ok(ret)
}
