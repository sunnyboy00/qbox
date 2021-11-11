use crate::db::memory::MemQuoteStore;
use crate::db::sqlite::SqliteStore;
use crate::db::{OrderStore, QuoteStore};
use anyhow::Result;
use lazy_static::lazy_static;

lazy_static! {
    static ref QUOTES: Option<MemQuoteStore> = None;
}

pub fn quotes<S: Into<String>>(unit: S) -> impl QuoteStore {
    MemQuoteStore::open(unit)
}

pub fn trades<S: AsRef<str>>(unit: S) -> Result<impl OrderStore> {
    SqliteStore::open(unit)
}
