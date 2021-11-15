use super::{OrderStore, QboxStore};
use crate::broker::*;
use ahash::RandomState;
use anyhow::Result;
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use rusqlite::{params, Connection, OpenFlags, Params};
use std::path::Path;
use std::sync::Arc;

#[derive(Clone)]
pub struct SqliteStore {
    unit: String,
    inner: Arc<Connection>,
    symbols: DashMap<String, Instrument, RandomState>,
    orders: DashMap<String, Vec<Order>, RandomState>,
    transactions: DashMap<String, DashMap<u64, Vec<Transaction>, RandomState>, RandomState>,
    positions: DashMap<String, Vec<Position>, RandomState>,
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
                let db = Self {
                    unit: unit.into(),
                    inner: conn,
                    symbols: DashMap::with_hasher(RandomState::new()),
                    orders: DashMap::with_hasher(RandomState::new()),
                    transactions: DashMap::with_hasher(RandomState::new()),
                    positions: DashMap::with_hasher(RandomState::new()),
                };
                if let Ok(Some(list)) = db.query_all_symbol() {
                    for itr in list {
                        db.symbols.insert(itr.security_id.clone(), itr);
                    }
                }
                db
            })
            .clone();
        Ok(ret)
    }
}

impl QboxStore for SqliteStore {
    fn set(&self, k: &str, v: &str) -> Result<()> {
        const SQL: &str = r#"INSERT OR REPLACE INTO qbox (unit,key,value) VALUES (?1,?2,?3);"#;
        self.inner.execute(SQL, params![self.unit, k, v])?;
        Ok(())
    }

    fn remove(&self, k: &str) -> Result<()> {
        const SQL: &str = r#"DELETE FROM qbox WHERE unit=? and key=?;"#;
        self.inner.execute(SQL, params![self.unit, k])?;
        Ok(())
    }

    fn get(&self, k: &str) -> Result<Option<String>> {
        const SQL: &str = "SELECT value FROM qbox WHERE unit=? and key=? LIMIT=1;";
        let mut stat = self.inner.prepare(SQL)?;
        let mut list = stat.query_map(params![self.unit, k], |row| {
            let val: String = row.get(0)?;
            Ok(val)
        })?;

        if let Some(val) = list.next() {
            let val = val?;
            return Ok(Some(val));
        }
        Ok(None)
    }

    fn get_all(&self) -> Result<Option<Vec<(String, String)>>> {
        let mut ret = vec![];
        const SQL: &str = "SELECT key,value FROM qbox WHERE unit=?;";
        let mut stat = self.inner.prepare(SQL)?;
        let list = stat.query_map(params![self.unit], |row| {
            let key: String = row.get(0)?;
            let val: String = row.get(1)?;
            Ok((key, val))
        })?;
        for val in list {
            ret.push(val?);
        }
        if ret.len() > 0 {
            Ok(Some(ret))
        } else {
            Ok(None)
        }
    }

    fn get_prefix(&self, prefix: &str) -> Result<Option<Vec<(String, String)>>> {
        let mut ret = vec![];
        const SQL: &str = "SELECT key,value FROM qbox WHERE unit=? AND key GLOB ?;";
        let mut stat = self.inner.prepare(SQL)?;
        let list = stat.query_map(params![self.unit, format!("{}*", prefix)], |row| {
            let key: String = row.get(0)?;
            let val: String = row.get(1)?;
            Ok((key, val))
        })?;
        for val in list {
            ret.push(val?);
        }
        if ret.len() > 0 {
            Ok(Some(ret))
        } else {
            Ok(None)
        }
    }

    fn get_prefixs(&self, prefixs: &[&str]) -> Result<Option<Vec<(String, String)>>> {
        let mut data = vec![];
        for prefix in prefixs {
            match self.get_prefix(prefix) {
                Ok(Some(val)) => data.push(val),
                Err(err) => return Err(anyhow::anyhow!("{}", err)),
                _ => {}
            }
        }
        if data.len() > 0 {
            let data = data.concat();
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    fn update_symbol(&self, symbol: Instrument) -> Result<()> {
        self.symbols
            .insert(symbol.security_id.clone(), symbol.clone());
        const SQL: &str = r#"INSERT OR REPLACE INTO symbols (security_id,exchange,symbol,kind,base_currency,quote_currency,multiplier,state,items,updated_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,CURRENT_TIMESTAMP);"#;
        let exchange: &str = symbol.exchange.into();
        let kind: &str = symbol.kind.into();
        let state = format!("{:?}", symbol.state);
        let items = ron::to_string(&symbol.items)?;
        self.inner.execute(
            SQL,
            params![
                symbol.security_id,
                exchange,
                symbol.symbol,
                kind,
                symbol.base_currency,
                symbol.quote_currency,
                symbol.multiplier,
                state,
                items
            ],
        )?;
        Ok(())
    }

    fn query_one_symbol(&self, security_id: &str) -> Result<Option<Instrument>> {
        if let Some(symbol) = self.symbols.get(security_id) {
            return Ok(Some(symbol.value().clone()));
        }
        const SQL:&str = "SELECT security_id,exchange,symbol,kind,base_currency,quote_currency,multiplier,state,items FROM symbols WHERE security_id = ?;";
        if let Some(list) = select_symbols(&self.inner, SQL, params![security_id])? {
            if let Some(one) = list.first() {
                return Ok(Some(one.clone()));
            }
        }
        Ok(None)
    }

    fn query_symbol_with_prefix(&self, prefix: &str) -> Result<Option<Vec<Instrument>>> {
        const SQL:&str = "SELECT security_id,exchange,symbol,kind,base_currency,quote_currency,multiplier,state,items FROM symbols WHERE security_id GLOB ?;";
        select_symbols(&self.inner, SQL, params![format!("{}*", prefix)])
    }

    fn query_symbol_with_prefixs(&self, prefixs: &[&str]) -> Result<Option<Vec<Instrument>>> {
        let mut data = vec![];
        for prefix in prefixs {
            match self.query_symbol_with_prefix(prefix) {
                Ok(Some(val)) => data.push(val),
                Err(err) => return Err(anyhow::anyhow!("{}", err)),
                _ => {}
            }
        }
        if data.len() > 0 {
            let data = data.concat();
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    fn query_all_symbol(&self) -> Result<Option<Vec<Instrument>>> {
        const SQL:&str = "SELECT security_id,exchange,symbol,kind,base_currency,quote_currency,multiplier,state,items FROM symbols;";
        select_symbols(&self.inner, SQL, [])
    }
}

impl OrderStore for SqliteStore {
    fn insert_order(&self, order: Order) -> Result<()> {
        self.update_order(order)
    }
    fn update_order(&self, order: Order) -> Result<()> {
        if let Some(mut orders) = self.orders.get_mut(order.security_id()) {
            orders.value_mut().push(order.clone());
        } else {
            let mut orders = vec![];
            orders.push(order.clone());
            self.orders.insert(order.security_id().into(), orders);
        }
        Ok(())
    }
    fn remove_order(&self, order_id: u64) -> Result<()> {
        unimplemented!()
    }
    fn query_one_order(&self, order_id: u64) -> Result<Option<Order>> {
        unimplemented!()
    }
    fn query_order(&self, security_id: &str) -> Result<Option<Vec<Order>>> {
        if let Some(orders) = self.orders.get(security_id) {
            Ok(Some(orders.value().clone()))
        } else {
            Ok(None)
        }
    }
    fn query_all_order(&self) -> Result<Option<Vec<Order>>> {
        let data: Vec<Vec<Order>> = self.orders.iter().map(|v| v.value().clone()).collect();
        if data.len() > 0 {
            Ok(Some(data.concat()))
        } else {
            Ok(None)
        }
    }

    fn insert_tx(&self, tx: Transaction) -> Result<()> {
        unimplemented!()
    }
    fn update_tx(&self, tx: Transaction) -> Result<()> {
        unimplemented!()
    }
    fn remove_tx(&self, txid: u64) -> Result<()> {
        unimplemented!()
    }
    fn query_one_tx(&self, txid: u64) -> Result<Option<Transaction>> {
        unimplemented!()
    }
    fn query_tx_with_order(&self, order_id: u64) -> Result<Option<Vec<Transaction>>> {
        unimplemented!()
    }
    fn query_tx_with_symbol(&self, security_id: &str) -> Result<Option<Vec<Transaction>>> {
        if let Some(positions) = self.transactions.get(security_id) {
            let data: Vec<Vec<Transaction>> = positions
                .value()
                .iter()
                .map(|item| item.value().clone())
                .collect();
            Ok(Some(data.concat()))
        } else {
            Ok(None)
        }
    }
    fn query_tx_with_symbol_and_order(
        &self,
        security_id: &str,
        order_id: u64,
    ) -> Result<Option<Vec<Transaction>>> {
        if let Some(trans) = self.transactions.get(security_id) {
            if let Some(txs) = trans.value().get(&order_id) {
                Ok(Some(txs.clone()))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    fn query_all_tx(&self) -> Result<Option<Vec<Transaction>>> {
        let data: Vec<Vec<Vec<Transaction>>> = self
            .transactions
            .iter()
            .map(|v| v.value().iter().map(|item| item.value().clone()).collect())
            .collect();
        if data.len() > 0 {
            Ok(Some(data.concat().concat()))
        } else {
            Ok(None)
        }
    }

    fn update_position(&self, position: Position) -> Result<()> {
        if let Some(mut positions) = self.positions.get_mut(&position.security_id) {
            positions.value_mut().push(position.clone());
        } else {
            let mut positions = vec![];
            positions.push(position.clone());
            self.positions
                .insert(position.security_id.clone(), positions);
        }
        Ok(())
    }

    fn remove_position(&self, security_id: &str) -> Result<()> {
        self.positions.remove(security_id);
        Ok(())
    }
    fn query_position(&self, security_id: &str) -> Result<Option<Vec<Position>>> {
        if let Some(positions) = self.positions.get(security_id) {
            Ok(Some(positions.value().clone()))
        } else {
            Ok(None)
        }
    }
    fn query_all_position(&self) -> Result<Option<Vec<Position>>> {
        let data: Vec<Vec<Position>> = self
            .positions
            .iter()
            .map(|item| item.value().clone())
            .collect();
        if data.len() > 0 {
            Ok(Some(data.concat()))
        } else {
            Ok(None)
        }
    }
}

pub fn select_symbols<P: Params>(
    db: &Connection,
    sql: &str,
    params: P,
) -> Result<Option<Vec<Instrument>>> {
    let mut ret = vec![];

    let mut stat = db.prepare(sql)?;
    let list = stat.query_map(params, |row| {
        let items: String = row.get(8)?;
        let exchange: String = row.get(1)?;
        let kind: String = row.get(3)?;
        let state: String = row.get(7)?;
        let items: Parameter = if let Ok(items) = ron::from_str::<Parameter>(&items) {
            items
        } else {
            Parameter::new()
        };
        Ok(Instrument {
            security_id: row.get(0)?,
            exchange: Exchange::from(&exchange),
            symbol: row.get(2)?,
            kind: TradeKind::from(kind.as_str()),
            base_currency: row.get(4)?,
            quote_currency: row.get(5)?,
            multiplier: row.get(6)?,
            state: InstState::from(state.as_str()),
            items,
        })
    })?;
    for instr in list {
        ret.push(instr?);
    }

    if ret.len() > 0 {
        Ok(Some(ret))
    } else {
        Ok(None)
    }
}
