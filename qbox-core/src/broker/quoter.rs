use crate::broker::{Factory, Quotes};
use ahash::RandomState;
use anyhow::Result;
use dashmap::DashMap;
use lazy_static::lazy_static;
use std::ops::Deref;
use std::sync::Arc;
use url::Url;

lazy_static! {
    static ref QUOTERS: DashMap<String, Quoter, RandomState> =
        DashMap::with_hasher(RandomState::new());
}

#[derive(Clone)]
pub struct Quoter {
    pub name: String,
    inner: Arc<dyn Quotes>,
}

impl Deref for Quoter {
    type Target = Arc<dyn Quotes>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub fn spawn(uri: Url) -> Result<Quoter> {
    let name = format!(
        "{}://{}@{}:{}{}",
        uri.scheme(),
        uri.username(),
        uri.host_str().unwrap_or(""),
        uri.port().unwrap_or(0),
        uri.path()
    );

    if QUOTERS.contains_key(&name) {
        return Err(anyhow::anyhow!("quoter {} exist", name));
    }
    log::debug!("spawn quotes {}", uri);

    let inner = Factory::create(uri.clone())?;
    let exec = Quoter {
        name: name.clone(),
        inner,
    };
    QUOTERS.insert(name, exec.clone());
    Ok(exec)
}

pub fn get(name: &str) -> Option<Quoter> {
    if let Some(exec) = QUOTERS.get(name) {
        return Some(exec.value().clone());
    }
    None
}

pub fn remove(name: &str) -> Option<Quoter> {
    if let Some((_, exec)) = QUOTERS.remove(name) {
        return Some(exec);
    }
    None
}

pub fn list() -> Vec<Quoter> {
    QUOTERS.iter().map(|exec| exec.value().clone()).collect()
}
