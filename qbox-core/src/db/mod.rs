pub mod sqlite;

use anyhow::Result;

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
    fn new_bucket<S: AsRef<str>, B: Bucket>(&self, name: S) -> Result<B>;
    fn remove_bucket<S: AsRef<str>, B: Bucket>(&self, name: S) -> Result<B>;
    // fn chain<R: Database>(self, next: R) -> Chain<Self, R>
    // where
    //     Self: Sized,
    // {
    //     Chain {
    //         first: self,
    //         second: next,
    //     }
    // }
}

pub trait Bucket {
    fn name(&self) -> &str;
    fn put<K, V>(&self, key: K, val: V) -> Result<()>;
    fn get<K, V>(&self, key: K) -> Result<Option<&V>>;
    fn remove<K, V>(&self, key: K) -> Result<Option<&V>>;

    fn find_prefix<K, V>(&self, prefix: K) -> Result<Option<Vec<&V>>>;
    fn remove_prefix<K, V>(&self, prefix: K) -> Result<Option<Vec<&V>>>;
    fn find_prefix_with_filter<K, V>(
        &self,
        prefix: K,
        filter: impl Fn(&V) -> FilterFlags,
    ) -> Result<Option<Vec<&V>>>;
    fn remove_prefix_with_filter<K, V>(
        &self,
        prefix: K,
        filter: impl Fn(&V) -> FilterFlags,
    ) -> Result<Option<Vec<&V>>>;

    fn batch_put<K, V>(&self, batch: PutBatch<K, V>) -> Result<()>;
    fn batch_get<K, V>(&self, batch: GetBatch<K>) -> Result<Option<Vec<&V>>>;
    fn batch_remove<K, V>(&self, batch: RemoveBatch<K>) -> Result<Option<Vec<&V>>>;
}

// #[derive(Default, Clone)]
// pub struct Chain<W, U> {
//     first: W,
//     second: U,
// }

// impl<W, U> Chain<W, U> {
//     pub fn into_inner(self) -> (W, U) {
//         (self.first, self.second)
//     }

//     pub fn get_ref(&self) -> (&W, &U) {
//         (&self.first, &self.second)
//     }
//     pub fn get_mut(&mut self) -> (&mut W, &mut U) {
//         (&mut self.first, &mut self.second)
//     }
// }

// impl<W: Database, U: Database> Database for Chain<W, U> {
//     fn put<K, V>(&self, key: K, val: V) -> Result<()> {
//         self.second.put(key, val)?;
//         self.first.put(key, val)?;
//         Ok(())
//     }
//     fn get<K, V>(&self, key: K) -> Result<Option<&V>> {
//         if let Ok(some) = self.first.get(key) {
//             return Ok(some);
//         }
//         self.second.get(key)
//     }
//     fn remove<K, V>(&self, key: K) -> Result<Option<&V>> {
//         unimplemented!()
//     }

//     fn find_prefix<K, V>(&self, prefix: K) -> Result<Option<Vec<&V>>> {
//         unimplemented!()
//     }
//     fn remove_prefix<K, V>(&self, prefix: K) -> Result<Option<Vec<&V>>> {
//         unimplemented!()
//     }
//     fn find_prefix_with_filter<K, V>(
//         &self,
//         prefix: K,
//         filter: impl Fn(&V) -> FilterFlags,
//     ) -> Result<Option<Vec<&V>>> {
//         unimplemented!()
//     }
//     fn remove_prefix_with_filter<K, V>(
//         &self,
//         prefix: K,
//         filter: impl Fn(&V) -> FilterFlags,
//     ) -> Result<Option<Vec<&V>>> {
//         unimplemented!()
//     }

//     fn batch_put<K, V>(&self, batch: PutBatch<K, V>) -> Result<()> {
//         unimplemented!()
//     }
//     fn batch_get<K, V>(&self, batch: GetBatch<K>) -> Result<Option<Vec<&V>>> {
//         unimplemented!()
//     }
//     fn batch_remove<K, V>(&self, batch: RemoveBatch<K>) -> Result<Option<Vec<&V>>> {
//         unimplemented!()
//     }
// }
