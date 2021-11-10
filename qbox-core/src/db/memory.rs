use super::{Bucket, Database, FilterFlags, GetBatch, PutBatch, RemoveBatch};
use crate::broker::{
    Bar, Instrument, Level1, Level2, QuoteEvent, TickToOffer, TickToTrade, TradeEvent,
};
use crate::core::{self, *};
use ahash::RandomState;
use anyhow::Result;
use crossbeam::channel::{self, Receiver};
use dashmap::mapref::multiple::RefMulti;
use dashmap::{DashMap, DashSet};
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use std::sync::Arc;

lazy_static! {
    static ref STORAGE: DashMap<String, DashMap<String, Vec<u8>>, RandomState> =
        DashMap::with_hasher(RandomState::new());
}

#[derive(Clone)]
pub struct MemoryDB {
    defbucket: MemBucket,
    inner: DashMap<String, MemBucket, RandomState>,
}

impl MemoryDB {
    pub fn open() -> Self {
        static INSTANCE: OnceCell<MemoryDB> = OnceCell::new();
        let ret = INSTANCE
            .get_or_init(|| {
                let inner = DashMap::with_hasher(RandomState::new());
                Self {
                    defbucket: MemBucket::new("defbucket"),
                    inner,
                }
            })
            .clone();
        ret
    }
}

impl Database for MemoryDB {
    type Bucket = MemBucket;
    fn buckets(&self) -> Vec<String> {
        let vec: Vec<String> = self.inner.iter().map(|v| v.key().clone()).collect();
        vec
    }
    fn bucket<S: Into<String>>(&self, name: S) -> Result<MemBucket> {
        let name = name.into();
        if let Some(bucket) = self.inner.get(&name) {
            Ok(bucket.value().clone())
        } else {
            let bucket = MemBucket::new(name.clone());
            self.inner.insert(name, bucket.clone());
            Ok(bucket)
        }
    }
    fn drop_bucket<S: AsRef<str>>(&self, name: S) -> Result<()> {
        self.inner.remove(name.as_ref());
        Ok(())
    }
}

impl Bucket for MemoryDB {
    fn name(&self) -> &str {
        self.defbucket.name()
    }
    fn put<K: Into<String>, V: AsRef<[u8]>>(&self, key: K, val: V) -> Result<()> {
        self.defbucket.put(key.into(), val)
    }

    fn get<K: AsRef<str>>(&self, key: K) -> Result<Option<Vec<u8>>> {
        self.defbucket.get(key)
    }
    fn remove<K: AsRef<str>>(&self, key: K) -> Result<()> {
        self.defbucket.remove(key)
    }

    fn find_prefix<K: AsRef<str>>(&self, prefix: K) -> Result<Option<Vec<Vec<u8>>>> {
        self.defbucket.find_prefix(prefix)
    }
    fn remove_prefix<K: AsRef<str>>(&self, prefix: K) -> Result<()> {
        self.defbucket.remove_prefix(prefix)
    }
    fn find_prefix_with_filter<K: AsRef<str>>(
        &self,
        prefix: K,
        filter: impl Fn(&[u8]) -> FilterFlags,
    ) -> Result<Option<Vec<Vec<u8>>>> {
        self.defbucket.find_prefix_with_filter(prefix, filter)
    }

    fn batch_put<K: Into<String>, V: AsRef<[u8]>>(&self, batch: PutBatch<K, V>) -> Result<()> {
        self.defbucket.batch_put(batch)
    }
    fn batch_get<K: AsRef<str>>(&self, batch: GetBatch<K>) -> Result<Option<Vec<Vec<u8>>>> {
        self.defbucket.batch_get(batch)
    }
    fn batch_remove<K: AsRef<str>>(&self, batch: RemoveBatch<K>) -> Result<()> {
        self.defbucket.batch_remove(batch)
    }

    fn count(&self) -> Result<usize> {
        self.defbucket.count()
    }
}

#[derive(Clone)]
pub struct MemBucket {
    name: String,
    table: DashMap<String, Vec<u8>, RandomState>,
}

impl MemBucket {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            table: DashMap::with_hasher(RandomState::new()),
        }
    }
}

impl Bucket for MemBucket {
    fn name(&self) -> &str {
        self.name.as_str()
    }
    fn put<K: Into<String>, V: AsRef<[u8]>>(&self, key: K, val: V) -> Result<()> {
        let key = key.into();
        let val = val.as_ref();
        self.table.insert(key, val.into());
        Ok(())
    }

    fn get<K: AsRef<str>>(&self, key: K) -> Result<Option<Vec<u8>>> {
        if let Some(val) = self.table.get(key.as_ref()) {
            Ok(Some(val.value().clone()))
        } else {
            Ok(None)
        }
    }
    fn remove<K: AsRef<str>>(&self, key: K) -> Result<()> {
        self.table.remove(key.as_ref());
        Ok(())
    }

    fn find_prefix<K: AsRef<str>>(&self, prefix: K) -> Result<Option<Vec<Vec<u8>>>> {
        let rs: Vec<Vec<u8>> = self
            .table
            .iter()
            .filter(|item| item.key().starts_with(prefix.as_ref()))
            .map(|item| item.value().clone())
            .collect();
        if rs.len() > 0 {
            Ok(Some(rs))
        } else {
            Ok(None)
        }
    }
    fn remove_prefix<K: AsRef<str>>(&self, prefix: K) -> Result<()> {
        let keys: Vec<String> = self
            .table
            .iter()
            .filter(|item| item.key().starts_with(prefix.as_ref()))
            .map(|item| item.key().clone())
            .collect();
        keys.into_iter().for_each(|key| {
            self.table.remove(&key);
        });
        Ok(())
    }
    fn find_prefix_with_filter<K: AsRef<str>>(
        &self,
        prefix: K,
        filter: impl Fn(&[u8]) -> FilterFlags,
    ) -> Result<Option<Vec<Vec<u8>>>> {
        let rs: Vec<Vec<u8>> = self
            .table
            .iter()
            .filter(|item| item.key().starts_with(prefix.as_ref()))
            .map(|item| item.value().clone())
            .collect();
        let mut ret = vec![];

        for v in rs {
            match filter(&v) {
                FilterFlags::Next => {
                    ret.push(v);
                }
                FilterFlags::Break => break,
                _ => {}
            }
        }
        if ret.len() > 0 {
            Ok(Some(ret))
        } else {
            Ok(None)
        }
    }

    fn batch_put<K: Into<String>, V: AsRef<[u8]>>(&self, batch: PutBatch<K, V>) -> Result<()> {
        batch.iter().for_each(|(_, key, val)| {
            let k = key.into();
            let key: String = <String as Into>::to_owned();
            self.table.insert(key, val.as_ref().into());
        });
        unimplemented!()
    }
    fn batch_get<K: AsRef<str>>(&self, batch: GetBatch<K>) -> Result<Option<Vec<Vec<u8>>>> {
        unimplemented!()
    }
    fn batch_remove<K: AsRef<str>>(&self, batch: RemoveBatch<K>) -> Result<()> {
        unimplemented!()
    }

    fn count(&self) -> Result<usize> {
        Ok(0)
    }
}
