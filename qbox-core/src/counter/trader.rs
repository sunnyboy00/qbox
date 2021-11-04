use crate::counter::{Factory, Trades};
use anyhow::Result;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use url::Url;

lazy_static! {
    static ref QUOTERS: Mutex<HashMap<Url, Trader>> = Mutex::new(HashMap::new());
}

#[derive(Clone)]
pub struct Trader {
    pub uri: Url,
    inner: Arc<dyn Trades>,
}

impl Deref for Trader {
    type Target = Arc<dyn Trades>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub fn spawn(uri: Url) -> Result<Trader> {
    log::debug!("spwan trades {}", uri);
    let inner = Factory::create(uri.clone())?;
    let exec = Trader {
        uri: uri.clone(),
        inner,
    };
    QUOTERS.lock().insert(uri, exec.clone());
    Ok(exec)
}

pub fn get(uri: &Url) -> Option<Trader> {
    if let Some(exec) = QUOTERS.lock().get(uri) {
        return Some(exec.clone());
    }
    None
}

pub fn remove(uri: &Url) -> Option<Trader> {
    if let Some(exec) = QUOTERS.lock().remove(uri) {
        return Some(exec);
    }
    None
}

pub fn list() -> Vec<Trader> {
    QUOTERS.lock().values().map(|exec| exec.clone()).collect()
}
