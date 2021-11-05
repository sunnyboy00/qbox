use crate::broker::{Position, TradeEvent};
use crate::bus::{self, topics, Event, Topic};
use anyhow::Result;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use std::collections::BTreeMap;
use std::sync::Arc;

lazy_static! {
    //仓位
    static ref POSITIONS: RwLock<BTreeMap<String, Vec<Position>>> = RwLock::new(BTreeMap::new());

}

pub fn get_position(security_id: &String) -> Option<Vec<Position>> {
    let map = POSITIONS.read();
    if let Some(positions) = map.get(security_id) {
        Some(positions.clone())
    } else {
        None
    }
}

pub fn get_all_position() -> Option<Vec<Position>> {
    let map = POSITIONS.read();
    let data: Vec<Vec<Position>> = map.values().into_iter().map(|v| v.clone()).collect();
    if data.len() > 0 {
        Some(data.concat())
    } else {
        None
    }
}

pub fn find_position_with_prefix(prefix: &str) -> Option<Vec<Position>> {
    let map = POSITIONS.read();
    let data: Vec<Vec<Position>> = map
        .iter()
        .filter(|(k, _)| k.starts_with(prefix))
        .map(|(_, v)| v.clone())
        .collect();
    if data.len() > 0 {
        Some(data.concat())
    } else {
        None
    }
}

pub fn find_position_with_prefixs(prefixs: &[&str]) -> Option<Vec<Position>> {
    let data: Vec<Vec<Position>> = prefixs
        .iter()
        .filter_map(|prefix| find_position_with_prefix(prefix))
        .collect();
    if data.len() > 0 {
        Some(data.concat())
    } else {
        None
    }
}

pub(crate) fn init() -> Result<()> {
    let _ = bus::subscribe(topics::QUERY_EVENT, process)?;
    Ok(())
}

fn process(topic: Topic, ev: Arc<Event>) {
    log::trace!("process {:?}", ev);
    if let Event::Trade(tev) = ev.as_ref() {
        match tev {
            TradeEvent::PositionResponse(pos) => {
                let mut map = POSITIONS.write();
                if let Some(positions) = map.get_mut(&pos.security_id) {
                    positions.push(pos.clone());
                } else {
                    let mut positions = vec![];
                    positions.push(pos.clone());
                    map.insert(pos.security_id.clone(), positions);
                }
            }
            _ => {}
        }
    } else {
        log::warn!("!!!!!!!!!! {} {:?}", topic, ev)
    }
}
