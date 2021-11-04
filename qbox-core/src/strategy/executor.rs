use super::Factory;
use super::Strategy;
use anyhow::Result;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use url::Url;

lazy_static! {
    static ref EXECUTORS: Mutex<HashMap<String, Executor>> = Mutex::new(HashMap::new());
}

#[derive(Clone)]
pub struct Executor {
    pub name: String,
    inner: Arc<dyn Strategy>,
}

impl Deref for Executor {
    type Target = Arc<dyn Strategy>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub fn spawn(uri: Url) -> std::io::Result<JoinHandle<Result<()>>> {
    thread::Builder::new()
        .name(format!(
            "{}://{}{}",
            uri.scheme(),
            uri.host_str().unwrap_or_default(),
            uri.path()
        ))
        .spawn(move || -> Result<()> {
            let name = thread::current().name().unwrap_or_default().to_string();
            let inner = Factory::create(uri)?;
            let exec = Executor {
                name: name.clone(),
                inner,
            };
            EXECUTORS.lock().insert(name, exec.clone());
            exec.start()?;
            Ok(())
        })
}

pub fn get(name: &String) -> Option<Executor> {
    if let Some(exec) = EXECUTORS.lock().get(name) {
        return Some(exec.clone());
    }
    None
}

pub fn remove(name: &String) -> Option<Executor> {
    if let Some(exec) = EXECUTORS.lock().remove(name) {
        return Some(exec);
    }
    None
}

pub fn list() -> Vec<Executor> {
    EXECUTORS.lock().values().map(|exec| exec.clone()).collect()
}
