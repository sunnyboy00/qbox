use crate::broker::{Order, TradeEvent, Transaction};
use crate::bus::{self, topics, Event, Topic};
use ahash::RandomState;
use anyhow::Result;
use dashmap::DashMap;
use lazy_static::lazy_static;
use std::collections::BTreeMap;
use std::sync::Arc;

lazy_static! {
    //订单
    static ref ORDERS: DashMap<String, Vec<Order>,RandomState> = DashMap::with_hasher(RandomState::new());

    //成交记录
    static ref TRANSACTIONS: DashMap<String, BTreeMap<String,Vec<Transaction>>> = DashMap::new();

}

pub fn get_order(security_id: &String) -> Option<Vec<Order>> {
    if let Some(positions) = ORDERS.get(security_id) {
        Some(positions.value().clone())
    } else {
        None
    }
}

pub fn get_all_order() -> Option<Vec<Order>> {
    let data: Vec<Vec<Order>> = ORDERS.iter().map(|v| v.value().clone()).collect();
    if data.len() > 0 {
        Some(data.concat())
    } else {
        None
    }
}

pub fn get_transaction(security_id: &String) -> Option<Vec<Transaction>> {
    if let Some(positions) = TRANSACTIONS.get(security_id) {
        let data: Vec<Vec<Transaction>> =
            positions.value().iter().map(|(_, v)| v.clone()).collect();
        Some(data.concat())
    } else {
        None
    }
}

pub fn get_all_transaction() -> Option<Vec<Transaction>> {
    let data: Vec<Vec<Vec<Transaction>>> = TRANSACTIONS
        .iter()
        .map(|v| v.value().values().into_iter().map(|v| v.clone()).collect())
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
    if let Some(trans) = TRANSACTIONS.get(security_id) {
        if let Some(tx) = trans.value().get(order_id) {
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
                if let Some(mut positions) = ORDERS.get_mut(pos.security_id()) {
                    positions.value_mut().push(pos.clone());
                } else {
                    let mut positions = vec![];
                    positions.push(pos.clone());
                    ORDERS.insert(pos.security_id().into(), positions);
                }
            }
            _ => {}
        }
    } else {
        log::warn!("!!!!!!!!!! {} {:?}", topic, ev)
    }
}
