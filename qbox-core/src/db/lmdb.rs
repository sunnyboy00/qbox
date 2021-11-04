use crate::bus::{self, topics, Event, Token};
use crate::counter::{Instrument, QuoteEvent, TradeEvent};
use anyhow::Result;
use crossbeam::channel;
use lazy_static::lazy_static;
use lmdb::{Database, Environment, WriteFlags};
use parking_lot::{Mutex, Once};
use std::path::Path;

lazy_static! {
    static ref DB: Environment = {
        let dir = Path::new(&crate::get_exec_path()).join("data");
        let lmdb = lmdb::Environment::new()
            .open_with_permissions(&dir, 0o777)
            .unwrap();

        let _ = lmdb
            .create_db(Some("quotes"), lmdb::DatabaseFlags::all())
            .unwrap();
        let _ = lmdb
            .create_db(Some("trades"), lmdb::DatabaseFlags::all())
            .unwrap();
        let _ = lmdb
            .create_db(Some("instruments"), lmdb::DatabaseFlags::all())
            .unwrap();
        lmdb
    };
}

//启动lmdb服务
pub fn start() -> Result<()> {
    let (tx, rx) = channel::unbounded();
    let dir = Path::new(&crate::get_exec_path()).join("data");
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }
    let quotes = DB.open_db(Some("quotes"))?;
    let trades = DB.open_db(Some("trades"))?;
    let instrs = DB.open_db(Some("instruments"))?;

    let topics = [
        topics::TRADES_EVENT,
        topics::QUOTES_EVENT,
        topics::QUERY_EVENT,
    ];
    let cb = move |_, msg| {
        let _ = tx.send(msg);
    };
    let tokens: Vec<Token> = topics
        .iter()
        .map(|topic| bus::subscribe(topic, cb.clone()).unwrap())
        .collect();

    std::thread::spawn(move || {
        if let Some(ids) = core_affinity::get_core_ids() {
            core_affinity::set_for_current(ids.last().unwrap().clone());
        }

        loop {
            if let Ok(ev) = rx.recv() {
                log::info!("{:?} {:?}", tokens, ev);
                match ev.as_ref() {
                    Event::Trade(ev) => match &ev {
                        TradeEvent::InstrumentsResponse(instr) => {
                            if let Err(err) = update_instrument(instrs.clone(), instr) {
                                log::error!("{:?} {:?} {}", instrs, instr, err);
                            }
                        }
                        _ => {
                            if let Err(err) = update_trades(trades.clone(), ev) {
                                log::error!("{:?} {:?} {}", trades, ev, err);
                            }
                        }
                    },
                    Event::Quote(ev) => {
                        if let Err(err) = update_quotes(quotes.clone(), ev) {
                            log::error!("{:?} {:?} {}", quotes, ev, err);
                        }
                    }
                    _ => {}
                }
            }
        }
    });
    Ok(())
}

fn update_instrument(table: Database, instr: &Instrument) -> Result<()> {
    if let Ok(mut rw) = DB.begin_rw_txn() {
        let data = serde_json::to_vec(instr)?;
        rw.put(table, &instr.security_id, &data, WriteFlags::all())?;
    }
    Ok(())
}

fn update_trades(table: Database, ev: &TradeEvent) -> Result<()> {
    if let Ok(mut rw) = DB.begin_rw_txn() {
        // let data = serde_json::to_vec(instr)?;
        // rw.put(table, &instr.security_id, &data, WriteFlags::all())?;
    }
    Ok(())
}
fn update_quotes(table: Database, ev: &QuoteEvent) -> Result<()> {
    if let Ok(mut rw) = DB.begin_rw_txn() {
        // let data = serde_json::to_vec(instr)?;
        // rw.put(table, &instr.security_id, &data, WriteFlags::all())?;
    }
    Ok(())
}
