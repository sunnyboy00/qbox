use crate::bus::{self, topics, Event, Topic};
use crate::counter::{
    Bar, Instrument, Level1, Level2, QuoteEvent, TickToOffer, TickToTrade, TradeEvent,
};
use anyhow::Result;
use dashmap::DashMap;
use lazy_static::lazy_static;
use std::sync::Arc;

const MAX_BAR_SIZE: usize = 1000;
const MAX_TTO_SIZE: usize = 100;
const MAX_TTT_SIZE: usize = 100;

lazy_static! {
    //图表
    static ref BARS: DashMap<String, Vec<Bar>> = DashMap::new();
    //LEVEL1行情
    static ref LEVEL1S: DashMap<String, Level1> = DashMap::new();
    //深度行情
    static ref DEPTHS: DashMap<String, Level2> = DashMap::new();
    //逐笔委托
    static ref TTOS: DashMap<String, Vec<TickToOffer>> = DashMap::new();
    //逐笔成交
    static ref TTTS: DashMap<String, Vec<TickToTrade>> = DashMap::new();
    //证券列表
    static ref INSTRUMENTS: DashMap<String, Instrument> = DashMap::new();
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
    //let map = LEVEL1S.read();
    let mut data: Vec<Level1> = LEVEL1S.iter().map(|item| item.value().clone()).collect();
    if data.len() > 0 {
        log::trace!("get_all_level1 {:?}", data);
        data.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        Some(data)
    } else {
        None
    }
}

pub fn find_level1_with_prefix(prefix: &str) -> Option<Vec<Level1>> {
    let mut data: Vec<Level1> = LEVEL1S
        .iter()
        .filter(|item| item.key().starts_with(prefix))
        .map(|item| item.value().clone())
        .collect();
    if data.len() > 0 {
        data.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
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
        let mut data = data.concat();
        data.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
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
    let work_dir = crate::get_exec_path();
    let path = std::path::Path::new(&work_dir).join("data");
    std::fs::create_dir_all(&path)?;
    if let Ok(b) = std::fs::read(path.join("symbols.txt")) {
        if let Ok(list) = ron::de::from_bytes::<Vec<Instrument>>(&b[..]) {
            list.into_iter().for_each(|instr| {
                INSTRUMENTS.insert(instr.security_id.clone(), instr);
            });
        }
    }

    let _ = bus::subscribe(topics::QUOTES_EVENT, process)?;
    let _ = bus::subscribe(topics::QUERY_EVENT, process)?;
    start_instrument_daemo();
    Ok(())
}

fn process(topic: Topic, ev: Arc<Event>) {
    log::trace!("process {:?}", ev);
    if let Event::Quote(quote) = ev.as_ref() {
        match quote {
            QuoteEvent::Level1(level1) => {
                {
                    // let mut map = LEVEL1S.write();
                    if let Some(old) = LEVEL1S.get(&level1.security_id) {
                        let mut l1 = old.value().clone();
                        l1.score += old.score;
                        LEVEL1S.insert(l1.security_id.clone(), l1);
                    } else {
                        LEVEL1S.insert(level1.security_id.clone(), level1.clone());
                    }
                }
                let bar = Bar {
                    security_id: level1.security_id.clone(),
                    exchange: level1.exchange,
                    time: level1.time,
                    open: level1.open,
                    high: level1.high,
                    low: level1.low,
                    close: if level1.close == f64::NAN {
                        level1.last
                    } else {
                        level1.close
                    },
                    volume: level1.last_quantity,
                    turnover: Some(level1.turnover),
                };
                bus::quotes_event(QuoteEvent::Bar(bar)).ok();
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
        }
    } else if let Event::Trade(TradeEvent::InstrumentsResponse(instr)) = ev.as_ref() {
        INSTRUMENTS.insert(instr.security_id.clone(), instr.clone());
    } else {
        log::warn!("!!!!!!!!!! {} {:?}", topic, ev)
    }
}

fn start_instrument_daemo() {
    let work_dir = crate::get_exec_path();
    let path = std::path::Path::new(&work_dir).join("data");
    std::thread::spawn(move || {
        let mut last_size = 0;
        loop {
            let data: Vec<Instrument> = INSTRUMENTS.iter().map(|v| v.value().clone()).collect();
            if data.len() != last_size {
                match std::fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path.join(".symbols.txt"))
                {
                    Ok(mut file) => {
                        if let Err(err) = ron::ser::to_writer_pretty(
                            &mut file,
                            &data,
                            ron::ser::PrettyConfig::default(),
                        ) {
                            log::error!("{:?}", err);
                        } else if let Err(err) =
                            std::fs::rename(path.join(".symbols.txt"), path.join("symbols.txt"))
                        {
                            log::error!("{:?}", err);
                        } else {
                            last_size = data.len();
                        }
                    }
                    Err(err) => {
                        log::error!("{:?}", err);
                    }
                }
                std::thread::sleep(std::time::Duration::from_secs(15));
            }
        }
    });
}
