pub mod executor;
use crate::broker::Parameter;
use crate::core::events::QuoteEvent;
use anyhow::Result;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use url::Url;

lazy_static! {
    static ref DRIVERS: Mutex<HashMap<&'static str, Box<dyn Driver>>> = Mutex::new(HashMap::new());
}

pub fn strategies() -> Vec<&'static str> {
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

struct Factory;

impl Factory {
    fn create(uri: Url) -> Result<Arc<dyn Strategy>> {
        let name = uri.scheme();
        if let Some(driver) = DRIVERS.lock().get(name) {
            let strategy = driver.create(uri);
            Ok(strategy)
        } else {
            Err(anyhow::anyhow!("{:?} not found", name))
        }
    }
}

pub trait Driver: Send {
    fn create(&self, opt: Url) -> Arc<dyn Strategy>;
}

#[doc = "策略接口"]
pub trait Strategy: Send + Sync {
    fn name(&self) -> &str;
    fn init(&self, params: Parameter);
    fn start(&self) -> Result<()>;
    fn suspend(&self) -> Result<()>;
    fn resume(&self) -> Result<()>;
    fn stop(&self) -> Result<()>;
    fn on_quotes(&self, quotes: QuoteEvent);
}
