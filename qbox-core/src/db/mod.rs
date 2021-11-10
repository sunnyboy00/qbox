pub mod memory;
pub mod rocksdb;
pub mod sqlite;

use anyhow::Result;
use std::ops::Deref;
use std::ops::DerefMut;
use zerocopy::{AsBytes, FromBytes};

pub enum FilterFlags {
    Next,
    Skip,
    Break,
}

pub enum Op {
    Put,
    Get,
    Remove,
}
pub struct Batch<K, V>(Vec<(Op, K, V)>);
pub type PutBatch<K, V> = Batch<K, V>;
pub type GetBatch<K> = Batch<K, ()>;
pub type RemoveBatch<K> = Batch<K, ()>;

impl<K, V> Deref for Batch<K, V> {
    type Target = Vec<(Op, K, V)>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> DerefMut for Batch<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, V> Batch<K, V> {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn with_op(mut self, op: Op, key: K, val: V) -> Self {
        self.0.push((op, key, val));
        self
    }

    pub fn op(&mut self, op: Op, key: K, val: V) {
        self.0.push((op, key, val));
    }
}

impl<K, V> PutBatch<K, V> {
    pub fn with_put(mut self, key: K, val: V) -> Self {
        self.with_op(Op::Put, key, val)
    }
    pub fn put(&mut self, key: K, val: V) {
        self.op(Op::Put, key, val);
    }
}

impl<K> GetBatch<K> {
    pub fn get(&mut self, key: K) {
        self.op(Op::Get, key, ());
    }

    pub fn with_get(mut self, key: K) -> Self {
        self.with_op(Op::Get, key, ())
    }
}

impl<K> RemoveBatch<K> {
    pub fn remove(&mut self, key: K) {
        self.op(Op::Remove, key, ());
    }
    pub fn with_remove(mut self, key: K) -> Self {
        self.with_op(Op::Remove, key, ())
    }
}

pub trait Database: Bucket {
    type Bucket: Bucket;
    fn buckets(&self) -> Vec<String>;
    fn bucket<S: Into<String>>(&self, name: S) -> Result<Self::Bucket>;
    fn drop_bucket<S: AsRef<str>>(&self, name: S) -> Result<()>;
}

pub trait Bucket: Send + Sync {
    fn name(&self) -> &str;
    fn put<K: Into<String>, V: AsRef<[u8]>>(&self, key: K, val: V) -> Result<()>;
    fn get<K: AsRef<str>>(&self, key: K) -> Result<Option<Vec<u8>>>;
    fn remove<K: AsRef<str>>(&self, key: K) -> Result<()>;
    fn remove_prefix<K: AsRef<str>>(&self, prefix: K) -> Result<()>;
    fn find_prefix<K: AsRef<str>>(&self, prefix: K) -> Result<Option<Vec<Vec<u8>>>>;
    fn find_prefix_with_filter<K: AsRef<str>>(
        &self,
        prefix: K,
        filter: impl Fn(&[u8]) -> FilterFlags,
    ) -> Result<Option<Vec<Vec<u8>>>>;

    fn batch_put<K: Into<String>, V: AsRef<[u8]>>(&self, batch: PutBatch<K, V>) -> Result<()>;
    fn batch_get<K: AsRef<str>>(&self, batch: GetBatch<K>) -> Result<Option<Vec<Vec<u8>>>>;
    fn batch_remove<K: AsRef<str>>(&self, batch: RemoveBatch<K>) -> Result<()>;
    fn count(&self) -> Result<usize>;
}
