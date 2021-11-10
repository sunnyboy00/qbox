use super::{Bucket, Database, FilterFlags, GetBatch, PutBatch, RemoveBatch};
use anyhow::Result;
use dashmap::DashSet;
use once_cell::sync::OnceCell;
use rusqlite::{params, Connection, OpenFlags};
use std::path::Path;
use std::sync::Arc;

#[derive(Clone)]
pub struct Sqlite {
    bucket_name: String,
    buckets: DashSet<String>,
    inner: Arc<Connection>,
}

unsafe impl Send for Sqlite {}
unsafe impl Sync for Sqlite {}

impl Sqlite {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Sqlite> {
        static INSTANCE: OnceCell<Sqlite> = OnceCell::new();
        let ret = INSTANCE
            .get_or_init(|| {
                let conn = Arc::new(
                    Connection::open_with_flags(
                        path,
                        OpenFlags::SQLITE_OPEN_CREATE
                            | OpenFlags::SQLITE_OPEN_READ_WRITE
                            | OpenFlags::SQLITE_OPEN_NO_MUTEX
                            | OpenFlags::SQLITE_OPEN_SHARED_CACHE
                            | OpenFlags::SQLITE_OPEN_URI,
                    )
                    .unwrap(),
                );
                Self {
                    bucket_name: "defbucket".into(),
                    buckets: tables(&conn).unwrap(),
                    inner: conn,
                }
            })
            .clone();
        Ok(ret)
    }
    pub fn open_memory() -> Result<Sqlite> {
        static INSTANCE: OnceCell<Sqlite> = OnceCell::new();
        let ret = INSTANCE
            .get_or_init(|| {
                let conn = Arc::new(
                    Connection::open_in_memory_with_flags(
                        OpenFlags::SQLITE_OPEN_READ_WRITE
                            | OpenFlags::SQLITE_OPEN_CREATE
                            | OpenFlags::SQLITE_OPEN_SHARED_CACHE
                            | OpenFlags::SQLITE_OPEN_NO_MUTEX
                            | OpenFlags::SQLITE_OPEN_URI,
                    )
                    .unwrap(),
                );
                Self {
                    bucket_name: "defbucket".into(),
                    buckets: DashSet::new(),
                    inner: conn,
                }
            })
            .clone();
        Ok(ret)
    }
}

impl Database for Sqlite {
    type Bucket = Sqlite;
    fn buckets(&self) -> Vec<String> {
        let vec: Vec<String> = self.buckets.iter().map(|v| v.clone()).collect();
        vec
    }
    fn bucket<S: Into<String>>(&self, name: S) -> Result<Sqlite> {
        let name = name.into();
        if self.buckets.contains(&name) {
            Ok(Sqlite {
                bucket_name: name.into(),
                buckets: self.buckets.clone(),
                inner: self.inner.clone(),
            })
        } else {
            const SQL: &str = "CREATE TABLE IF NOT EXISTS ? (
                key TEXT NOT NULL PRIMARY KEY,
                value BLOB,
                created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
            );";
            self.inner.execute(SQL, params![&name])?;
            self.buckets.insert(name.clone());
            Ok(Sqlite {
                bucket_name: name,
                buckets: self.buckets.clone(),
                inner: self.inner.clone(),
            })
        }
    }
    fn drop_bucket<S: AsRef<str>>(&self, name: S) -> Result<()> {
        const SQL: &str = "DROP TABLE ?;";
        self.inner.execute(SQL, params![name.as_ref()])?;
        self.buckets.remove(name.as_ref());
        Ok(())
    }
}

impl Bucket for Sqlite {
    fn name(&self) -> &str {
        self.bucket_name.as_str()
    }
    fn put<K: Into<String>, V: AsRef<[u8]>>(&self, key: K, val: V) -> Result<()> {
        let key = key.into();
        let val = val.as_ref();
        const SQL_INSERT: &str = r#"INSERT INTO ? (key,value) VALUES (?,?);"#;
        const SQL_UPDATE: &str =
            r#"UPDATE ? SET value=?, updated_at=CURRENT_TIMESTAMP WHERE key=?;"#;
        let len = self
            .inner
            .as_ref()
            .execute(SQL_UPDATE, params![self.bucket_name, val, &key])?;
        if len == 0 {
            self.inner
                .as_ref()
                .execute(SQL_INSERT, params![self.bucket_name, &key, val])?;
        }
        Ok(())
    }

    fn get<K: AsRef<str>>(&self, key: K) -> Result<Option<Vec<u8>>> {
        const SQL: &str = "SELECT value FROM ? WHERE key=? LIMIT 1;";
        let mut stat = self.inner.as_ref().prepare(SQL)?;
        let mut list = stat.query_map(params![self.bucket_name, key.as_ref()], |row| {
            let val: Vec<u8> = row.get(0)?;
            Ok(val)
        })?;

        if let Some(val) = list.next() {
            let val = val?;
            return Ok(Some(val));
        }
        Ok(None)
    }
    fn remove<K: AsRef<str>>(&self, key: K) -> Result<()> {
        const SQL: &str = r#"DELETE FROM ? WHERE key=?;"#;
        self.inner
            .as_ref()
            .execute(SQL, params![self.bucket_name, key.as_ref()])?;
        Ok(())
    }

    fn find_prefix<K: AsRef<str>>(&self, prefix: K) -> Result<Option<Vec<Vec<u8>>>> {
        let prefix = format!("{}*", prefix.as_ref());
        const SQL: &str = "SELECT value FROM ? WHERE key GLOB ?;";
        let mut stat = self.inner.as_ref().prepare(SQL)?;
        let list = stat.query_map(params![self.bucket_name, &prefix], |row| {
            let val: Vec<u8> = row.get(0)?;
            Ok(val)
        })?;

        let mut ret = vec![];
        for instr in list {
            ret.push(instr?);
        }
        if ret.len() > 0 {
            Ok(Some(ret))
        } else {
            Ok(None)
        }
    }
    fn remove_prefix<K: AsRef<str>>(&self, prefix: K) -> Result<()> {
        let prefix = format!("{}*", prefix.as_ref());
        const SQL: &str = "DELETE FROM ? WHERE key GLOB ?;";
        self.inner
            .as_ref()
            .execute(SQL, params![self.bucket_name, &prefix])?;
        Ok(())
    }
    fn find_prefix_with_filter<K: AsRef<str>>(
        &self,
        prefix: K,
        filter: impl Fn(&[u8]) -> FilterFlags,
    ) -> Result<Option<Vec<Vec<u8>>>> {
        let prefix = format!("{}*", prefix.as_ref());
        const SQL: &str = "SELECT value FROM ? WHERE key GLOB ?;";
        let mut stat = self.inner.as_ref().prepare(SQL)?;
        let list = stat.query_map(params![self.bucket_name, &prefix], |row| {
            let val: Vec<u8> = row.get(0)?;
            Ok(val)
        })?;

        let mut ret = vec![];
        for val in list {
            let val = val?;
            match filter(&val) {
                FilterFlags::Next => {
                    ret.push(val);
                }
                FilterFlags::Break => break,
                FilterFlags::Skip => {}
            }
        }
        if ret.len() > 0 {
            Ok(Some(ret))
        } else {
            Ok(None)
        }
    }

    fn batch_put<K: Into<String>, V: AsRef<[u8]>>(&self, batch: PutBatch<K, V>) -> Result<()> {
        unimplemented!()
    }
    fn batch_get<K: AsRef<str>>(&self, batch: GetBatch<K>) -> Result<Option<Vec<Vec<u8>>>> {
        unimplemented!()
    }
    fn batch_remove<K: AsRef<str>>(&self, batch: RemoveBatch<K>) -> Result<()> {
        unimplemented!()
    }

    fn count(&self) -> Result<usize> {
        const SQL: &str = "SELECT count(*) FROM ?;";
        let mut stat = self.inner.as_ref().prepare(SQL)?;
        let mut list = stat.query_map(params![self.bucket_name], |row| {
            let val: usize = row.get(0)?;
            Ok(val)
        })?;
        Ok(list.next().unwrap()?)
    }
}

fn tables<C: AsRef<Connection>>(db: C) -> Result<DashSet<String>> {
    let ret = DashSet::new();
    const SQL: &str =
        "SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%';";
    {
        let mut stat = db.as_ref().prepare(SQL)?;
        let list = stat.query_map([], |row| {
            let name: String = row.get(0)?;
            Ok(name)
        })?;
        for instr in list {
            ret.insert(instr?);
        }
    }
    Ok(ret)
}
