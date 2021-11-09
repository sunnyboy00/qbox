#![feature(trait_upcasting)]
#![allow(incomplete_features)]

mod bus;

pub mod broker;
pub mod comm;
pub mod core;
mod db;
pub mod filter;
pub mod indicators;
pub mod setting;
pub mod strategy;

const DATA_PATH: &str = "data";
const LOG_PATH: &str = "logs";

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
