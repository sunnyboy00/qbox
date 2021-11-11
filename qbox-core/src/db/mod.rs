pub mod memory;
pub mod rocksdb;
pub mod sqlite;

use crate::broker::{
    Bar, Instrument, Level1, Level2, Order, Period, Position, TickToOffer, TickToTrade, Transaction,
};
use anyhow::Result;

pub trait QuoteStore {
    fn insert_level1(&self, level1: Level1) -> Result<()> {
        unimplemented!()
    }
    fn update_level1(&self, level1: Level1) -> Result<()> {
        unimplemented!()
    }
    fn query_one_level1(&self, security_id: &str) -> Result<Option<Level1>> {
        unimplemented!()
    }
    fn query_all_level1(&self) -> Result<Option<Vec<Level1>>> {
        unimplemented!()
    }
    fn query_level1_with_prefix(&self, prefix: &str) -> Result<Option<Vec<Level1>>> {
        unimplemented!()
    }
    fn query_level1_with_prefixs(&self, prefixs: &[&str]) -> Result<Option<Vec<Level1>>> {
        unimplemented!()
    }
    fn insert_bar(&self, bar: Bar) -> Result<()> {
        unimplemented!()
    }
    fn query_bar(&self, security_id: &str, period: Period) -> Result<Option<Vec<Bar>>> {
        unimplemented!()
    }
    fn insert_tick2offer(&self, tto: TickToOffer) -> Result<()> {
        unimplemented!()
    }
    fn query_tick2offer(&self, security_id: &str) -> Result<Option<Vec<TickToOffer>>> {
        unimplemented!()
    }
    fn insert_tick2trade(&self, ttt: TickToTrade) -> Result<()> {
        unimplemented!()
    }
    fn query_tick2trade(&self, security_id: &str) -> Result<Option<Vec<TickToTrade>>> {
        unimplemented!()
    }
    fn update_depth(&self, level2: Level2) -> Result<()> {
        unimplemented!()
    }
    fn query_depth(&self, security_id: &str) -> Result<Option<Level2>> {
        unimplemented!()
    }
}

pub trait OrderStore {
    fn insert_order(&self, order: Order) -> Result<()> {
        unimplemented!()
    }
    fn update_order(&self, order: Order) -> Result<()> {
        unimplemented!()
    }
    fn remove_order(&self, order_id: u64) -> Result<()> {
        unimplemented!()
    }
    fn query_one_order(&self, order_id: u64) -> Result<Option<Order>> {
        unimplemented!()
    }
    fn query_order(&self, security_id: &str) -> Result<Option<Vec<Order>>> {
        unimplemented!()
    }
    fn query_all_order(&self) -> Result<Option<Vec<Order>>> {
        unimplemented!()
    }

    fn insert_tx(&self, tx: Transaction) -> Result<()> {
        unimplemented!()
    }
    fn update_tx(&self, tx: Transaction) -> Result<()> {
        unimplemented!()
    }
    fn remove_tx(&self, txid: u64) -> Result<()> {
        unimplemented!()
    }
    fn query_one_tx(&self, txid: u64) -> Result<Option<Transaction>> {
        unimplemented!()
    }
    fn query_tx(&self, order_id: u64) -> Result<Option<Vec<Transaction>>> {
        unimplemented!()
    }
    fn query_tx_with_symbol(&self, security_id: &str) -> Result<Option<Vec<Transaction>>> {
        unimplemented!()
    }
    fn query_tx_with_symbol_and_order(
        &self,
        security_id: &str,
        order_id: u64,
    ) -> Result<Option<Vec<Transaction>>> {
        unimplemented!()
    }
    fn query_all_tx(&self) -> Result<Option<Vec<Transaction>>> {
        unimplemented!()
    }

    fn update_position(&self, position: Position) -> Result<()> {
        unimplemented!()
    }
    fn update_or_insert_position(&self, position: Position) -> Result<()> {
        unimplemented!()
    }
    fn remove_position(&self, security_id: &str) -> Result<()> {
        unimplemented!()
    }
    fn query_position(&self, security_id: &str) -> Result<Option<Vec<Position>>> {
        unimplemented!()
    }
    fn query_all_position(&self) -> Result<Option<Vec<Position>>> {
        unimplemented!()
    }
}

pub trait QboxStore {
    fn set(&self, k: &str, v: &str) -> Result<()> {
        unimplemented!()
    }
    fn remove(&self, k: &str, v: &str) -> Result<()> {
        unimplemented!()
    }
    fn get(&self, k: &str, def: &str) -> Result<Option<String>> {
        unimplemented!()
    }
    fn get_all(&self, k: &str) -> Result<Option<Vec<String>>> {
        unimplemented!()
    }
    fn get_prefix(&self, prefix: &str) -> Result<Option<Vec<String>>> {
        unimplemented!()
    }
    fn get_prefixs(&self, prefixs: &[&str]) -> Result<Option<Vec<String>>> {
        unimplemented!()
    }

    fn update_symbol(&self) -> Result<Option<Vec<Instrument>>> {
        unimplemented!()
    }
    fn query_one_symbol(&self, security_id: &str) -> Result<Option<Vec<Instrument>>> {
        unimplemented!()
    }
    fn query_symbol_with_prefix(&self, prefix: &str) -> Result<Option<Vec<Instrument>>> {
        unimplemented!()
    }
    fn query_symbol_with_prefixs(&self, prefixs: &[&str]) -> Result<Option<Vec<Instrument>>> {
        unimplemented!()
    }
    fn query_all_symbol(&self) -> Result<Option<Vec<Instrument>>> {
        unimplemented!()
    }
}
