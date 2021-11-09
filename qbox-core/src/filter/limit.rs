use super::Filter;
use crate::core::Event;
use crate::setting;
use anyhow::Result;
use dashmap::DashMap;
use lazy_static::lazy_static;
use leaky_bucket::RateLimiter;
use once_cell::sync::OnceCell;
use std::sync::Arc;

lazy_static! {
    static ref API_LEAKY_BUCKETS: DashMap<String, RateLimiter> = DashMap::new();
}

#[derive(Clone)]
pub(crate) struct ApiLimiter {
    on: bool,
}

impl ApiLimiter {
    fn on_event(&self, ev: Arc<Event>) {}
}

impl ApiLimiter {
    pub fn new() -> Self {
        static SELF: OnceCell<ApiLimiter> = OnceCell::new();
        SELF.get_or_init(|| {
            let on = setting::get_with_default::<bool>("QBOX_API_LIMITER", "true").unwrap();
            if on {
                log::debug!("api limiter filter");
                let ret = Self { on };
                let cret = ret.clone();
                let _ = crate::core::subscribe("topic", move |topic, ev| {
                    cret.on_event(ev);
                });
                ret
            } else {
                Self { on }
            }
        })
        .clone()
    }
}

impl Default for ApiLimiter {
    fn default() -> Self {
        Self::new()
    }
}

impl<TradeEvent> Filter<TradeEvent> for ApiLimiter {
    fn name(&self) -> &'static str {
        "ApiLimiter"
    }
    fn do_filter(&self, ev: &TradeEvent) -> Result<()> {
        Ok(())
    }
}
