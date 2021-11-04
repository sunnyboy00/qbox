use crate::bus::{self, topics, Event, Topic};
use crate::counter::{Order, TradeEvent, Transaction};
use anyhow::Result;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

lazy_static! {
    //订单
    static ref ORDERS: RwLock<HashMap<String, Vec<Order>>> = RwLock::new(HashMap::new());

    //成交记录
    static ref TRANSACTIONS: RwLock<HashMap<String,BTreeMap<String,Vec<Transaction>> >> = RwLock::new(HashMap::new());

}

pub fn get_order(security_id: &String) -> Option<Vec<Order>> {
    let map = ORDERS.read();
    if let Some(positions) = map.get(security_id) {
        Some(positions.clone())
    } else {
        None
    }
}

pub fn get_all_order() -> Option<Vec<Order>> {
    let map = ORDERS.read();
    let data: Vec<Vec<Order>> = map.values().into_iter().map(|v| v.clone()).collect();
    if data.len() > 0 {
        Some(data.concat())
    } else {
        None
    }
}

pub fn get_transaction(security_id: &String) -> Option<Vec<Transaction>> {
    let map = TRANSACTIONS.read();
    if let Some(positions) = map.get(security_id) {
        let data: Vec<Vec<Transaction>> =
            positions.values().into_iter().map(|v| v.clone()).collect();
        Some(data.concat())
    } else {
        None
    }
}

pub fn get_all_transaction() -> Option<Vec<Transaction>> {
    let map = TRANSACTIONS.read();
    let data: Vec<Vec<Vec<Transaction>>> = map
        .values()
        .into_iter()
        .map(|v| v.values().into_iter().map(|v| v.clone()).collect())
        .collect();
    if data.len() > 0 {
        Some(data.concat().concat())
    } else {
        None
    }
}

pub fn get_all_transaction_with_order_id(
    security_id: &String,
    order_id: &String,
) -> Option<Vec<Transaction>> {
    let map = TRANSACTIONS.read();
    if let Some(trans) = map.get(security_id) {
        if let Some(tx) = trans.get(order_id) {
            Some(tx.clone())
        } else {
            None
        }
    } else {
        None
    }
}

pub(crate) fn init() -> Result<()> {
    let _ = bus::subscribe(topics::TRADES_EVENT, process)?;
    Ok(())
}

fn process(topic: Topic, ev: Arc<Event>) {
    log::trace!("process {:?}", ev);
    if let Event::Trade(tev) = ev.as_ref() {
        match tev {
            TradeEvent::Offer(pos) => {
                let mut map = ORDERS.write();
                if let Some(positions) = map.get_mut(pos.security_id()) {
                    positions.push(pos.clone());
                } else {
                    let mut positions = vec![];
                    positions.push(pos.clone());
                    map.insert(pos.security_id().into(), positions);
                }
            }
            _ => {}
        }
    } else {
        log::warn!("!!!!!!!!!! {} {:?}", topic, ev)
    }
}
