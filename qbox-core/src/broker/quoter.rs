use crate::broker::{Factory, Quotes};
use anyhow::Result;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use url::Url;

lazy_static! {
    static ref QUOTERS: Mutex<HashMap<Url, Quoter>> = Mutex::new(HashMap::new());
}

#[derive(Clone)]
pub struct Quoter {
    pub uri: Url,
    inner: Arc<dyn Quotes>,
}

impl Deref for Quoter {
    type Target = Arc<dyn Quotes>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub fn spawn(uri: Url) -> Result<Quoter> {
    log::debug!("spawn quotes {}", uri);
    let inner = Factory::create(uri.clone())?;
    let exec = Quoter {
        uri: uri.clone(),
        inner,
    };
    QUOTERS.lock().insert(uri, exec.clone());
    Ok(exec)
}

pub fn get(uri: &Url) -> Option<Quoter> {
    if let Some(exec) = QUOTERS.lock().get(uri) {
        return Some(exec.clone());
    }
    None
}

pub fn remove(uri: &Url) -> Option<Quoter> {
    if let Some(exec) = QUOTERS.lock().remove(uri) {
        return Some(exec);
    }
    None
}

pub fn list() -> Vec<Quoter> {
    QUOTERS.lock().values().map(|exec| exec.clone()).collect()
}
