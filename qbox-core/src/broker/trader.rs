use crate::broker::{Factory, Trades};
use ahash::RandomState;
use anyhow::Result;
use dashmap::DashMap;
use lazy_static::lazy_static;
use std::ops::Deref;
use std::sync::Arc;
use url::Url;

lazy_static! {
    static ref TRADERS: DashMap<String, Trader, RandomState> =
        DashMap::with_hasher(RandomState::new());
}

#[derive(Clone)]
pub struct Trader {
    pub name: String,
    inner: Arc<dyn Trades>,
}

impl Deref for Trader {
    type Target = Arc<dyn Trades>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub fn spawn(uri: Url) -> Result<Trader> {
    let name = format!(
        "{}://{}@{}:{}{}",
        uri.scheme(),
        uri.username(),
        uri.host_str().unwrap_or(""),
        uri.port().unwrap_or(0),
        uri.path()
    );

    if TRADERS.contains_key(&name) {
        return Err(anyhow::anyhow!("trader {} exist", name));
    }
    log::debug!("spwan trades {}", uri);
    let inner = Factory::create(uri.clone())?;
    let exec = Trader {
        name: name.clone(),
        inner,
    };

    TRADERS.insert(name, exec.clone());
    Ok(exec)
}

pub fn get(name: &str) -> Option<Trader> {
    if let Some(exec) = TRADERS.get(name) {
        return Some(exec.value().clone());
    }
    None
}

pub fn remove(name: &str) -> Option<Trader> {
    if let Some((_, trader)) = TRADERS.remove(name) {
        return Some(trader);
    }
    None
}

pub fn list() -> Vec<Trader> {
    TRADERS.iter().map(|exec| exec.value().clone()).collect()
}
