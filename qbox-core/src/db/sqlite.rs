use super::{OrderStore, QboxStore};
use crate::broker::*;
use anyhow::Result;
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use rusqlite::{params, Connection, OpenFlags};
use std::path::Path;
use std::sync::Arc;

#[derive(Clone)]
pub struct SqliteStore {
    unit: String,
    inner: Arc<Connection>,
}

unsafe impl Send for SqliteStore {}
unsafe impl Sync for SqliteStore {}

impl SqliteStore {
    pub fn open<S: AsRef<str>>(unit: S) -> Result<SqliteStore> {
        let unit = unit.as_ref();
        let path = Path::new(&crate::data_path()).join(format!("{}.db", unit));
        static INSTANCE: OnceCell<SqliteStore> = OnceCell::new();
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
                    unit: unit.into(),
                    inner: conn,
                }
            })
            .clone();
        Ok(ret)
    }
}

impl OrderStore for SqliteStore {
    // type Bucket = Sqlite;
    // fn buckets(&self) -> Vec<String> {
    //     let vec: Vec<String> = self.buckets.iter().map(|v| v.clone()).collect();
    //     vec
    // }
    // fn bucket<S: Into<String>>(&self, name: S) -> Result<Sqlite> {
    //     let name = name.into();
    //     if self.buckets.contains(&name) {
    //         Ok(Sqlite {
    //             bucket_name: name.into(),
    //             buckets: self.buckets.clone(),
    //             inner: self.inner.clone(),
    //         })
    //     } else {
    //         const SQL: &str = "CREATE TABLE IF NOT EXISTS ? (
    //             key TEXT NOT NULL PRIMARY KEY,
    //             value BLOB,
    //             created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    //             updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
    //         );";
    //         self.inner.execute(SQL, params![&name])?;
    //         self.buckets.insert(name.clone());
    //         Ok(Sqlite {
    //             bucket_name: name,
    //             buckets: self.buckets.clone(),
    //             inner: self.inner.clone(),
    //         })
    //     }
    // }
    // fn drop_bucket<S: AsRef<str>>(&self, name: S) -> Result<()> {
    //     const SQL: &str = "DROP TABLE ?;";
    //     self.inner.execute(SQL, params![name.as_ref()])?;
    //     self.buckets.remove(name.as_ref());
    //     Ok(())
    // }
}

impl QboxStore for SqliteStore {
    // fn name(&self) -> &str {
    //     self.bucket_name.as_str()
    // }
    // fn put<K: Into<String>, V: AsRef<[u8]>>(&self, key: K, val: V) -> Result<()> {
    //     let key = key.into();
    //     let val = val.as_ref();
    //     const SQL_INSERT: &str = r#"INSERT INTO ? (key,value) VALUES (?,?);"#;
    //     const SQL_UPDATE: &str =
    //         r#"UPDATE ? SET value=?, updated_at=CURRENT_TIMESTAMP WHERE key=?;"#;
    //     let len = self
    //         .inner
    //         .as_ref()
    //         .execute(SQL_UPDATE, params![self.bucket_name, val, &key])?;
    //     if len == 0 {
    //         self.inner
    //             .as_ref()
    //             .execute(SQL_INSERT, params![self.bucket_name, &key, val])?;
    //     }
    //     Ok(())
    // }

    // fn get<K: AsRef<str>>(&self, key: K) -> Result<Option<Vec<u8>>> {
    //     const SQL: &str = "SELECT value FROM ? WHERE key=? LIMIT 1;";
    //     let mut stat = self.inner.as_ref().prepare(SQL)?;
    //     let mut list = stat.query_map(params![self.bucket_name, key.as_ref()], |row| {
    //         let val: Vec<u8> = row.get(0)?;
    //         Ok(val)
    //     })?;

    //     if let Some(val) = list.next() {
    //         let val = val?;
    //         return Ok(Some(val));
    //     }
    //     Ok(None)
    // }
    // fn remove<K: AsRef<str>>(&self, key: K) -> Result<()> {
    //     const SQL: &str = r#"DELETE FROM ? WHERE key=?;"#;
    //     self.inner
    //         .as_ref()
    //         .execute(SQL, params![self.bucket_name, key.as_ref()])?;
    //     Ok(())
    // }

    // fn find_prefix<K: AsRef<str>>(&self, prefix: K) -> Result<Option<Vec<Vec<u8>>>> {
    //     let prefix = format!("{}*", prefix.as_ref());
    //     const SQL: &str = "SELECT value FROM ? WHERE key GLOB ?;";
    //     let mut stat = self.inner.as_ref().prepare(SQL)?;
    //     let list = stat.query_map(params![self.bucket_name, &prefix], |row| {
    //         let val: Vec<u8> = row.get(0)?;
    //         Ok(val)
    //     })?;

    //     let mut ret = vec![];
    //     for instr in list {
    //         ret.push(instr?);
    //     }
    //     if ret.len() > 0 {
    //         Ok(Some(ret))
    //     } else {
    //         Ok(None)
    //     }
    // }
    // fn remove_prefix<K: AsRef<str>>(&self, prefix: K) -> Result<()> {
    //     let prefix = format!("{}*", prefix.as_ref());
    //     const SQL: &str = "DELETE FROM ? WHERE key GLOB ?;";
    //     self.inner
    //         .as_ref()
    //         .execute(SQL, params![self.bucket_name, &prefix])?;
    //     Ok(())
    // }
    // fn find_prefix_with_filter<K: AsRef<str>>(
    //     &self,
    //     prefix: K,
    //     filter: impl Fn(&[u8]) -> FilterFlags,
    // ) -> Result<Option<Vec<Vec<u8>>>> {
    //     let prefix = format!("{}*", prefix.as_ref());
    //     const SQL: &str = "SELECT value FROM ? WHERE key GLOB ?;";
    //     let mut stat = self.inner.as_ref().prepare(SQL)?;
    //     let list = stat.query_map(params![self.bucket_name, &prefix], |row| {
    //         let val: Vec<u8> = row.get(0)?;
    //         Ok(val)
    //     })?;

    //     let mut ret = vec![];
    //     for val in list {
    //         let val = val?;
    //         match filter(&val) {
    //             FilterFlags::Next => {
    //                 ret.push(val);
    //             }
    //             FilterFlags::Break => break,
    //             FilterFlags::Skip => {}
    //         }
    //     }
    //     if ret.len() > 0 {
    //         Ok(Some(ret))
    //     } else {
    //         Ok(None)
    //     }
    // }

    // fn batch_put<K: Into<String>, V: AsRef<[u8]>>(&self, batch: PutBatch<K, V>) -> Result<()> {
    //     unimplemented!()
    // }
    // fn batch_get<K: AsRef<str>>(&self, batch: GetBatch<K>) -> Result<Option<Vec<Vec<u8>>>> {
    //     unimplemented!()
    // }
    // fn batch_remove<K: AsRef<str>>(&self, batch: RemoveBatch<K>) -> Result<()> {
    //     unimplemented!()
    // }

    // fn count(&self) -> Result<usize> {
    //     const SQL: &str = "SELECT count(*) FROM ?;";
    //     let mut stat = self.inner.as_ref().prepare(SQL)?;
    //     let mut list = stat.query_map(params![self.bucket_name], |row| {
    //         let val: usize = row.get(0)?;
    //         Ok(val)
    //     })?;
    //     Ok(list.next().unwrap()?)
    // }
}
// pub fn find_all_instruments(db: &Connection) -> Result<Vec<Instrument>> {
//     let mut ret = vec![];
//     const SQL:&str = "SELECT security_id,exchange,symbol,kind,base_currency,quote_currency,multiplier,state,items FROM instruments;";
//     {
//         let mut stat = db.prepare(SQL)?;
//         let list = stat.query_map([], |row| {
//             let items: String = row.get(8)?;
//             let exchange: String = row.get(1)?;
//             let kind: String = row.get(3)?;
//             let state: String = row.get(7)?;
//             let items: Parameter = if let Ok(items) = ron::from_str::<Parameter>(&items) {
//                 items
//             } else {
//                 Parameter::new()
//             };
//             Ok(Instrument {
//                 security_id: row.get(0)?,
//                 exchange: Exchange::from(&exchange),
//                 symbol: row.get(2)?,
//                 kind: TradeKind::from(kind.as_str()),
//                 base_currency: row.get(4)?,
//                 quote_currency: row.get(5)?,
//                 multiplier: row.get(6)?,
//                 state: InstState::from(state.as_str()),
//                 items,
//             })
//         })?;
//         for instr in list {
//             ret.push(instr?);
//         }
//     }
//     Ok(ret)
// }

// pub fn insert_instrument(db: &Connection, instr: &Instrument) -> Result<()> {
//     const SQL: &str = r#"INSERT OR REPLACE INTO instruments (security_id,exchange,symbol,kind,base_currency,quote_currency,multiplier,state,items,updated_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,CURRENT_TIMESTAMP);"#;
//     let exchange: &str = instr.exchange.into();
//     let kind: &str = instr.kind.into();
//     let state = format!("{:?}", instr.state);
//     let items = ron::to_string(&instr.items)?;
//     db.execute(
//         SQL,
//         params![
//             instr.security_id,
//             exchange,
//             instr.symbol,
//             kind,
//             instr.base_currency,
//             instr.quote_currency,
//             instr.multiplier,
//             state,
//             items
//         ],
//     )?;
//     Ok(())
// }
