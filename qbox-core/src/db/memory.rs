use super::QuoteStore;
use crate::broker::{
    Bar, Instrument, Level1, Level2, QuoteEvent, TickToOffer, TickToTrade, TradeEvent,
};
use crate::core::{self, *};
use ahash::RandomState;
use anyhow::Result;
use crossbeam::channel::{self, Receiver};
use dashmap::mapref::multiple::RefMulti;
use dashmap::{DashMap, DashSet};
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use std::sync::Arc;

lazy_static! {
    static ref STORAGE: DashMap<String, DashMap<String, Vec<u8>>, RandomState> =
        DashMap::with_hasher(RandomState::new());
        static ref BARS: DashMap<String, Vec<Bar>,RandomState> = DashMap::with_hasher(RandomState::new());
        //LEVEL1行情
        static ref LEVEL1S: DashMap<String, Level1,RandomState> = DashMap::with_hasher(RandomState::new());
        //深度行情
        static ref DEPTHS: DashMap<String, Level2,RandomState> = DashMap::with_hasher(RandomState::new());
        //逐笔委托
        static ref TTOS: DashMap<String, Vec<TickToOffer>,RandomState> = DashMap::with_hasher(RandomState::new());
        //逐笔成交
        static ref TTTS: DashMap<String, Vec<TickToTrade>,RandomState> = DashMap::with_hasher(RandomState::new());
}

#[derive(Clone)]
pub struct MemQuoteStore {
    unit: String,
    level1: DashMap<String, Level1, RandomState>,
    bars: DashMap<String, Vec<Bar>, RandomState>,
    depths: DashMap<String, Level2, RandomState>,
    ttos: DashMap<String, Vec<TickToOffer>, RandomState>,
    ttts: DashMap<String, Vec<TickToTrade>, RandomState>,
}

impl MemQuoteStore {
    pub fn open<S: Into<String>>(unit: S) -> Self {
        static INSTANCE: OnceCell<MemQuoteStore> = OnceCell::new();
        let ret = INSTANCE
            .get_or_init(|| Self {
                unit: unit.into(),
                level1: DashMap::with_hasher(RandomState::new()),
                bars: DashMap::with_hasher(RandomState::new()),
                depths: DashMap::with_hasher(RandomState::new()),
                ttos: DashMap::with_hasher(RandomState::new()),
                ttts: DashMap::with_hasher(RandomState::new()),
            })
            .clone();
        ret
    }
}

impl QuoteStore for MemQuoteStore {}
