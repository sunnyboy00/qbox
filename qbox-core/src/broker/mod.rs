pub mod quoter;
pub mod trader;
pub mod types;

pub use types::*;

use anyhow::Result;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use url::Url;

lazy_static! {
    static ref DRIVERS: Mutex<HashMap<&'static str, Box<dyn Driver>>> = Mutex::new(HashMap::new());
}

pub fn counters() -> Vec<&'static str> {
    let keys: Vec<&str> = DRIVERS.lock().keys().map(|name| *name).collect();
    keys
}

pub fn register_driver(name: &'static str, driver: impl Driver + 'static) -> Result<()> {
    let mut drivers = DRIVERS.lock();
    if drivers.contains_key(name) {
        return Err(anyhow::anyhow!("driver `{}` existed"));
    }
    drivers.insert(name, Box::new(driver));
    Ok(())
}

pub fn unregister_driver(name: &str) -> Result<()> {
    let _ = DRIVERS.lock().remove(name);
    Ok(())
}

pub(crate) struct Factory;

impl Factory {
    pub(crate) fn create(uri: Url) -> Result<Arc<dyn Counter>> {
        let name = uri.scheme();
        if let Some(driver) = DRIVERS.lock().get(name) {
            log::debug!("create counter {}", uri);
            driver.create(uri)
        } else {
            Err(anyhow::anyhow!("{:?} not found", name))
        }
    }
}

#[doc = "柜台接口"]
pub trait Counter: Quotes + Trades {
    fn stop(&self) -> Result<()>;
}

pub trait Driver: Send {
    fn create(&self, opt: Url) -> Result<Arc<dyn Counter>>;
}

#[doc = "交易接口"]
pub trait Trades: Send + Sync {
    fn name(&self) -> &'static str;
    //查结算货币
    fn currencies(&self) -> Vec<Currency>;
    //查证券
    fn instruments(&self, filter: &[&str]);
    //查账户
    fn accounts(&self, filter: &[&str]);
    //查时区
    fn timezone(&mut self, zone: &'static str) -> String;
    //报单
    fn offer(&self, order: Order) -> Result<Order>;
    //撤单
    fn cancel(&self, order: Order) -> Result<()>;
    //查订单
    fn query(&self, order: Order) -> Result<Order>;
    //查持仓
    fn positions(&self, after: &str, before: &str, limit: u8, filters: &[&str]) -> Vec<Position>;
}

#[doc = "行情接口"]
pub trait Quotes: Send + Sync {
    fn subscribe(&self, filter: &[&str]);
    fn unsubscribe(&self, filter: &[&str]);
}

#[doc = "费用计算"]
pub trait Fee<T> {
    type Output;
    fn evaluate(&mut self, input: T) -> Self::Output;
}
