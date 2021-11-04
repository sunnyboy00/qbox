#![feature(trait_upcasting)]
#![allow(incomplete_features)]

mod bus;
mod db;

pub mod counter;
pub mod indicators;
pub mod strategy;
pub use bus::{
    broadcast, log, publish, query_event, quotes_event, subscribe, topics, trade_event, Event,
};
pub use db::*;

use anyhow::Result;

pub fn startup() -> Result<()> {
    //启动总线
    log::debug!("qbox bus startup");
    crate::bus::startup()?;
    log::debug!("qbox db startup");
    //启动数据库
    crate::db::startup()?;
    Ok(())
}

pub fn get_exec_path() -> String {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}
