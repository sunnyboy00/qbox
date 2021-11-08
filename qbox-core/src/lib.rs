#![feature(trait_upcasting)]
#![allow(incomplete_features)]

mod bus;
mod db;
mod filters;
mod setting;

pub mod broker;
pub mod indicators;
pub mod strategy;
pub use bus::{
    broadcast, log, publish, query_event, quotes_event, subscribe, topics, trade_event, Event,
};
pub use db::*;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

const DATA_PATH: &str = "data";
const LOG_PATH: &str = "logs";

pub fn startup() -> Result<()> {
    //启动总线
    log::debug!("qbox bus startup");
    crate::bus::startup()?;
    log::debug!("qbox db startup");
    //启动数据库
    crate::db::startup()?;
    Ok(())
}

pub fn data_path() -> String {
    let data_path = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join(DATA_PATH);
    if !data_path.exists() {
        std::fs::create_dir_all(&data_path).ok();
    }
    data_path.to_str().unwrap().to_string()
}

pub fn log_path() -> String {
    let log_path = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join(LOG_PATH);
    if !log_path.exists() {
        std::fs::create_dir_all(&log_path).ok();
    }
    log_path.to_str().unwrap().to_string()
}

pub type Item = String;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Value {
    F32(f32),
    F64(f64),
    String(String),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    ISize(isize),
    USize(usize),
    Char(char),
    Bytes(Vec<u8>),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Parameter(HashMap<Item, Value>);

impl Deref for Parameter {
    type Target = HashMap<Item, Value>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Parameter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Parameter {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    pub fn with<S: AsRef<str>>(mut self, key: S, val: Value) -> Self {
        self.insert(key.as_ref().into(), val);
        self
    }
}
