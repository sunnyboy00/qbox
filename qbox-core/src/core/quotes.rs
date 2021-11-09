use crate::broker::{
    Bar, Instrument, Level1, Level2, QuoteEvent, TickToOffer, TickToTrade, TradeEvent,
};
use crate::core::{self, *};
use crate::db::sqlite;
use ahash::RandomState;
use anyhow::Result;
use crossbeam::channel::{self, Receiver};
use dashmap::DashMap;
use lazy_static::lazy_static;
use std::sync::Arc;

const MAX_BAR_SIZE: usize = 1000;
const MAX_TTO_SIZE: usize = 100;
const MAX_TTT_SIZE: usize = 100;

lazy_static! {
    //图表
    static ref BARS: DashMap<String, Vec<Bar>,RandomState> = DashMap::with_hasher(RandomState::new());
    //LEVEL1行情
    static ref LEVEL1S: DashMap<String, Level1,RandomState> = DashMap::with_hasher(RandomState::new());
    //深度行情
    static ref DEPTHS: DashMap<String, Level2,RandomState> = DashMap::with_hasher(RandomState::new());
    //逐笔委托
    static ref TTOS: DashMap<String, Vec<TickToOffer>,RandomState> = DashMap::with_hasher(RandomState::new());
    //逐笔成交
    static ref TTTS: DashMap<String, Vec<TickToTrade>,RandomState> = DashMap::with_hasher(RandomState::new());
    //证券列表
    static ref INSTRUMENTS: DashMap<String, Instrument,RandomState> = DashMap::with_hasher(RandomState::new());
}

pub fn get_bar(security_id: &String) -> Option<Vec<Bar>> {
    if let Some(bars) = BARS.get(security_id) {
        Some(bars.value().clone())
    } else {
        None
    }
}

pub fn get_tick2offer(security_id: &String) -> Option<Vec<TickToOffer>> {
    if let Some(ttos) = TTOS.get(security_id) {
        Some(ttos.value().clone())
    } else {
        None
    }
}

pub fn get_tick2trade(security_id: &String) -> Option<Vec<TickToTrade>> {
    if let Some(ttts) = TTTS.get(security_id) {
        Some(ttts.value().clone())
    } else {
        None
    }
}

pub fn get_level1(security_id: &String) -> Option<Level1> {
    if let Some(level1) = LEVEL1S.get(security_id) {
        Some(level1.value().clone())
    } else {
        None
    }
}

pub fn get_all_level1() -> Option<Vec<Level1>> {
    log::trace!("get_all_level1",);

    let data: Vec<Level1> = LEVEL1S.iter().map(|item| item.value().clone()).collect();
    if data.len() > 0 {
        Some(data)
    } else {
        None
    }
}

pub fn find_level1_with_prefix(prefix: &str) -> Option<Vec<Level1>> {
    let data: Vec<Level1> = LEVEL1S
        .iter()
        .filter(|item| item.key().starts_with(prefix))
        .map(|item| item.value().clone())
        .collect();
    if data.len() > 0 {
        Some(data)
    } else {
        None
    }
}

pub fn find_level1_with_prefixs(prefixs: &[&str]) -> Option<Vec<Level1>> {
    let data: Vec<Vec<Level1>> = prefixs
        .iter()
        .filter_map(|prefix| find_level1_with_prefix(prefix))
        .collect();
    if data.len() > 0 {
        let data = data.concat();
        Some(data)
    } else {
        None
    }
}

pub fn get_instrument(security_id: &String) -> Option<Instrument> {
    if let Some(level1) = INSTRUMENTS.get(security_id) {
        Some(level1.value().clone())
    } else {
        None
    }
}

pub fn get_all_instrument() -> Option<Vec<Instrument>> {
    let data: Vec<Instrument> = INSTRUMENTS.iter().map(|v| v.value().clone()).collect();
    if data.len() > 0 {
        Some(data)
    } else {
        None
    }
}

pub fn find_instrument_with_prefix(prefix: &str) -> Option<Vec<Instrument>> {
    let data: Vec<Instrument> = INSTRUMENTS
        .iter()
        .filter(|v| v.key().starts_with(prefix))
        .map(|v| v.value().clone())
        .collect();
    if data.len() > 0 {
        Some(data)
    } else {
        None
    }
}

pub fn find_instrument_with_prefixs(prefixs: &[&str]) -> Option<Vec<Instrument>> {
    let data: Vec<Vec<Instrument>> = prefixs
        .iter()
        .filter_map(|prefix| find_instrument_with_prefix(prefix))
        .collect();
    if data.len() > 0 {
        Some(data.concat())
    } else {
        None
    }
}

pub(crate) fn init() -> Result<()> {
    let (tx, rx) = channel::bounded(8192);
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    quote_worker(rx)?;
    let _ = core::subscribe(topics::QUOTES_EVENT, move |_, ev| {
        match ev.as_ref() {
            Event::Quote(quote) => match quote {
                QuoteEvent::Level1(level1) => {
                    {
                        LEVEL1S.insert(level1.security_id.clone(), level1.clone());
                    }
                    let bar = level1.to_bar();
                    if let Some(mut bars) = BARS.get_mut(&bar.security_id) {
                        bars.value_mut().push(bar.clone());
                        if bars.len() > MAX_BAR_SIZE {
                            bars.remove(0);
                        }
                    } else {
                        let mut bars = Vec::with_capacity(MAX_BAR_SIZE);
                        bars.push(bar.clone());
                        BARS.insert(bar.security_id.clone(), bars);
                    }
                    tx1.send(Event::Quote(QuoteEvent::Bar(bar)).arced()).ok();
                }
                QuoteEvent::Bar(bar) => {
                    if let Some(mut bars) = BARS.get_mut(&bar.security_id) {
                        bars.value_mut().push(bar.clone());
                        if bars.len() > MAX_BAR_SIZE {
                            bars.remove(0);
                        }
                    } else {
                        let mut bars = Vec::with_capacity(MAX_BAR_SIZE);
                        bars.push(bar.clone());
                        BARS.insert(bar.security_id.clone(), bars);
                    }
                }
                QuoteEvent::Level2(level2) => {
                    DEPTHS.insert(level2.security_id.clone(), level2.clone());
                }
                QuoteEvent::TickToOffer(tto) => {
                    if let Some(mut ttos) = TTOS.get_mut(&tto.security_id) {
                        ttos.value_mut().push(tto.clone());
                        if ttos.len() > MAX_TTO_SIZE {
                            ttos.remove(0);
                        }
                    } else {
                        let mut ttos = Vec::with_capacity(MAX_TTO_SIZE);
                        ttos.push(tto.clone());
                        TTOS.insert(tto.security_id.clone(), ttos);
                    }
                }
                QuoteEvent::TickToTrade(ttt) => {
                    if let Some(mut ttts) = TTTS.get_mut(&ttt.security_id) {
                        ttts.value_mut().push(ttt.clone());
                        if ttts.len() > MAX_TTT_SIZE {
                            ttts.remove(0);
                        }
                    } else {
                        let mut ttts = Vec::with_capacity(MAX_TTT_SIZE);
                        ttts.push(ttt.clone());
                        TTTS.insert(ttt.security_id.clone(), ttts);
                    }
                }
            },
            _ => {}
        }
        // tx1.send(ev).ok();
    })?;
    let _ = core::subscribe(topics::QUERY_EVENT, move |_, ev| {
        match ev.as_ref() {
            Event::Trade(TradeEvent::InstrumentsResponse(instr)) => {
                INSTRUMENTS.insert(instr.security_id.clone(), instr.clone());
            }
            _ => {}
        }
        tx2.send(ev).ok();
    })?;
    Ok(())
}

fn quote_worker(rx: Receiver<Arc<Event>>) -> Result<()> {
    let db = sqlite::opendb()?;
    sqlite::find_all_instruments(&db)?.iter().for_each(|instr| {
        INSTRUMENTS.insert(instr.security_id.clone(), instr.clone());
    });
    std::thread::Builder::new()
        .name("qbox-quote-worker".into())
        .spawn(move || loop {
            match rx.recv() {
                Ok(ev) => {
                    log::trace!("process {:?}", ev);
                    match ev.as_ref() {
                        Event::Trade(TradeEvent::InstrumentsResponse(instr)) => {
                            if let Err(err) = sqlite::insert_instrument(&db, instr) {
                                log::error!("sqlite error {:?}", err);
                            }
                        }
                        _ => {}
                    }
                }
                Err(err) => {
                    log::error!("!!!!!!!!!! {:?}", err);
                }
            }
        })
        .ok();
    Ok(())
}
