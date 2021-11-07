use crate::broker::{Factory, Trades};
use ahash::RandomState;
use anyhow::Result;
use dashmap::DashMap;
use lazy_static::lazy_static;
use std::ops::Deref;
use std::sync::Arc;
use url::Url;

lazy_static! {
    static ref TRADERS: DashMap<Url, Trader, RandomState> =
        DashMap::with_hasher(RandomState::new());
}

#[derive(Clone)]
pub struct Trader {
    name: String,
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
    let name = uri.path().into();

    let exec = Trader { name, inner };
    TRADERS.insert(uri, exec.clone());
    Ok(exec)
}

pub fn get(uri: &Url) -> Option<Trader> {
    if let Some(exec) = TRADERS.get(uri) {
        return Some(exec.value().clone());
    }
    None
}

pub fn remove(uri: &Url) -> Option<Trader> {
    if let Some((_, trader)) = TRADERS.remove(uri) {
        return Some(trader);
    }
    None
}

pub fn list() -> Vec<Trader> {
    TRADERS.values().map(|exec| exec.clone()).collect()
}
