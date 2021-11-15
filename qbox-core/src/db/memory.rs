use super::QuoteStore;
use crate::broker::{Bar, Level1, Level2, Period, TickToOffer, TickToTrade};
use ahash::RandomState;
use anyhow::Result;
use dashmap::DashMap;
use once_cell::sync::OnceCell;

const MAX_BAR_SIZE: usize = 1000;
const MAX_TTO_SIZE: usize = 100;
const MAX_TTT_SIZE: usize = 100;
#[derive(Clone)]
pub struct MemQuoteStore {
    unit: String,
    level1: DashMap<String, Level1, RandomState>,
    bars: DashMap<String, Vec<Bar>, RandomState>,
    depths: DashMap<String, Level2, RandomState>,
    ttos: DashMap<String, Vec<TickToOffer>, RandomState>,
    ttts: DashMap<String, Vec<TickToTrade>, RandomState>,
}

impl MemQuoteStore {
    pub fn open<S: Into<String>>(unit: S) -> Self {
        static INSTANCE: OnceCell<MemQuoteStore> = OnceCell::new();
        let ret = INSTANCE
            .get_or_init(|| Self {
                unit: unit.into(),
                level1: DashMap::with_hasher(RandomState::new()),
                bars: DashMap::with_hasher(RandomState::new()),
                depths: DashMap::with_hasher(RandomState::new()),
                ttos: DashMap::with_hasher(RandomState::new()),
                ttts: DashMap::with_hasher(RandomState::new()),
            })
            .clone();
        ret
    }
}

impl QuoteStore for MemQuoteStore {
    fn update_level1(&self, level1: Level1) -> Result<()> {
        self.level1.insert(level1.security_id.clone(), level1);
        Ok(())
    }
    fn query_one_level1(&self, security_id: &str) -> Result<Option<Level1>> {
        if let Some(level1) = self.level1.get(security_id) {
            return Ok(Some(level1.value().clone()));
        }
        Ok(None)
    }
    fn query_all_level1(&self) -> Result<Option<Vec<Level1>>> {
        let data: Vec<Level1> = self
            .level1
            .iter()
            .map(|item| item.value().clone())
            .collect();
        if data.len() > 0 {
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    fn query_level1_with_prefix(&self, prefix: &str) -> Result<Option<Vec<Level1>>> {
        let data: Vec<Level1> = self
            .level1
            .iter()
            .filter(|item| item.key().starts_with(prefix))
            .map(|item| item.value().clone())
            .collect();
        if data.len() > 0 {
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    fn query_level1_with_prefixs(&self, prefixs: &[&str]) -> Result<Option<Vec<Level1>>> {
        let mut data = vec![];
        for prefix in prefixs {
            match self.query_level1_with_prefix(prefix) {
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
    fn insert_bar(&self, bar: Bar) -> Result<()> {
        if let Some(mut bars) = self.bars.get_mut(&bar.security_id) {
            bars.value_mut().push(bar.clone());
            if bars.len() > MAX_BAR_SIZE {
                bars.remove(0);
            }
        } else {
            let mut bars = Vec::with_capacity(MAX_BAR_SIZE);
            bars.push(bar.clone());
            self.bars.insert(bar.security_id.clone(), bars);
        }

        Ok(())
    }
    fn query_bar(&self, security_id: &str, period: Period) -> Result<Option<Vec<Bar>>> {
        if let Some(bars) = self.bars.get(security_id) {
            Ok(Some(bars.value().clone()))
        } else {
            Ok(None)
        }
    }
    fn insert_tick2offer(&self, tto: TickToOffer) -> Result<()> {
        if let Some(mut ttos) = self.ttos.get_mut(&tto.security_id) {
            ttos.value_mut().push(tto.clone());
            if ttos.len() > MAX_TTO_SIZE {
                ttos.remove(0);
            }
        } else {
            let mut ttos = Vec::with_capacity(MAX_TTO_SIZE);
            ttos.push(tto.clone());
            self.ttos.insert(tto.security_id.clone(), ttos);
        }
        Ok(())
    }
    fn query_tick2offer(&self, security_id: &str) -> Result<Option<Vec<TickToOffer>>> {
        if let Some(ttos) = self.ttos.get(security_id) {
            Ok(Some(ttos.value().clone()))
        } else {
            Ok(None)
        }
    }
    fn insert_tick2trade(&self, ttt: TickToTrade) -> Result<()> {
        if let Some(mut ttts) = self.ttts.get_mut(&ttt.security_id) {
            ttts.value_mut().push(ttt.clone());
            if ttts.len() > MAX_TTT_SIZE {
                ttts.remove(0);
            }
        } else {
            let mut ttts = Vec::with_capacity(MAX_TTT_SIZE);
            ttts.push(ttt.clone());
            self.ttts.insert(ttt.security_id.clone(), ttts);
        }
        Ok(())
    }
    fn query_tick2trade(&self, security_id: &str) -> Result<Option<Vec<TickToTrade>>> {
        if let Some(ttts) = self.ttts.get(security_id) {
            Ok(Some(ttts.value().clone()))
        } else {
            Ok(None)
        }
    }
    fn update_depth(&self, level2: Level2) -> Result<()> {
        self.depths.insert(level2.security_id.clone(), level2);
        Ok(())
    }
    fn query_depth(&self, security_id: &str) -> Result<Option<Level2>> {
        if let Some(level2) = self.depths.get(security_id) {
            Ok(Some(level2.value().clone()))
        } else {
            Ok(None)
        }
    }
}
